// Copyright 2020 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! DomRF API.

use exonum::{
    blockchain::{BlockProof, IndexProof},
    crypto::{Hash, PublicKey},
    messages::{AnyTx, Verified},
    runtime::CallerAddress as Address,
};
use exonum_merkledb::{proof_map::Raw, ListProof, MapProof};
use exonum_rust_runtime::api::{self, ServiceApiBuilder, ServiceApiState};

use crate::schema::SchemaImpl;
use crate::loan_request::LoanRequest;
use crate::insurance::Insurance;
use crate::loan_order::LoanOrder;
use crate::utils::Utils;

/// Describes the query parameters for the `public key`.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct PubKeyQuery {
    /// Public key of the queried wallet.
    pub pub_key: PublicKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryByHashIdentifier {
    pub entity: String,
    pub item_number: String
}

impl QueryByHashIdentifier {
    pub fn hash(&self) -> Hash {
        Utils::hash_by_params(&self.entity, &self.item_number)
    }
}

// /// Proof of existence for specific wallet.
// #[derive(Debug, Serialize, Deserialize)]
// pub struct WalletProof {
//     /// Proof of the whole wallets table.
//     pub to_table: MapProof<String, Hash>,
//     /// Proof of the specific wallet in this table.
//     pub to_wallet: MapProof<Address, Wallet, Raw>,
// }
//
// /// Wallet history.
// #[derive(Debug, Serialize, Deserialize)]
// pub struct WalletHistory {
//     /// Proof of the list of transaction hashes.
//     pub proof: ListProof<Hash>,
//     /// List of above transactions.
//     pub transactions: Vec<Verified<AnyTx>>,
// }
//
// /// Wallet information.
// #[derive(Debug, Serialize, Deserialize)]
// pub struct WalletInfo {
//     /// Proof of the last block.
//     pub block_proof: BlockProof,
//     /// Proof of the appropriate wallet.
//     pub wallet_proof: WalletProof,
//     /// History of the appropriate wallet.
//     pub wallet_history: Option<WalletHistory>,
// }

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {

    pub async fn loan_requests_list(state: ServiceApiState, query: PubKeyQuery)
        -> api::Result<Vec<LoanRequest>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_requests;
        Ok(
            index.iter()
                .map(|(hash, loan_request)| loan_request)
                .collect::<Vec<LoanRequest>>()
        )
    }

    pub async fn loan_requests_by_params(state: ServiceApiState, query: QueryByHashIdentifier)
                                    -> api::Result<Option<LoanRequest>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_requests;
        Ok(index.get(&query.hash()))
    }

    pub async fn insurance_list(state: ServiceApiState, query: PubKeyQuery)
                                    -> api::Result<Vec<Insurance>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.insurances;
        Ok(index.iter()
            .map(|(hash, insurance)| insurance)
            .collect::<Vec<Insurance>>())
    }

    pub async fn insurance_by_params(state: ServiceApiState, query: QueryByHashIdentifier)
                                         -> api::Result<Option<Insurance>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.insurances;
        Ok(index.get(&query.hash()))
    }

    pub async fn loan_orders_list(state: ServiceApiState, query: PubKeyQuery)
                                    -> api::Result<Vec<LoanOrder>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_orders;
        Ok(
            index.iter()
                .map(|(hash, loan_order)| loan_order)
                .collect::<Vec<LoanOrder>>()
        )
    }

    pub async fn loan_order_by_params(state: ServiceApiState, query: QueryByHashIdentifier)
                                     -> api::Result<Option<LoanOrder>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_orders;
        Ok(index.get(&query.hash()))
    }


    // /// Endpoint for getting a single wallet.
    // pub async fn wallet_info(
    //     state: ServiceApiState,
    //     query: WalletQuery,
    // ) -> api::Result<WalletInfo> {
    //     let IndexProof {
    //         block_proof,
    //         index_proof,
    //         ..
    //     } = state.data().proof_for_service_index("wallets").unwrap();
    //
    //     let currency_schema = SchemaImpl::new(state.service_data());
    //     let address = Address::from_key(query.pub_key);
    //     let to_wallet = currency_schema.public.wallets.get_proof(address);
    //     let wallet_proof = WalletProof {
    //         to_table: index_proof,
    //         to_wallet,
    //     };
    //     let wallet = currency_schema.public.wallets.get(&address);
    //
    //     let wallet_history = wallet.map(|_| {
    //         // `history` is always present for existing wallets.
    //         let history = currency_schema.wallet_history.get(&address);
    //         let proof = history.get_range_proof(..);
    //
    //         let transactions = state.data().for_core().transactions();
    //         let transactions = history
    //             .iter()
    //             .map(|tx_hash| transactions.get(&tx_hash).unwrap())
    //             .collect();
    //
    //         WalletHistory {
    //             proof,
    //             transactions,
    //         }
    //     });
    //
    //     Ok(WalletInfo {
    //         block_proof,
    //         wallet_proof,
    //         wallet_history,
    //     })
    // }

    /// Wires the above endpoint to public scope of the given `ServiceApiBuilder`.
    pub fn wire(builder: &mut ServiceApiBuilder) {
        builder
            .public_scope()
            .endpoint("v1/loan_request/list", Self::loan_requests_list)
            .endpoint("v1/loan_request/by_params", Self::loan_requests_by_params)
            .endpoint("v1/insurance/list", Self::insurance_list)
            .endpoint("v1/insurance/by_params", Self::insurance_by_params)
            .endpoint("v1/loan_order/list", Self::loan_orders_list)
            .endpoint("v1/loan_order/by_params", Self::loan_order_by_params);
    }
}
