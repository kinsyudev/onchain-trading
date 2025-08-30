# Solana Indexer

A high-performance Solana blockchain indexer built with the Carbon framework, supporting both Pumpfun and Raydium AMM V4 protocols.

## Features

- 🚀 **Multiple Data Sources**: Support for both Yellowstone gRPC (high-performance) and RPC WebSocket (development)
- 🔄 **Protocol Support**: Built-in decoders for Pumpfun and Raydium AMM V4
- 📊 **Comprehensive Logging**: Detailed transaction and instruction logging with emojis for easy identification
- ⚙️ **Flexible Configuration**: Environment-based configuration with validation
- 🏗️ **Modular Architecture**: Clean separation of concerns with dedicated modules

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── datasources.rs       # Data source creation and management
└── processors/
    ├── mod.rs           # Processor module exports
    ├── pumpfun.rs       # Pumpfun instruction processor
    └── raydium.rs       # Raydium AMM V4 instruction processor
```

## Configuration

Create a `.env` file in the project root with the following variables:

```bash
# Data Source Configuration
# Options: "geyser", "rpc", "both"
DATA_SOURCE=geyser

# Geyser Configuration (for high-performance indexing)
GEYSER_URL=https://api.mainnet-beta.solana.com
X_TOKEN=your_auth_token_here

# RPC Configuration (for development and testing)
RPC_URL=https://api.mainnet-beta.solana.com
RPC_WS_URL=wss://api.mainnet-beta.solana.com

# Logging
LOG_LEVEL=info
```

### Configuration Options

#### Data Source Types

1. **`geyser`**: High-performance Yellowstone gRPC data source
   - Best for production environments
   - Requires `GEYSER_URL` and optionally `X_TOKEN`

2. **`rpc`**: RPC WebSocket data source  
   - Good for development and testing
   - Requires `RPC_WS_URL`

3. **`both`**: Uses both data sources for redundancy
   - Maximum reliability
   - Requires all connection parameters

#### Environment Examples

**Development Setup:**
```bash
DATA_SOURCE=rpc
RPC_URL=https://api.devnet.solana.com
RPC_WS_URL=wss://api.devnet.solana.com
LOG_LEVEL=debug
```

**Production Setup:**
```bash
DATA_SOURCE=geyser
GEYSER_URL=https://your-geyser-endpoint.com
X_TOKEN=your_production_token
LOG_LEVEL=info
```

## Running the Indexer

1. **Install dependencies:**
   ```bash
   cargo build
   ```

2. **Set up environment:**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

3. **Run the indexer:**
   ```bash
   cargo run
   ```

## Supported Instructions

### Pumpfun Instructions
- ✅ **CreateEvent**: New token launches
- ✅ **TradeEvent**: Buy/sell transactions  
- ✅ **CompleteEvent**: Token graduation events

### Raydium AMM V4 Instructions
- ✅ **SwapBaseIn**: Token swaps (base token in)
- ✅ **SwapBaseOut**: Token swaps (base token out)
- ✅ **Initialize**: Pool initialization
- ✅ **Deposit**: Liquidity deposits
- ✅ **Withdraw**: Liquidity withdrawals

## Log Output Examples

```
🚀 Pumpfun CreateEvent - Signature: 2x..., Slot: 123456, Mint: 7x..., Name: MyToken, Symbol: MTK
💱 Pumpfun TradeEvent - Signature: 3x..., Slot: 123457, Mint: 7x..., Sol Amount: 1000000, Is Buy: true
🔄 Raydium SwapBaseIn - Signature: 4x..., Slot: 123458, Amount In: 500000, Min Amount Out: 450000
```

## Architecture

The indexer uses the Carbon framework's pipeline architecture:

1. **Data Sources** → Fetch blockchain data from Solana
2. **Decoders** → Parse raw instruction data into typed structures  
3. **Processors** → Handle decoded instructions with business logic
4. **Metrics** → Monitor pipeline performance

## Performance Considerations

- **Geyser Data Source**: Recommended for production, handles high transaction volumes
- **RPC Data Source**: Suitable for development, has rate limits
- **Both Data Sources**: Provides redundancy but uses more resources

## Error Handling

The indexer includes comprehensive error handling:
- Configuration validation on startup
- Automatic retry logic for transient failures  
- Graceful shutdown with pending transaction processing
- Detailed error logging with context

## Development

To extend the indexer:

1. **Add new processors**: Create new files in `src/processors/`
2. **Add new decoders**: Update dependencies and wire into pipeline
3. **Modify configuration**: Extend the `Config` struct in `config.rs`
4. **Add data sources**: Implement new sources in `datasources.rs`

## Dependencies

- **Carbon Framework**: Core indexing infrastructure
- **Solana SDK**: Blockchain interaction utilities
- **Tokio**: Async runtime
- **Eyre**: Error handling
- **Serde**: Serialization/deserialization
