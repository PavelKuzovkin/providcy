use exonum::{crypto::Hash, runtime::CallerAddress as Address};
use exonum_derive::{BinaryValue, ObjectHash};
use exonum_proto::ProtobufConvert;

use crate::proto;
use exonum::crypto::hash;

/// Borrower information stored in the database.
#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::LoanRequest", serde_pb_convert)]
pub struct LoanRequest {
    pub snils: String,
    pub bank: String,            // Наименование банка
    pub request_number: String,  // Номер заявки на кредит
    pub sum: u64,                // Сумма кредита
    pub created_at: u64,         // Дата заявки
    pub status: u32              // статус заявки
}

impl LoanRequest {
    pub fn new(
        snils: &str,
        bank: &str,
        request_number: &str,
        sum: u64,
        created_at: u64,
        status: u32
    ) -> Self {
        Self {
            snils: snils.to_owned(),
            bank: bank.to_owned(),
            request_number: request_number.to_owned(),
            sum, created_at, status
        }
    }

    pub fn hash(&self) -> Hash {
        let mut hash_string = "".to_owned();
        hash_string.push_str(&self.bank);
        hash_string.push_str(&self.request_number);
        hash(hash_string.as_bytes())
    }
}