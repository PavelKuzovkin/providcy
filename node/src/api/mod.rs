/// Cryptocurrency API implementation.
pub mod api {
    use exonum::crypto::PublicKey;
    use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};

    use crate::schema::{CurrencySchema, Wallet};
    use crate::schema::schema::{CurrencySchema, Wallet};

    /// Public service API description.
    #[derive(Debug, Clone, Copy)]
    pub struct CryptocurrencyApi;

    /// The structure describes the query parameters for the `get_wallet` endpoint.
    #[derive(Debug, Serialize, Deserialize, Clone, Copy)]
    pub struct WalletQuery {
        /// Public key of the queried wallet.
        pub pub_key: PublicKey,
    }

    impl CryptocurrencyApi {
        /// Endpoint for getting a single wallet.
        pub async fn get_wallet(state: ServiceApiState, query: WalletQuery) -> api::Result<Wallet> {
            let schema = CurrencySchema::new(state.service_data());
            schema
                .wallets
                .get(&query.pub_key)
                .ok_or_else(|| api::Error::not_found().title("Wallet not found"))
        }

        /// Endpoint for dumping all wallets from the storage.
        pub async fn get_wallets(state: ServiceApiState, _query: ()) -> api::Result<Vec<Wallet>> {
            let schema = CurrencySchema::new(state.service_data());
            Ok(schema.wallets.values().collect())
        }

        /// `ServiceApiBuilder` facilitates conversion between read requests and REST
        /// endpoints.
        pub fn wire(builder: &mut ServiceApiBuilder) {
            // Binds handlers to specific routes.
            builder
                .public_scope()
                .endpoint("v1/wallet", Self::get_wallet)
                .endpoint("v1/wallets", Self::get_wallets);
        }
    }
}