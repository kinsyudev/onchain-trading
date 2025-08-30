# Solana Indexer - Raydium Trade Monitoring

## Overview

This application is a high-performance Solana blockchain indexer built with Carbon framework that monitors and indexes all trade events from Raydium, Solana's leading AMM DEX. It's part of a cross-chain arbitrage trading platform that identifies trading opportunities across multiple blockchains.

## Architecture

### Core Components

1. **Carbon Indexer Engine**
   - Real-time blockchain event monitoring using Solana RPC/WebSocket connections
   - Efficient filtering for Raydium program instructions
   - Transaction parsing and data extraction

2. **Data Pipeline**
   ```
   Solana Blockchain → Carbon Indexer → RabbitMQ → TypeScript Consumer → ClickHouse
   ```

3. **Event Processing**
   - Monitors Raydium AMM swap instructions
   - Extracts swap data: token pairs, amounts, prices, fees, timestamps
   - Enriches data with pool information and token metadata

### Key Features

- **Real-time Indexing**: Processes Raydium swaps as they occur on-chain
- **Data Extraction**: Captures comprehensive swap data including:
  - Token addresses (mint A/B)
  - Input/output amounts
  - Price impact
  - Pool liquidity state
  - Transaction signatures
  - Block time and slot
  - Trader addresses

- **Message Queue Integration**: Publishes normalized swap events to RabbitMQ for downstream processing
- **Error Resilience**: Automatic reconnection and backfill capabilities
- **Performance Optimized**: Async processing with minimal latency

## Data Model

### Swap Event Structure
```rust
struct RaydiumSwap {
    signature: String,
    slot: u64,
    timestamp: i64,
    pool_id: String,
    token_a_mint: String,
    token_b_mint: String,
    amount_in: u64,
    amount_out: u64,
    is_token_a_to_b: bool,
    trader: String,
    fee_amount: u64,
    price: f64,
    pool_liquidity_a: u64,
    pool_liquidity_b: u64,
}
```

## Configuration

### Environment Variables
- `SOLANA_RPC_URL`: Solana RPC endpoint (mainnet-beta)
- `RABBITMQ_URL`: Message queue connection string
- `RAYDIUM_PROGRAM_ID`: Raydium AMM program address

### Carbon Configuration
- Block processing: Real-time with 1-slot confirmation
- Retry policy: Exponential backoff for RPC failures
- Batch size: Optimized for Raydium transaction volume

## Integration with Trading Platform

This indexer feeds into the larger cross-chain arbitrage system:

1. **Event Emission**: Publishes standardized swap events to RabbitMQ
2. **Cross-Chain Analysis**: TypeScript consumers compare prices across chains
3. **Opportunity Detection**: Identifies arbitrage opportunities in real-time
4. **Data Storage**: ClickHouse stores hot data for analysis, MinIO archives historical data

## Performance Considerations

- Processes ~1000-2000 Raydium swaps per minute during peak times
- Sub-second latency from on-chain event to RabbitMQ publication
- Efficient memory usage through streaming processing
- Horizontal scaling supported via Carbon's architecture

## Monitoring

- Health checks exposed on `/health` endpoint
- Metrics include:
  - Events indexed per minute
  - RPC latency
  - Queue depth
  - Error rates
- Integrated with platform-wide observability stack