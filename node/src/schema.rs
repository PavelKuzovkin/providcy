use exonum::{
    crypto::Hash,
    merkledb::{
        access::{Access, FromAccess, RawAccessMut},
        Group, ObjectHash, ProofListIndex, RawProofMapIndex,
    },
    runtime::CallerAddress as Address,
};
use exonum_derive::{FromAccess, RequireArtifact};

// use crate::{wallet::Wallet, INITIAL_BALANCE};
use exonum_merkledb::MapIndex;
use crate::borrower::Borrower;
use crate::loan_request::LoanRequest;
use exonum::crypto::hash;
use std::time::Instant;
use crate::insurance::Insurance;
use crate::loan_order::LoanOrder;

/// Database schema for the cryptocurrency.
///
/// Note that the schema is crate-private, but it has a public part.
#[derive(Debug, FromAccess)]
pub(crate) struct SchemaImpl<T: Access> {
    /// Public part of the schema.
    #[from_access(flatten)]
    pub public: Schema<T>,
    // /// History for specific wallets.
    pub wallet_history: Group<T, Address, ProofListIndex<T::Base, Hash>>,
}


/// Public part of the cryptocurrency schema.
#[derive(Debug, FromAccess, RequireArtifact)]
#[require_artifact(name = "exonum-cryptocurrency")]
pub struct Schema<T: Access> {
    pub borrowers: MapIndex<T::Base, Hash, Borrower>,
    pub loan_requests: RawProofMapIndex<T::Base, Hash, LoanRequest>,
    pub insurances: RawProofMapIndex<T::Base, Hash, Insurance>,
    pub loan_orders: RawProofMapIndex<T::Base, Hash, LoanOrder>,
    // Map of wallet keys to information about the corresponding account.
    // pub wallets: RawProofMapIndex<T::Base, Address, Wallet>,
}

impl<T: Access> SchemaImpl<T> {
    pub fn new(access: T) -> Self {
        Self::from_root(access).unwrap()
    }
    pub fn borrower(&self, request_hash: Hash ) -> Option<Borrower> { self.public.borrowers.get(&request_hash) }
    pub fn loan_request(&self, request_hash: Hash ) -> Option<LoanRequest> { self.public.loan_requests.get(&request_hash) }
    pub fn insurance_for_loan_request(&self, loan_request: LoanRequest) -> Option<Insurance> {
        let insurances = self.public.insurances.iter().filter(|(hash, insurance)|
            insurance.request_number.eq(&loan_request.request_number) &&
            insurance.bank.eq(&loan_request.bank)
        ).map(|item| item.1).collect::<Vec<Insurance>>();
        insurances.into_iter().nth(0)
    }
    pub fn insurance(&self, request_hash: Hash ) -> Option<Insurance> { self.public.insurances.get(&request_hash) }
    pub fn loan_order(&self, request_hash: Hash ) -> Option<LoanOrder> { self.public.loan_orders.get(&request_hash) }
}

impl<T> SchemaImpl<T>
where
    T: Access,
    T::Base: RawAccessMut,
{
    // Increases balance of the wallet and append new record to its history.
    // pub fn increase_wallet_balance(&mut self, wallet: Wallet, amount: u64, transaction: Hash) {
    //     let mut history = self.wallet_history.get(&wallet.owner);
    //     history.push(transaction);
    //     let history_hash = history.object_hash();
    //     let balance = wallet.balance;
    //     let wallet = wallet.set_balance(balance + amount, &history_hash);
    //     let wallet_key = wallet.owner;
    //     self.public.wallets.put(&wallet_key, wallet);
    // }

    // Decreases balance of the wallet and append new record to its history.
    // pub fn decrease_wallet_balance(&mut self, wallet: Wallet, amount: u64, transaction: Hash) {
    //     let mut history = self.wallet_history.get(&wallet.owner);
    //     history.push(transaction);
    //     let history_hash = history.object_hash();
    //     let balance = wallet.balance;
    //     let wallet = wallet.set_balance(balance - amount, &history_hash);
    //     let wallet_key = wallet.owner;
    //     self.public.wallets.put(&wallet_key, wallet);
    // }

    // Creates a new wallet and append first record to its history.
    // pub fn create_wallet(&mut self, key: Address, name: &str, transaction: Hash) {
    //     let mut history = self.wallet_history.get(&key);
    //     history.push(transaction);
    //     let history_hash = history.object_hash();
    //     let wallet = Wallet::new(key, name, INITIAL_BALANCE, history.len(), &history_hash);
    //     self.public.wallets.put(&key, wallet);
    // }

    pub fn create_borrower(&mut self, borrower: Borrower) {
        self.public.borrowers.put(&borrower.hash(), borrower);
    }

    pub fn create_loan_request(&mut self, loan_request: LoanRequest) {
        self.public.loan_requests.put(&loan_request.hash(), loan_request);
    }

    pub fn update_loan_request_status(&mut self, loan_request_hash: Hash, loan_request: LoanRequest, status: u32) {
        let updated_loan_request = LoanRequest::new(
            &loan_request.snils,
            &loan_request.bank,
            &loan_request.request_number,
            loan_request.sum,
            loan_request.created_at,
            status
        );
        self.public.loan_requests.put(&loan_request_hash, updated_loan_request);
    }

    pub fn save_insurance(&mut self, insurance: Insurance) {
        self.public.insurances.put(&insurance.hash(), insurance);
    }

    pub fn save_loan_order(&mut self, loan_order: LoanOrder) {
        self.public.loan_orders.put(&loan_order.hash(), loan_order);
    }

}
