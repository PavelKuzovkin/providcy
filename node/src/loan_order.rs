use exonum::{crypto::Hash, runtime::CallerAddress as Address};
use exonum_derive::{BinaryValue, ObjectHash};
use exonum_proto::ProtobufConvert;

use crate::proto;
use exonum::crypto::hash;
use crate::utils::Utils;

/// Borrower information stored in the database.
#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::LoanOrder", serde_pb_convert)]
pub struct LoanOrder {
    pub snils: String,
    pub bank: String,
    pub request_number: String,
    pub order_number: String,
    pub sum: u64,
    pub created_at: u64,
    pub expires_at: u64
}

impl LoanOrder {
    pub fn new(
        snils: &str,
        bank: &str,
        request_number: &str,
        order_number: &str,
        sum: u64,
        created_at: u64,
        expires_at: u64
    ) -> Self {
        Self {
            snils: snils.to_owned(),
            bank: bank.to_owned(),
            request_number: request_number.to_owned(),
            order_number: order_number.to_owned(),
            sum, created_at, expires_at
        }
    }

    pub fn hash(&self) -> Hash {
        Utils::hash_by_params(&self.bank, &self.order_number)
    }

    pub fn update(order: LoanOrder, sum: u64, expires_at: u64) -> Self {
        Self {
            snils: order.snils.to_string(),
            bank: order.bank.to_string(),
            request_number: order.request_number.to_string(),
            order_number: order.order_number.to_string(),
            sum,
            created_at: order.created_at,
            expires_at
        }
    }
}