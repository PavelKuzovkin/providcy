use exonum::{crypto::Hash};
use exonum_derive::{BinaryValue, ObjectHash};
use exonum_proto::ProtobufConvert;

use crate::proto;
use exonum::crypto::hash;

/// Borrower information stored in the database.
#[derive(Clone, Debug, ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::Borrower", serde_pb_convert)]
pub struct Borrower {
    pub snils: String,           // СНИЛС заемщика
    pub fio: String,             // ФИО заемщика
    pub email: String,           // email заемщика
    pub phone: String,           // Телефон заемщика
}

impl Borrower {
    /// Creates a new borrower.
    pub fn new(
        snils: &str,
        fio: &str,
        email: &str,
        phone: &str
    ) -> Self {
        Self {
            snils: snils.to_owned(),
            fio: fio.to_owned(),
            email: email.to_owned(),
            phone: phone.to_owned()
        }
    }
    pub fn hash(&self) -> Hash {
        hash(self.snils.as_bytes())
    }
}