//! DomRf implementation example using [exonum](http://exonum.com/).

#![deny(unsafe_code, bare_trait_objects)]
#![warn(missing_docs, missing_debug_implementations)]

#[macro_use]
extern crate serde_derive; // Required for Protobuf.

pub use crate::{schema::Schema, transactions::DomRfServiceInterface};

pub mod proto;
pub mod schema;
pub mod transactions;
pub mod borrower;
pub mod loan_request;
pub mod insurance;
pub mod loan_order;
pub mod utils;
pub mod api;

use exonum::runtime::{ExecutionContext, ExecutionError, InstanceId};
use exonum_derive::{ServiceDispatcher, ServiceFactory};
use exonum_rust_runtime::{api::ServiceApiBuilder, DefaultInstance, Service};

use crate::{api::PublicApi as DomRfApi, schema::SchemaImpl};

/// DomRf service implementation.
#[derive(Debug, ServiceDispatcher, ServiceFactory)]
#[service_dispatcher(implements("DomRfServiceInterface"))]
#[service_factory(artifact_name = "exonum-domrf", proto_sources = "proto")]
pub struct DomRfService;

impl Service for DomRfService {
    fn initialize(
        &self,
        context: ExecutionContext<'_>,
        _params: Vec<u8>,
    ) -> Result<(), ExecutionError> {
        // Initialize indexes. Not doing this may lead to errors in HTTP API, since it relies on
        // `wallets` indexes being initialized for returning corresponding proofs.
        SchemaImpl::new(context.service_data());
        Ok(())
    }

    fn wire_api(&self, builder: &mut ServiceApiBuilder) {
        DomRfApi::wire(builder);
    }
}

/// Use predefined instance name and id for frontend.
impl DefaultInstance for DomRfService {
    const INSTANCE_ID: InstanceId = 3;
    const INSTANCE_NAME: &'static str = "crypto";
}
