use exonum::{crypto::Hash, runtime::CallerAddress as Address};
use exonum_derive::{BinaryValue, ObjectHash};
use exonum_proto::ProtobufConvert;

use crate::proto;
use exonum::crypto::hash;
use crate::utils::Utils;
use crate::transactions::TxUpdateInsurance;

/// Insurance information stored in the database.
#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Insurance", serde_pb_convert)]
pub struct Insurance {
    pub snils: String,
    pub bank: String,            // Наименование банка
    pub request_number: String,  // Номер заявки на кредит
    pub order_number: String,  // Номер заявки на кредит
    pub insurer: String,                // Сумма кредита
    pub policy_number: String,                // Сумма кредита
    pub sum: u64,
    pub created_at: u64,         // Дата заявки
    pub starts_at: u64,         // Дата заявки
    pub expires_at: u64,         // Дата заявки
}

impl Insurance {
    pub fn new(
        snils: &str,
        bank: &str,
        request_number: &str,
        order_number: &str,
        insurer: &str,
        policy_number: &str,
        sum: u64,
        created_at: u64,
        starts_at: u64,
        expires_at: u64
    ) -> Self {
        Self {
            snils: snils.to_owned(),
            bank: bank.to_owned(),
            request_number: request_number.to_owned(),
            order_number: order_number.to_owned(),
            insurer: insurer.to_owned(),
            policy_number: policy_number.to_owned(),
            sum, created_at, starts_at,expires_at
        }
    }

    pub fn hash(&self) -> Hash {
        Utils::hash_by_params(&self.insurer, &self.policy_number)
    }

    pub fn update(insurance: Insurance, order_number: &str) -> Self {
        Self {
            snils: insurance.snils.to_string(),
            bank: insurance.bank.to_string(),
            request_number: insurance.request_number.to_string(),
            order_number: order_number.to_string(),
            insurer: insurance.insurer.to_string(),
            policy_number: insurance.policy_number.to_string(),
            sum: insurance.sum,
            created_at: insurance.created_at,
            starts_at: insurance.starts_at,
            expires_at: insurance.expires_at
        }
    }
}