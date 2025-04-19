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
use std::env;
use uniswap_v3_sdk::prelude::sdk_core::utils::ToBig;

pub struct Sql_Handler {
    connection: MySqlConnection,
}

impl Sql_Handler {
    pub async fn new() -> Result<Self> {
        let database_url = std::env::var("MYSQL_DSN").expect("MYSQL_DSN must be set");
        let connection = MySqlConnection::connect(&database_url).await?;

        Ok(Self { connection })
    }

    pub async fn insert_raw_event(
        &mut self,
        block_number: u64,
        transaction_hash: String,
        log: IUniswapV3PoolEvents::Swap,
    ) -> Result<()> {
        sqlx::query("INSERT INTO swap_raw (block_number, transaction_hash, sender, recipient, amount0, amount1, sqrtPriceX96, liquidity, tick) VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?)")
                .bind(block_number)
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
