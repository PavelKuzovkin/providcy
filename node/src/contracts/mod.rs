/// Contract errors.
pub mod errors {
    use exonum_derive::ExecutionFail;

    /// Error codes emitted by `TxCreateWallet` and/or `TxTransfer` transactions during execution.
    #[derive(Debug, ExecutionFail)]
    pub enum Error {
        /// Wallet already exists.
        ///
        /// Can be emitted by `TxCreateWallet`.
        WalletAlreadyExists = 0,
        /// Sender doesn't exist.
        ///
        /// Can be emitted by `TxTransfer`.
        SenderNotFound = 1,
        /// Receiver doesn't exist.
        ///
        /// Can be emitted by `TxTransfer`.
        ReceiverNotFound = 2,
        /// Insufficient currency amount.
        ///
        /// Can be emitted by `TxTransfer`.
        InsufficientCurrencyAmount = 3,
        /// Sender same as receiver.
        ///
        /// Can be emitted by `TxTransfer`.
        SenderSameAsReceiver = 4,
    }
}

/// Contracts.
pub mod contracts {
    use exonum::runtime::{ExecutionContext, ExecutionError};
    use exonum_derive::{exonum_interface, interface_method, ServiceDispatcher, ServiceFactory};
    use exonum_rust_runtime::{api::ServiceApiBuilder, DefaultInstance, Service};

    use crate::{
        api::CryptocurrencyApi,
        errors::Error,
        schema::{CurrencySchema, Wallet},
        transactions::{CreateWallet, TxTransfer},
    };

    /// Initial balance of a newly created wallet.
    const INIT_BALANCE: u64 = 100;

    /// Cryptocurrency service transactions.
    #[exonum_interface]
    pub trait CryptocurrencyInterface<Ctx> {
        /// Output of the methods in this interface.
        type Output;

        /// Creates wallet with the given `name`.
        #[interface_method(id = 0)]
        fn create_wallet(&self, ctx: Ctx, arg: CreateWallet) -> Self::Output;
        /// Transfers `amount` of the currency from one wallet to another.
        #[interface_method(id = 1)]
        fn transfer(&self, ctx: Ctx, arg: TxTransfer) -> Self::Output;
    }

    /// Cryptocurrency service implementation.
    #[derive(Debug, ServiceFactory, ServiceDispatcher)]
    #[service_dispatcher(implements("CryptocurrencyInterface"))]
    #[service_factory(proto_sources = "crate::proto")]
    pub struct CryptocurrencyService;

    impl CryptocurrencyInterface<ExecutionContext<'_>> for CryptocurrencyService {
        type Output = Result<(), ExecutionError>;

        fn create_wallet(&self, context: ExecutionContext<'_>, arg: CreateWallet) -> Self::Output {
            let author = context
                .caller()
                .author()
                .expect("Wrong `TxCreateWallet` initiator");

            let mut schema = CurrencySchema::new(context.service_data());
            if schema.wallets.get(&author).is_none() {
                let wallet = Wallet::new(&author, &arg.name, INIT_BALANCE);
                println!("Created wallet: {:?}", wallet);
                schema.wallets.put(&author, wallet);
                Ok(())
            } else {
                Err(Error::WalletAlreadyExists.into())
            }
        }

        fn transfer(&self, context: ExecutionContext<'_>, arg: TxTransfer) -> Self::Output {
            let author = context
                .caller()
                .author()
                .expect("Wrong 'TxTransfer' initiator");
            if author == arg.to {
                return Err(Error::SenderSameAsReceiver.into());
            }

            let mut schema = CurrencySchema::new(context.service_data());
            let sender = schema.wallets.get(&author).ok_or(Error::SenderNotFound)?;
            let receiver = schema.wallets.get(&arg.to).ok_or(Error::ReceiverNotFound)?;

            let amount = arg.amount;
            if sender.balance >= amount {
                let sender = sender.decrease(amount);
                let receiver = receiver.increase(amount);
                println!("Transfer between wallets: {:?} => {:?}", sender, receiver);
                schema.wallets.put(&author, sender);
                schema.wallets.put(&arg.to, receiver);
                Ok(())
            } else {
                Err(Error::InsufficientCurrencyAmount.into())
            }
        }
    }

    impl Service for CryptocurrencyService {
        fn wire_api(&self, builder: &mut ServiceApiBuilder) {
            CryptocurrencyApi::wire(builder);
        }
    }

    // Specify default instantiation parameters for the service.
    impl DefaultInstance for CryptocurrencyService {
        const INSTANCE_ID: u32 = 101;
        const INSTANCE_NAME: &'static str = "cryptocurrency";
    }
}