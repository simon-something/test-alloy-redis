CREATE TABLE `swap_events` (
  `block_number` int NOT NULL,
  `block_timestamp` datetime NOT NULL,
  `transaction_hash` varchar(66) COLLATE utf8mb4_unicode_ci NOT NULL,
  `sender` varchar(42) COLLATE utf8mb4_unicode_ci NOT NULL,
  `recipient` varchar(42) COLLATE utf8mb4_unicode_ci NOT NULL,
  `amount0` bigint NOT NULL,
  `amount1` bigint NOT NULL,
  `sqrtPriceX96` bigint NOT NULL,
  `liquidity` bigint NOT NULL,
  `tick` int NOT NULL,
  PRIMARY KEY (`block_number`, `transaction_hash`)
);