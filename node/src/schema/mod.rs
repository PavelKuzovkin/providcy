/// Persistent data.
pub mod schema {
    use exonum::{
        crypto::PublicKey,
        merkledb::{
            access::{Access, FromAccess},
            MapIndex,
        },
    };
    use exonum_derive::{BinaryValue, FromAccess, ObjectHash};
    use exonum_proto::ProtobufConvert;

    use crate::proto;

    // Declare the data to be stored in the blockchain, namely wallets with balances.
    // See [serialization docs][1] for details.
    //
    // [1]: https://exonum.com/doc/version/latest/architecture/serialization

    /// Wallet struct used to persist data within the service.
    #[derive(Clone, Debug)]
    #[derive(Serialize, Deserialize)]
    #[derive(ProtobufConvert, BinaryValue, ObjectHash)]
    #[protobuf_convert(source = "proto::Wallet")]
    pub struct Wallet {
        /// Public key of the wallet owner.
        pub pub_key: PublicKey,
        /// Name of the wallet owner.
        pub name: String,
        /// Current balance.
        pub balance: u64,
    }

    /// Additional methods for managing balance of the wallet in an immutable fashion.
    impl Wallet {
        /// Create new Wallet.
        pub fn new(&pub_key: &PublicKey, name: &str, balance: u64) -> Self {
            Self {
                pub_key,
                name: name.to_owned(),
                balance,
            }
        }

        /// Returns a copy of this wallet with the balance increased by the specified amount.
        pub fn increase(self, amount: u64) -> Self {
            let balance = self.balance + amount;
            Self::new(&self.pub_key, &self.name, balance)
        }

        /// Returns a copy of this wallet with the balance decreased by the specified amount.
        pub fn decrease(self, amount: u64) -> Self {
            debug_assert!(self.balance >= amount);
            let balance = self.balance - amount;
            Self::new(&self.pub_key, &self.name, balance)
        }
    }

    /// Schema of the key-value storage used by the demo cryptocurrency service.
    #[derive(Debug, FromAccess)]
    pub struct CurrencySchema<T: Access> {
        /// Correspondence of public keys of users to the account information.
        pub wallets: MapIndex<T::Base, PublicKey, Wallet>,
    }

    impl<T: Access> CurrencySchema<T> {
        /// Creates a new schema.
        pub fn new(access: T) -> Self {
            Self::from_root(access).unwrap()
        }
    }
}