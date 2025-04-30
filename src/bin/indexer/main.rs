mod sql_handler;

use crate::sql_handler::Sql_Handler;
use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent,
};
use dotenv::dotenv;
use eyre::Result;
use futures_util::stream::StreamExt;
sol! {
    #[sol(rpc)]
    "interfaces/IUniswapV3PoolEvents.sol"
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let rpc_url = &std::env::var("RPC_URL").unwrap();
    let ws = WsConnect::new(rpc_url);

    let provider = ProviderBuilder::new().on_ws(ws).await?;
    let pool = address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640");

    let filter = Filter::new()
        .address(pool)
        .from_block(BlockNumberOrTag::Latest);

    let sub = provider.subscribe_logs(&filter).await?;
    let mut stream = sub.into_stream();

    let mut sql_handler = Sql_Handler::new().await?;

    while let Some(log) = stream.next().await {
        println!("usdc eth: {log:?}");
        match log.topic0() {
            Some(&IUniswapV3PoolEvents::Swap::SIGNATURE_HASH) => {
                let swap_event = log.log_decode()?.inner.data;

                let IUniswapV3PoolEvents::Swap {
                    sender,
                    recipient,
                    amount0,
                    amount1,
                    sqrtPriceX96,
                    liquidity,
                    tick,
                } = swap_event;

                println!(
                    "Swap from {sender} to {recipient} of value {amount0} and {amount1} at {sqrtPriceX96} with {liquidity} at {tick}"
                );

                // timestamps are inconsistent/null? not collected for poc
                sql_handler
                    .insert_raw_event(
                        log.block_number.unwrap(),
                        log.transaction_hash.unwrap().to_string(),
                        swap_event,
                    )
                    .await?;
            }
            _ => {}
        }
    }

    Ok(())
}
