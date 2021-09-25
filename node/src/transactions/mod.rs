/// Transactions.
pub mod transactions {
    use exonum::crypto::PublicKey;
    use exonum_derive::{BinaryValue, ObjectHash};
    use exonum_proto::ProtobufConvert;

    use super::proto;

    /// Transaction type for creating a new wallet.
    ///
    /// See [the `Transaction` trait implementation](#impl-Transaction) for details how
    /// `TxCreateWallet` transactions are processed.
    #[derive(Clone, Debug)]
    #[derive(Serialize, Deserialize)]
    #[derive(ProtobufConvert, BinaryValue, ObjectHash)]
    #[protobuf_convert(source = "proto::TxCreateWallet")]
    pub struct CreateWallet {
        /// UTF-8 string with the owner's name.
        pub name: String,
    }

    impl CreateWallet {
        /// Creates a wallet with the specified name.
        pub fn new(name: impl Into<String>) -> Self {
            Self { name: name.into() }
        }
    }

    /// Transaction type for transferring tokens between two wallets.
    ///
    /// See [the `Transaction` trait implementation](#impl-Transaction) for details how
    /// `TxTransfer` transactions are processed.
    #[derive(Clone, Debug)]
    #[derive(Serialize, Deserialize)]
    #[derive(ProtobufConvert, BinaryValue, ObjectHash)]
    #[protobuf_convert(source = "proto::TxTransfer")]
    pub struct TxTransfer {
        /// Public key of the receiver.
        pub to: PublicKey,
        /// Number of tokens to transfer from sender's account to receiver's account.
        pub amount: u64,
        /// Auxiliary number to guarantee [non-idempotence][idempotence] of transactions.
        ///
        /// [idempotence]: https://en.wikipedia.org/wiki/Idempotence
        pub seed: u64,
    }
}