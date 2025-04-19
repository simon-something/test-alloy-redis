CREATE TABLE `swap_raw` (
  `block_number` int NOT NULL,
  `transaction_hash` varchar(66) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL,
  `sender` varchar(42) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL,
  `recipient` varchar(42) CHARACTER SET ascii COLLATE ascii_general_ci NOT NULL,
  `amount0` bigint,
  `amount1` bigint,
  `sqrtPriceX96` DECIMAL(38,0),
  `liquidity` bigint,
  `tick` int,
  PRIMARY KEY (`block_number`, `transaction_hash`)
);