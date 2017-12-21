extern crate caper;
extern crate caper_examples;
extern crate env_logger;
extern crate futures;
extern crate futures_cpupool;
extern crate primal;
extern crate protobuf;
extern crate rand;
extern crate tokio_core;
extern crate tokio_service;
extern crate tokio_timer;

use caper::controller::Controller;
use caper::channel::{ChannelBuilder, ChannelOption};
use caper::dispatcher::ServiceRegistry;
use caper::service::MethodError;
use caper::server::ServerBuilder;
use caper_examples::protos::demo::{GreetMessage, PrimeRequest, PrimeResponse};
use caper_examples::protos::demo_caper::{DemoRegistrant, DemoService, DemoStub};
use futures::Future;
use futures::future::FutureResult;
use futures::future;
use futures_cpupool::CpuPool;
use rand::Rng;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use tokio_core::reactor::Core;
use tokio_timer::Timer;
use primal::is_prime;

#[derive(Clone)]
struct Demo {
    pool: Arc<CpuPool>,
}

impl Demo {
    pub fn new(pool: Arc<CpuPool>) -> Self {
        Demo { pool }
    }
}

impl DemoService for Demo {
    type GreetToFuture = FutureResult<(GreetMessage, Controller), MethodError>;

    type IsPrimeFuture = Box<Future<Item = (PrimeResponse, Controller), Error = MethodError>>;

    fn greet_to(&self, msg: (GreetMessage, Controller)) -> Self::GreetToFuture {
        let (msg, controller) = msg;
        let name = msg.msg;
        let mut resp = GreetMessage::new();
        resp.set_msg(format!("Greetings! {}.", name));
        future::ok((resp, controller))
    }

    fn is_prime(&self, msg: (PrimeRequest, Controller)) -> Self::IsPrimeFuture {
        let (msg, controller) = msg;
        let number = msg.get_number();
        let future = self.pool
            .spawn_fn(move || Ok(is_prime(number)))
            .map(move |re| {
                let mut resp = PrimeResponse::new();
                resp.set_is_prime(re);
                resp.set_number(number);
                (resp, controller)
            });
        Box::new(future)
    }
}

fn random_name() -> String {
    let mut gen = rand::thread_rng();
    let words = gen.gen_range(1, 6);

    (0..words)
        .map(|_| {
            let word_len = gen.gen_range(1, 10);
            gen.gen_ascii_chars()
                .filter(|c| c.is_lowercase())
                .take(word_len)
                .enumerate()
                .map(|(i, c)| {
                    if i == 0 {
                        c.to_uppercase().next().unwrap()
                    } else {
                        c
                    }
                })
                .collect::<String>()
        })
        .fold(String::new(), |name, word| name + " " + &word)
}

fn main() {
    env_logger::init().unwrap();

    //setup server
    let addr = "127.0.0.1:8992";
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let mut registry = ServiceRegistry::new();
    let pool = CpuPool::new(2);

    let registrant = DemoRegistrant::new(Demo::new(Arc::new(pool)));
    registry.register_service("Demo", registrant);

    let server = ServerBuilder::new(addr, registry).build();
    thread::spawn(move || {
        server.start();
    });
    thread::sleep(Duration::from_millis(100));

    //setup client
    let option = ChannelOption::new();
    let (channel, backend) = core.run(ChannelBuilder::single_server(addr, handle.clone(), option))
        .unwrap();
    handle.spawn(backend);

    //create a stub for DemoService
    let stub = DemoStub::new(&channel);

    let timer = Timer::default();
    //let mut gen = rand::thread_rng();

    loop {
        let mut hello_req = GreetMessage::new();
        hello_req.set_msg(random_name());
        let wait = timer.sleep(Duration::from_millis(1500)).map_err(|_| ());
        println!("Sent: {}", hello_req.get_msg());
        let fut = stub.greet_to(hello_req.clone())
            .map_err(|_| ())
            .map(|(msg, _)| {
                println!("Received: {}", msg.get_msg());
                println!("--------------------------------");
            });

        core.run(fut.join(wait)).unwrap();
    }
}
