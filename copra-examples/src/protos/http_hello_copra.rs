// This file is generated, Do not edit
// @generated

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub trait HelloService {
    type HelloGeneralFuture: ::futures::Future<
        Item = (super::http_hello::HelloResponse, ::copra::controller::Controller), 
        Error = ::copra::service::MethodError,
    > + 'static;

    type HelloToFuture: ::futures::Future<
        Item = (super::http_hello::HelloResponse, ::copra::controller::Controller), 
        Error = ::copra::service::MethodError,
    > + 'static;

    fn hello_general(&self, msg: (super::http_hello::HelloRequest, ::copra::controller::Controller)) -> Self::HelloGeneralFuture;

    fn hello_to(&self, msg: (super::http_hello::HelloRequest, ::copra::controller::Controller)) -> Self::HelloToFuture;
}

pub struct HelloRegistrant<S> {
    provider: S,
}

impl<S> HelloRegistrant<S> {
    pub fn new(provider: S) -> Self {
        HelloRegistrant { provider }
    }
}

impl<S> ::copra::dispatcher::Registrant for HelloRegistrant<S>
where
    S: HelloService + Clone + Send + Sync + 'static,
{
    fn methods(&self) -> Vec<(String, ::copra::service::NewEncapService)> {
        let mut entries = Vec::new();
        let provider = &self.provider;
    
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: HelloService + Clone,
            {
                type Request = (super::http_hello::HelloRequest, ::copra::controller::Controller);
                type Response = (super::http_hello::HelloResponse, ::copra::controller::Controller);
                type Error = ::copra::service::MethodError;
                type Future = <S as HelloService>::HelloGeneralFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.hello_general(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::EncapsulatedMethod::new(
                ::copra::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::copra::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "hello_general".to_string(), 
                Box::new(new_method) as ::copra::service::NewEncapService,
            ));
        }
        
        {
            #[derive(Clone)]
            struct Wrapper<S: Clone>(S);

            impl<S> ::copra::service::Service for Wrapper<S>
            where
                S: HelloService + Clone,
            {
                type Request = (super::http_hello::HelloRequest, ::copra::controller::Controller);
                type Response = (super::http_hello::HelloResponse, ::copra::controller::Controller);
                type Error = ::copra::service::MethodError;
                type Future = <S as HelloService>::HelloToFuture;

                fn call(&self, req: Self::Request) -> Self::Future {
                    self.0.hello_to(req)
                }
            }

            let wrap = Wrapper(provider.clone());
            let method = ::copra::service::EncapsulatedMethod::new(
                ::copra::codec::ProtobufCodec::new(), wrap
            );
            let new_method = ::copra::service::NewEncapsulatedMethod::new(method);
            entries.push((
                "hello_to".to_string(), 
                Box::new(new_method) as ::copra::service::NewEncapService,
            ));
        }
        
        entries
    }
}

impl<S> ::copra::dispatcher::NamedRegistrant for HelloRegistrant<S> 
where 
    S: HelloService + Clone + Send + Sync + 'static,
{
    fn name() -> &'static str {
        "Hello"
    }
}

#[derive(Clone)]
pub struct HelloStub<'a> {
    hello_general_wrapper: ::copra::stub::RpcWrapper<'a,
        ::copra::codec::ProtobufCodec<super::http_hello::HelloResponse, super::http_hello::HelloRequest>>,

    hello_to_wrapper: ::copra::stub::RpcWrapper<'a,
        ::copra::codec::ProtobufCodec<super::http_hello::HelloResponse, super::http_hello::HelloRequest>>,
}

impl<'a> HelloStub<'a> {
    pub fn new(channel: &'a ::copra::channel::Channel) -> Self {
        HelloStub {
            hello_general_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(), channel
            ),

            hello_to_wrapper: ::copra::stub::RpcWrapper::new(
                ::copra::codec::ProtobufCodec::new(), channel
            ),
        }
    }

    pub fn hello_general(
        &'a self, 
        msg: super::http_hello::HelloRequest,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::http_hello::HelloResponse,
            super::http_hello::HelloRequest,
        >,
    > {
        self.hello_general_wrapper
            .call((msg, "Hello".to_string(), "hello_general".to_string()))
    }

    pub fn hello_to(
        &'a self, 
        msg: super::http_hello::HelloRequest,
    ) -> ::copra::stub::StubFuture<
        ::copra::codec::ProtobufCodec<
            super::http_hello::HelloResponse,
            super::http_hello::HelloRequest,
        >,
    > {
        self.hello_to_wrapper
            .call((msg, "Hello".to_string(), "hello_to".to_string()))
    }
}
