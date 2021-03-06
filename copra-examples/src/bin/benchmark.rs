extern crate copra;
extern crate copra_examples;
extern crate env_logger;
#[macro_use]
extern crate futures;
extern crate tokio_core;

use copra::{ChannelBuilder, Controller, MethodError, ServerBuilder, ServiceRegistry};
use copra::codec::ProtobufCodec;
use copra::channel::Channel;
use copra::stub::StubFuture;
use copra::protocol::http::HttpStatus;
use copra_examples::protos::benchmark::{Empty, PressureRequest, StringMessage};
use copra_examples::protos::benchmark_copra::{MetricRegistrant, MetricService, PressureRegistrant,
                                              PressureService, PressureStub};
use futures::stream::futures_unordered::FuturesUnordered;
use futures::{task, Async, Poll, Stream};
use futures::future::{self, Future, FutureResult};
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio_core::reactor::Core;

#[derive(Clone)]
struct Pressure;

impl PressureService for Pressure {
    type EchoFuture = FutureResult<(StringMessage, Controller), MethodError>;

    type ProcessFuture = FutureResult<(Empty, Controller), MethodError>;

    fn echo(&self, msg: (StringMessage, Controller)) -> Self::EchoFuture {
        let (msg, controller) = msg;
        let string = msg.msg;
        let mut resp = StringMessage::new();
        resp.msg = string;
        future::ok((resp, controller))
    }

    fn process(&self, _msg: (PressureRequest, Controller)) -> Self::ProcessFuture {
        unimplemented!()
    }
}

#[derive(Clone)]
struct Metric {
    throughput: Arc<AtomicUsize>,
}

impl Metric {
    pub fn new(throughput: Arc<AtomicUsize>) -> Self {
        Metric { throughput }
    }
}

impl MetricService for Metric {
    type MetricFuture = FutureResult<(Empty, Controller), MethodError>;

    fn metric(&self, msg: (Empty, Controller)) -> Self::MetricFuture {
        let (empty, mut controller) = msg;
        let throughput = self.throughput.load(Ordering::Relaxed);
        let resp = format!("Throughput: {}", throughput);
        controller.status = Some(HttpStatus::Ok);
        controller.response_body = resp.into();
        controller.set_content_type("text/plain");

        future::ok((empty, controller))
    }
}

struct Sender {
    channel: Channel,
    in_flight: FuturesUnordered<StubFuture<ProtobufCodec<StringMessage, StringMessage>>>,
}

impl Sender {
    pub fn new(channel: Channel) -> Self {
        Sender {
            channel,
            in_flight: FuturesUnordered::new(),
        }
    }
}

impl Future for Sender {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        while !self.channel.congested() {
            let stub = PressureStub::new(&self.channel);
            let mut req = StringMessage::new();
            req.set_msg("ABCDE_ABCDE_ABCDE_ABCDE_ABCDE_ABCDE_ABCDE_ABCDE".to_string());
            let resp = stub.echo(req);
            self.in_flight.push(resp);
        }
        loop {
            if let None = try_ready!(self.in_flight.poll().map_err(|_| ())) {
                task::current().notify();
                return Ok(Async::NotReady);
            }
        }
    }
}

fn main() {
    env_logger::init().unwrap();

    let client_thread_num = 1;
    let addr = "127.0.0.1:8991";
    let mut core = Core::new().unwrap();
    let mut registry = ServiceRegistry::new();
    let throughtput = Arc::new(AtomicUsize::new(0));

    let registrant = PressureRegistrant::new(Pressure);
    registry.register_service(registrant);
    let registrant = MetricRegistrant::new(Metric::new(throughtput.clone()));
    registry.register_service(registrant);

    let server = ServerBuilder::new(addr, registry)
        .threads(1)
        .throughput(throughtput, core.remote())
        .build()
        .unwrap();

    thread::spawn(move || {
        server.start();
    });
    thread::sleep(Duration::from_millis(100));

    let _threads: Vec<_> = (0..client_thread_num)
        .map(|_| {
            thread::spawn(move || {
                let mut core = Core::new().unwrap();
                let handle = core.handle();
                let channel = core.run(
                    ChannelBuilder::single_server(addr, handle)
                        .max_concurrency(1000)
                        .build(),
                ).unwrap();

                let sender = Sender::new(channel);
                core.run(sender).unwrap();
            })
        })
        .collect();

    core.run(future::empty::<(), ()>()).unwrap();
}
