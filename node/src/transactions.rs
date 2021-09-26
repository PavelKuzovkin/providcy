use exonum::{
    crypto::Hash,
    runtime::{CallerAddress as Address, CommonError, ExecutionContext, ExecutionError},
};
use exonum_derive::{exonum_interface, interface_method, BinaryValue, ExecutionFail, ObjectHash};
use exonum_proto::ProtobufConvert;

use crate::{proto, schema::SchemaImpl, DomRfService};
use exonum::crypto::hash;
use crate::insurance::Insurance;
use std::time::Instant;
use crate::loan_request::LoanRequest;
use crate::loan_order::LoanOrder;
use crate::utils::Utils;
use crate::borrower::Borrower;

/// Error codes emitted by wallet transactions during execution.
#[derive(Debug, ExecutionFail)]
pub enum Error {
    LoanRequestShouldHaveApprovedStatus = 2,
    LoanRequestAlreadyExists = 3,
    PolisyAlreadyExists = 4,
    LoanRequestDoesntExist = 5,
    InsuranceDoesntExist = 6,
    OrderDoesntExist = 7,
    PolicyWasNotFound = 8,
    PolicyWasFoundButInactual = 9
}

#[derive(Clone, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::TxCreateLoanRequest", serde_pb_convert)]
pub struct TxCreateLoanRequest {
    pub snils: String,
    pub fio: String,
    pub email: String,
    pub phone: String,
    pub bank: String,
    pub request_number: String,
    pub sum: u64,
    pub status: u32,
}

impl Into<LoanRequest> for TxCreateLoanRequest {
    fn into(self) -> LoanRequest {
        LoanRequest {
            snils: self.snils.to_string(),
            bank: self.bank.to_string(),
            request_number: self.request_number.to_string(),
            sum: self.sum,
            created_at: Utils::now(),
            status: self.status
        }
    }
}

impl Into<Borrower> for TxCreateLoanRequest {
    fn into(self) -> Borrower {
        Borrower {
            snils: self.snils.to_string(),
            fio: self.fio.to_string(),
            email: self.email.to_string(),
            phone: self.phone.to_string()
        }
    }
}

#[derive(Clone, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::TxUpdateLoanRequestStatus", serde_pb_convert)]
pub struct TxUpdateLoanRequestStatus {
    pub bank: String,
    pub request_number: String,
    pub status: u32,
}

#[derive(Clone, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::TxCreateInsurance", serde_pb_convert)]
pub struct TxCreateInsurance {
    pub snils: String,
    pub bank: String,
    pub request_number: String,
    pub order_number: String,
    pub insurer: String,
    pub policy_number: String,
    pub sum: u64,
    pub starts_at: u64,
    pub expires_at: u64
}

impl Into<Insurance> for TxCreateInsurance {
    fn into(self) -> Insurance {
        Insurance{
            snils: self.snils.to_string(),
            bank: self.bank.to_string(),
            request_number: self.request_number.to_string(),
            order_number: self.order_number.to_string(),
            insurer: self.insurer.to_string(),
            policy_number: self.policy_number.to_string(),
            sum: self.sum,
            created_at:  Utils::now(),
            starts_at: self.starts_at,
            expires_at: self.expires_at
        }
    }
}

#[derive(Clone, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::TxCreateLoanOrder", serde_pb_convert)]
pub struct TxCreateLoanOrder {
    pub snils: String,
    pub bank: String,
    pub request_number: String,
    pub order_number: String,
    pub sum: u64,
    pub expires_at: u64
}

impl Into<LoanOrder> for TxCreateLoanOrder {
    fn into(self) -> LoanOrder {
        LoanOrder {
            snils: self.snils.to_string(),
            bank: self.bank.to_string(),
            request_number: self.request_number.to_string(),
            order_number: self.order_number.to_string(),
            sum: self.sum,
            created_at: Utils::now(),
            expires_at: self.expires_at
        }
    }
}

#[derive(Clone, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::TxUpdateInsurance", serde_pb_convert)]
pub struct TxUpdateInsurance {
    pub insurer: String,
    pub policy_number: String,
    pub order_number: String
}

#[derive(Clone, Debug)]
#[derive(ProtobufConvert, BinaryValue, ObjectHash)]
#[protobuf_convert(source = "proto::TxUpdateLoanOrder", serde_pb_convert)]
pub struct TxUpdateLoanOrder {
    pub bank: String,
    pub order_number: String,
    pub sum: u64,
    pub expires_at: u64
}

/// Cryptocurrency service transactions.
#[exonum_interface]
pub trait DomRfServiceInterface<Ctx> {
    /// Output returned by the interface methods.
    type Output;

    /// Transfers `amount` of the currency from one wallet to another.
    #[interface_method(id = 0)]
    fn create_loan_request(&self, ctx: Ctx, arg: TxCreateLoanRequest) -> Self::Output;
    #[interface_method(id = 1)]
    fn update_loan_request_status(&self, ctx: Ctx, arg: TxUpdateLoanRequestStatus) -> Self::Output;
    #[interface_method(id = 2)]
    fn create_insurance(&self, ctx: Ctx, arg: TxCreateInsurance) -> Self::Output;
    #[interface_method(id = 3)]
    fn create_loan_order(&self, ctx: Ctx, arg: TxCreateLoanOrder) -> Self::Output;
    #[interface_method(id = 4)]
    fn update_insurance(&self, ctx: Ctx, arg: TxUpdateInsurance) -> Self::Output;
    #[interface_method(id = 5)]
    fn update_loan_order(&self, ctx: Ctx, arg: TxUpdateLoanOrder) -> Self::Output;
}

