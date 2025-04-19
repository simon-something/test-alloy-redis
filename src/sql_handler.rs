use crate::IUniswapV3PoolEvents;
use alloy::{
    primitives::address,
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::{BlockNumberOrTag, Filter},
    sol,
    sol_types::SolEvent,
};
use eyre::Result;
use sqlx::{
    Connection, MySql, Pool,
    mysql::{MySqlConnectOptions, MySqlConnection},
};

pub struct Sql_Handler {
    connection: MySqlConnection,
}

impl Sql_Handler {
    pub async fn new() -> Result<Self> {
        let opt = MySqlConnectOptions::new()
            .host("localhost")
            .port(3306)
            .username("indexer")
            .password("indexer")
            .database("all_events");
        let connection = MySqlConnection::connect_with(&opt).await?;

        Ok(Self { connection })
    }

    pub async fn insert_raw_event(
        &mut self,
        block_number: u64,
        block_timestamp: u64,
        transaction_hash: String,
        log: IUniswapV3PoolEvents::Swap,
    ) -> Result<()> {
        sqlx::query("INSERT INTO swap_events (block_number, block_timestamp, transaction_hash, sender, recipient, amount0, amount1, sqrtPriceX96, liquidity, tick) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(block_number)
                .bind(block_timestamp)
                .bind(transaction_hash)
                .bind(log.sender.to_string())
                .bind(log.recipient.to_string())
                .bind(log.amount0.to_string())
                .bind(log.amount1.to_string())
                .bind(log.sqrtPriceX96.to_string())
                .bind(log.liquidity.to_string())
                .bind(log.tick.to_string())
                .execute(&mut self.connection)
                .await
                .unwrap();
        Ok(())
    }
}
