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
    crypto::{Hash, PublicKey, hash},
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
use crate::borrower::Borrower;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanRequestDto {
    pub loan_request: LoanRequest,
    pub borrower: Option<Borrower>,
    pub loan_order: Option<LoanOrder>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsuranceDto {
    pub insurance: Insurance,
    pub borrower: Option<Borrower>,
    pub loan_orders: Vec<LoanOrder>,
    pub loan_request: Option<LoanRequest>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoanOrderDto {
    pub loan_order: LoanOrder,
    pub borrower: Option<Borrower>,
    pub loan_request: Option<LoanRequest>,
    pub insurances: Vec<Insurance>
}

impl QueryByHashIdentifier {
    pub fn hash(&self) -> Hash {
        Utils::hash_by_params(&self.entity, &self.item_number)
    }
}

/// Public service API description.
#[derive(Debug, Clone, Copy)]
pub struct PublicApi;

impl PublicApi {

    pub async fn loan_requests_list(state: ServiceApiState, query: PubKeyQuery)
        -> api::Result<Vec<LoanRequestDto>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_requests;
        let index_borrower = schema.public.borrowers;
        let index_orders = schema.public.loan_orders;
        Ok(
            index.iter()
                .map(|(hash, loan_request)| loan_request)
                .map(|loan_request| {
                    let x: Vec<LoanOrder> = index_orders.iter().filter(|(hash, loan_order)|
                        loan_order.bank.eq(&loan_request.bank) && loan_order.request_number.eq(&loan_request.request_number)
                    ).map(|tuple| tuple.1).collect();

                    let req = loan_request.clone();
                    LoanRequestDto{
                        loan_request: req,
                        borrower: index_borrower.get(&hash(loan_request.snils.as_bytes())),
                        loan_order: x.into_iter().nth(0)
                    }
                })
                .collect::<Vec<LoanRequestDto>>()
        )
    }

    pub async fn loan_requests_by_params(state: ServiceApiState, query: QueryByHashIdentifier)
                                    -> api::Result<Option<LoanRequestDto>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_requests;
        let index_borrower = schema.public.borrowers;
        let index_order = schema.public.loan_orders;
        Ok(index.get(&query.hash()).map(|loan_request| {
            let x: Vec<LoanOrder> = index_order.iter().filter(|(hash, order)| {
                loan_request.request_number.eq(&order.request_number) && loan_request.bank.eq(&order.bank)
            }).map(|tuple| tuple.1).collect();
            let req = loan_request.clone();
            LoanRequestDto{
                loan_request: req,
                borrower: index_borrower.get(&hash(loan_request.snils.as_bytes())),
                loan_order: x.into_iter().nth(0)
            }
        }))
    }

    pub async fn insurance_list(state: ServiceApiState, query: PubKeyQuery)
                                    -> api::Result<Vec<InsuranceDto>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.insurances;
        let index_borrower = schema.public.borrowers;
        let index_order = schema.public.loan_orders;
        let index_request = schema.public.loan_requests;
        Ok(index.iter()
            .map(|(hash, insurance)| insurance)
            .map(|insurance| {
                let x: Vec<LoanOrder> = index_order
                    .iter()
                    .filter(|(hash, order)| {
                        order.request_number.eq(&insurance.request_number) && order.bank.eq(&insurance.bank)
                    })
                    .map(|tuple| tuple.1)
                    .collect();

                let y: Vec<LoanRequest> = index_request
                    .iter()
                    .filter(|(hash, loan_request)| {
                        loan_request.request_number.eq(&insurance.request_number) && loan_request.bank.eq(&insurance.bank)
                    })
                    .map(|tuple| tuple.1)
                    .collect();

                let ins = insurance.clone();
                InsuranceDto{
                    insurance: ins,
                    borrower: index_borrower.get(&hash(insurance.snils.as_bytes())),
                    loan_orders: x,
                    loan_request: y.into_iter().nth(0)
                }
            })
            .collect::<Vec<InsuranceDto>>())
    }

    pub async fn insurance_by_params(state: ServiceApiState, query: QueryByHashIdentifier)
                                         -> api::Result<Option<InsuranceDto>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.insurances;
        let index_borrower = schema.public.borrowers;
        let index_order = schema.public.loan_orders;
        let index_request = schema.public.loan_requests;
        Ok(index.get(&query.hash())
            .map(|insurance| {

                let x: Vec<LoanOrder> = index_order
                    .iter()
                    .filter(|(hash, order)| {
                        order.request_number.eq(&insurance.request_number) && order.bank.eq(&insurance.bank)
                    })
                    .map(|tuple| tuple.1)
                    .collect();

                let y: Vec<LoanRequest> = index_request
                    .iter()
                    .filter(|(hash, loan_request)| {
                        loan_request.request_number.eq(&insurance.request_number) && loan_request.bank.eq(&insurance.bank)
                    })
                    .map(|tuple| tuple.1)
                    .collect();

                let ins = insurance.clone();
                InsuranceDto{
                    insurance: ins,
                    borrower: index_borrower.get(&hash(insurance.snils.as_bytes())),
                    loan_orders: x,
                    loan_request: y.into_iter().nth(0)
                }
            }))
    }

    pub async fn loan_orders_list(state: ServiceApiState, query: PubKeyQuery)
                                    -> api::Result<Vec<LoanOrderDto>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_orders;
        let index_borrower = schema.public.borrowers;
        let index_insurance = schema.public.insurances;
        let index_request = schema.public.loan_requests;
        Ok(
            index.iter()
                .map(|(hash, loan_order)| loan_order)
                .map(|loan_order| {

                    let y: Vec<LoanRequest> = index_request
                        .iter()
                        .filter(|(hash, loan_request)| {
                            loan_request.request_number.eq(&loan_order.request_number) && loan_request.bank.eq(&loan_order.bank)
                        })
                        .map(|tuple| tuple.1)
                        .collect();

                    let x: Vec<Insurance> = index_insurance
                        .iter()
                        .filter(|(hash, insurance)| {
                            loan_order.request_number.eq(&insurance.request_number) && loan_order.bank.eq(&insurance.bank)
                        })
                        .map(|tuple| tuple.1)
                        .collect();

                    let lo = loan_order.clone();
                    LoanOrderDto{
                        loan_order: lo,
                        borrower: index_borrower.get(&hash(loan_order.snils.as_bytes())),
                        loan_request: y.into_iter().nth(0),
                        insurances: x
                    }
                })
                .collect::<Vec<LoanOrderDto>>()
        )
    }

    pub async fn loan_order_by_params(state: ServiceApiState, query: QueryByHashIdentifier)
                                     -> api::Result<Option<LoanOrderDto>> {
        let schema = SchemaImpl::new(state.service_data());
        let index = schema.public.loan_orders;
        let index_borrower = schema.public.borrowers;
        let index_insurance = schema.public.insurances;
        let index_request = schema.public.loan_requests;
        Ok(index.get(&query.hash())
            .map(|loan_order| {

                let y: Vec<LoanRequest> = index_request
                    .iter()
                    .filter(|(hash, loan_request)| {
                        loan_request.request_number.eq(&loan_order.request_number) && loan_request.bank.eq(&loan_order.bank)
                    })
                    .map(|tuple| tuple.1)
                    .collect();

                let x: Vec<Insurance> = index_insurance
                    .iter()
                    .filter(|(hash, insurance)| {
                        loan_order.request_number.eq(&insurance.request_number) && loan_order.bank.eq(&insurance.bank)
                    })
                    .map(|tuple| tuple.1)
                    .collect();

                let lo = loan_order.clone();
                LoanOrderDto{
                    loan_order: lo,
                    borrower: index_borrower.get(&hash(loan_order.snils.as_bytes())),
                    loan_request: y.into_iter().nth(0),
                    insurances: x
                }
            })
        )
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
