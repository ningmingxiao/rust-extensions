// This file is generated by ttrpc-compiler 0.4.0. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clipto_camel_casepy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]
use protobuf::{CodedInputStream, CodedOutputStream, Message};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub struct EventsClient {
    client: ::ttrpc::Client,
}

impl EventsClient {
    pub fn new(client: ::ttrpc::Client) -> Self {
        EventsClient {
            client: client,
        }
    }

    pub fn forward(&self, ctx: ttrpc::context::Context, req: &super::events::ForwardRequest) -> ::ttrpc::Result<super::empty::Empty> {
        let mut cres = super::empty::Empty::new();
        ::ttrpc::client_request!(self, ctx, req, "containerd.services.events.ttrpc.v1.Events", "Forward", cres);
        Ok(cres)
    }
}

struct ForwardMethod {
    service: Arc<std::boxed::Box<dyn Events + Send + Sync>>,
}

impl ::ttrpc::MethodHandler for ForwardMethod {
    fn handler(&self, ctx: ::ttrpc::TtrpcContext, req: ::ttrpc::Request) -> ::ttrpc::Result<()> {
        ::ttrpc::request_handler!(self, ctx, req, events, ForwardRequest, forward);
        Ok(())
    }
}

pub trait Events {
    fn forward(&self, _ctx: &::ttrpc::TtrpcContext, _req: super::events::ForwardRequest) -> ::ttrpc::Result<super::empty::Empty> {
        Err(::ttrpc::Error::RpcStatus(::ttrpc::get_status(::ttrpc::Code::NOT_FOUND, "/containerd.services.events.ttrpc.v1.Events/Forward is not supported".to_string())))
    }
}

pub fn create_events(service: Arc<std::boxed::Box<dyn Events + Send + Sync>>) -> HashMap <String, Box<dyn ::ttrpc::MethodHandler + Send + Sync>> {
    let mut methods = HashMap::new();

    methods.insert("/containerd.services.events.ttrpc.v1.Events/Forward".to_string(),
                    std::boxed::Box::new(ForwardMethod{service: service.clone()}) as std::boxed::Box<dyn ::ttrpc::MethodHandler + Send + Sync>);

    methods
}