impl DomRfServiceInterface<ExecutionContext<'_>> for DomRfService {
    type Output = Result<(), ExecutionError>;

    /// Create Loan Request smart-contract
    fn create_loan_request(&self, context: ExecutionContext<'_>, arg: TxCreateLoanRequest) -> Self::Output {
        let (from, tx_hash) = extract_info(&context)?;

        // Check if the LoanRequest already exists
        let mut schema = SchemaImpl::new(context.service_data());
        let hash_string = Utils::hash_by_params(&arg.bank, &arg.request_number);
        if let Some(loan_request) = schema.loan_request(hash_string) {
            return Err(Error::LoanRequestAlreadyExists.into());
        }

        let borrower = arg.clone().into();
        let request = arg.into();
        schema.create_borrower(borrower);
        schema.create_loan_request(request);
        Ok(())
    }

    /// Update Loan Request smart-contract
    fn update_loan_request_status(&self, context: ExecutionContext<'_>, arg: TxUpdateLoanRequestStatus) -> Self::Output {
        let (from, tx_hash) = extract_info(&context)?;
        let mut schema = SchemaImpl::new(context.service_data());

        let hash_string = Utils::hash_by_params(&arg.bank, &arg.request_number);
        if let Some(loan_request) = schema.loan_request(hash_string) {
            let loan_request_hash = Utils::hash_by_params(&arg.bank, &arg.request_number);
            schema.update_loan_request_status(loan_request_hash, loan_request, arg.status);
        } else {
            return Err(Error::LoanRequestDoesntExist.into());
        }

        Ok(())
    }

    /// Create Insurance Policy smart-contract
    fn create_insurance(&self, context: ExecutionContext<'_>, arg: TxCreateInsurance) -> Self::Output {
        let (from, tx_hash) = extract_info(&context)?;
        let mut schema = SchemaImpl::new(context.service_data());

        let hash_string_ins = Utils::hash_by_params(&arg.insurer, &arg.policy_number);
        if schema.insurance(hash_string_ins).is_some() {
            return Err(Error::PolisyAlreadyExists.into());
        }

        let hash_string = Utils::hash_by_params(&arg.bank, &arg.request_number);
        if let Some(loan_request) = schema.loan_request(hash_string) {
            if loan_request.status != 2 {
                return Err(Error::LoanRequestShouldHaveApprovedStatus.into());
            }
        } else {
            return Err(Error::LoanRequestDoesntExist.into());
        }

        schema.save_insurance(arg.into());
        Ok(())
    }

    /// Create Loan Order smart-contract
    fn create_loan_order(&self, context: ExecutionContext<'_>, arg: TxCreateLoanOrder) -> Self::Output {
        let (from, tx_hash) = extract_info(&context)?;
        let mut schema = SchemaImpl::new(context.service_data());

        let hash_string = Utils::hash_by_params(&arg.bank, &arg.request_number);
        if let Some(loan_request) = schema.loan_request(hash_string) {
            if loan_request.status != 2 {
                return Err(Error::LoanRequestShouldHaveApprovedStatus.into());
            }
            if let Some(insurance) = schema.insurance_for_loan_request(loan_request) {
                let now =  Utils::now();
                if insurance.starts_at > now && insurance.expires_at < now {
                    return Err(Error::PolicyWasFoundButInactual.into());
                }
            } else {
                return Err(Error::PolicyWasNotFound.into());
            }
        } else {
            return Err(Error::LoanRequestDoesntExist.into());
        }
        schema.save_loan_order(arg.into());
        Ok(())
    }

    /// Update Insurance policy smart-contract
    fn update_insurance(&self, context: ExecutionContext<'_>, arg: TxUpdateInsurance) -> Self::Output {
        let (from, tx_hash) = extract_info(&context)?;
        let mut schema = SchemaImpl::new(context.service_data());
        let insurance_hash = Utils::hash_by_params(&arg.insurer, &arg.policy_number);
        if let Some(insurance) = schema.public.insurances.get(&insurance_hash) {
            let insurance = Insurance::update(insurance, &arg.order_number);
            schema.public.insurances.put(&insurance.hash(), insurance);
        } else {
            return Err(Error::InsuranceDoesntExist.into());
        }
        Ok(())
    }

    /// Update Loan Order smart-contract
    fn update_loan_order(&self, context: ExecutionContext<'_>, arg: TxUpdateLoanOrder) -> Self::Output {
        let (from, tx_hash) = extract_info(&context)?;
        let mut schema = SchemaImpl::new(context.service_data());
        let order_hash = Utils::hash_by_params(&arg.bank, &arg.order_number);
        if let Some(order) = schema.public.loan_orders.get(&order_hash) {
            let updated_order = LoanOrder::update(order, arg.sum, arg.expires_at);
            schema.public.loan_orders.put(&updated_order.hash(), updated_order);
        } else {
            return Err(Error::OrderDoesntExist.into());
        }
        Ok(())
    }

}

fn extract_info(context: &ExecutionContext<'_>) -> Result<(Address, Hash), ExecutionError> {
    let tx_hash = context
        .transaction_hash()
        .ok_or(CommonError::UnauthorizedCaller)?;
    let from = context.caller().address();
    Ok((from, tx_hash))
}
