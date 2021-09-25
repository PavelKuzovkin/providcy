use exonum_cli::{NodeBuilder, Spec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {

    exonum::helpers::init_logger()?;

    NodeBuilder::development_node()?
        .with(Spec::new(CryptocurrencyService).with_default_instance())
        .run()
        .await
}
