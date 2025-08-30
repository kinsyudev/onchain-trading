use {
    crate::config::Config,
    carbon_rpc_block_subscribe_datasource::{Filters, RpcBlockSubscribe},
    carbon_yellowstone_grpc_datasource::YellowstoneGrpcGeyserClient,
    carbon_pumpfun_decoder::PROGRAM_ID as PUMPFUN_PROGRAM_ID,
    carbon_raydium_amm_v4_decoder::PROGRAM_ID as RAYDIUMAMMV4_PROGRAM_ID,
    solana_client::rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
    solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey},
    std::{collections::{HashMap, HashSet}, sync::Arc},
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
    },
};

pub fn create_geyser_datasource(config: &Config) -> eyre::Result<YellowstoneGrpcGeyserClient> {
    let mut account_filters: HashMap<String, SubscribeRequestFilterAccounts> = HashMap::new();
    account_filters.insert(
        "program_accounts".to_string(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: vec![
                PUMPFUN_PROGRAM_ID.to_string(),
                RAYDIUMAMMV4_PROGRAM_ID.to_string(),
            ],
            filters: vec![],
            nonempty_txn_signature: None,
        },
    );

    let transaction_filter = SubscribeRequestFilterTransactions {
        vote: Some(false),
        failed: Some(false),
        account_include: vec![],
        account_exclude: vec![],
        account_required: vec![
            PUMPFUN_PROGRAM_ID.to_string(),
            RAYDIUMAMMV4_PROGRAM_ID.to_string(),
        ],
        signature: None,
    };

    let mut transaction_filters: HashMap<String, SubscribeRequestFilterTransactions> = HashMap::new();
    transaction_filters.insert("program_transactions".to_string(), transaction_filter);

    let datasource = YellowstoneGrpcGeyserClient::new(
        config.geyser_url.clone(),
        config.x_token.clone(),
        Some(CommitmentLevel::Confirmed),
        account_filters,
        transaction_filters,
        Default::default(),
        Arc::new(tokio::sync::RwLock::new(HashSet::<Pubkey>::new())),
    );

    Ok(datasource)
}

pub fn create_rpc_datasource(config: &Config) -> eyre::Result<RpcBlockSubscribe> {
    // Create filters for programs we're interested in
    let program_filter = RpcBlockSubscribeFilter::MentionsAccountOrProgram(
        format!("{},{}", PUMPFUN_PROGRAM_ID, RAYDIUMAMMV4_PROGRAM_ID)
    );

    let rpc_config = RpcBlockSubscribeConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        encoding: None,
        transaction_details: None,
        show_rewards: Some(false),
        max_supported_transaction_version: Some(0),
    };

    let filters = Filters::new(program_filter, Some(rpc_config));

    let datasource = RpcBlockSubscribe::new(config.rpc_ws_url.clone(), filters);

    Ok(datasource)
}

/// Create a combined datasource filter that includes multiple programs
pub fn create_program_filters() -> Vec<String> {
    vec![
        PUMPFUN_PROGRAM_ID.to_string(),
        RAYDIUMAMMV4_PROGRAM_ID.to_string(),
    ]
}
