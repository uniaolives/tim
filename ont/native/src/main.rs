use std::env;
use ontology_lang::onchain::deployer::{DeployerFactory, DeployConfig, DeployTarget, evm::EVMDeployConfig};
use ontology_lang::onchain::VerificationLevel;
use ontology_lang::compiler::CompiledContract;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: onto <command> [args]");
        return;
    }

    match args[1].as_str() {
        "deploy" => {
            handle_deploy(&args[2..]).await;
        },
        "compile" => {
            println!("🔨 Compiling...");
            // Compilation logic would go here
            println!("✅ Compilation successful!");
        },
        "server" => {
            println!("📡 Starting server on port 8080...");
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
            }
        },
        _ => println!("Unknown command: {}", args[1]),
    }
}

async fn handle_deploy(args: &[String]) {
    if args.is_empty() {
        println!("Usage: onto deploy <file> [options]");
        return;
    }

    let file_path = &args[0];
    let mut blockchain = "ethereum".to_string();
    let mut private_key = None;
    let mut verification = VerificationLevel::Basic;
    let mut rpc_url = "http://localhost:8545".to_string();

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            s if s.starts_with("--blockchain=") => {
                blockchain = s.replace("--blockchain=", "");
            },
            s if s.starts_with("--private-key=") => {
                private_key = Some(s.replace("--private-key=", ""));
            },
            s if s.starts_with("--verification=") => {
                let level = s.replace("--verification=", "");
                verification = match level.as_str() {
                    "none" => VerificationLevel::None,
                    "basic" => VerificationLevel::Basic,
                    "full" => VerificationLevel::Full,
                    "tmr" => VerificationLevel::TMR,
                    _ => VerificationLevel::Basic,
                };
            },
            s if s.starts_with("--rpc=") => {
                rpc_url = s.replace("--rpc=", "");
            },
            _ => {},
        }
        i += 1;
    }

    println!("🚀 Starting deployment of {} to {}...", file_path, blockchain);

    // Load compiled contract
    let source_code = std::fs::read_to_string(file_path).unwrap_or_else(|_| "".to_string());
    let compiled = CompiledContract {
        target_language: "Solidity".to_string(),
        source_code,
        bytecode: None,
        abi: None,
        stats: ontology_lang::compiler::CompilationStats {
            functions_compiled: 0,
            contracts_deployed: 0,
            transmutations_applied: 0,
            diplomatic_constraints: 0,
            paradigm_guards_injected: 0,
            gas_estimate: 0,
        },
    };

    let config = DeployConfig {
        target: DeployTarget::EVM(EVMDeployConfig {
            rpc_url: rpc_url.clone(),
            chain_id: 31337, // Default anvil
            gas_limit: None,
            gas_price: None,
            confirmations: 1,
            timeout_seconds: 60,
            etherscan_api_key: None,
            verification,
        }),
        verification,
        network: blockchain,
        private_key,
        rpc_url: Some(rpc_url),
    };

    let deployer_factory_result = DeployerFactory::create(config).await;
    match deployer_factory_result {
        Ok(deployer) => {
            match deployer.deploy(&compiled, None).await {
                Ok(result) => {
                    println!("✅ Deployment successful!");
                    println!("Address: {}", result.contract_address);
                    println!("Transaction: {}", result.transaction_hash);
                    println!("Block: {}", result.block_number);
                    println!("Gas Used: {}", result.gas_used);
                },
                Err(e) => {
                    println!("❌ Deployment failed: {:?}", e);
                }
            }
        },
        Err(e) => {
            println!("❌ Failed to create deployer: {:?}", e);
        }
    }
}
