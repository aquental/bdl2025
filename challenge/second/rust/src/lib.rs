use bitcoincore_rpc::{Auth, Client};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DEFAULT_RPC_IP: &str = "127.0.0.1";
const DEFAULT_RPC_PORT: u16 = 8332;
const DEFAULT_RPC_PORT_STR: &str = "8332";

const ENV_RPC_CONNECT: &str = "rpcconnect";
const ENV_RPC_PORT: &str = "rpcport";
const ENV_RPC_USER: &str = "rpcuser";
const ENV_RPC_PASSWORD: &str = "rpcpassword";

//const DEFAULT_BITCOIN_CONF_PATH: &str = "~/.bitcoin/bitcoin.conf";
fn get_default_bitcoin_conf_path() -> Option<PathBuf> {
    env::home_dir().map(|home| home.join(".bitcoin").join("bitcoin.conf"))
}

pub fn read_bitcoin_conf(
    path: &Path,
) -> Result<(String, u16, String, String), Box<dyn std::error::Error>> {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                // Fallback to default Bitcoin conf path (~/.bitcoin/bitcoin.conf)
                let default_path = get_default_bitcoin_conf_path();
                match default_path {
                    Some(dp) => match fs::read_to_string(&dp) {
                        Ok(content) => {
                            println!("Using default bitcoin.conf at {:?}", dp);
                            content
                        }
                        Err(de) => {
                            if de.kind() == std::io::ErrorKind::NotFound {
                                println!(
                                    "Default bitcoin.conf not found; falling back to environment variables."
                                );
                                return read_from_env();
                            } else {
                                return Err(de.into());
                            }
                        }
                    },
                    None => {
                        println!(
                            "Could not determine default bitcoin.conf path; falling back to environment variables."
                        );
                        return read_from_env();
                    }
                }
            } else {
                return Err(e.into());
            }
        }
    };
    let mut rpc_ip = "127.0.0.1".to_string();
    let mut rpc_port: u16 = 8332; // Default mainnet
    let mut rpc_user = String::new();
    let mut rpc_password = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(eq_pos) = line.find('=') {
            let key: &str = &(&line[..eq_pos].trim().to_lowercase());
            let value = &line[eq_pos + 1..].trim();
            match key {
                "rpcconnect" => {
                    let parts: Vec<&str> = value.split(':').collect();
                    if parts.len() == 2 {
                        rpc_ip = parts[0].trim().to_string();
                        if let Ok(port) = parts[1].trim().parse::<u16>() {
                            rpc_port = port;
                        }
                        // If port parse fails, keep default rpc_port
                    } else {
                        rpc_ip = value.to_string();
                    }
                }
                "rpcport" => rpc_port = value.parse()?,
                "rpcuser" => rpc_user = value.to_string(),
                "rpcpassword" => rpc_password = value.to_string(),
                _ => {}
            }
        }
    }

    if rpc_user.is_empty() || rpc_password.is_empty() {
        return Err("rpcuser and rpcpassword must be set in bitcoin.conf".into());
    }

    Ok((rpc_ip, rpc_port, rpc_user, rpc_password))
}

#[allow(unused_assignments)]
pub fn read_from_env() -> Result<(String, u16, String, String), Box<dyn std::error::Error>> {
    let rpc_connect = env::var(ENV_RPC_CONNECT).unwrap_or_else(|_| DEFAULT_RPC_IP.to_string());
    let mut rpc_ip = rpc_connect.clone();
    let mut rpc_port: u16 = DEFAULT_RPC_PORT;

    // Check if RPC_CONNECT has port
    let parts: Vec<&str> = rpc_connect.split(':').collect();
    if parts.len() == 2 {
        rpc_ip = parts[0].trim().to_string();
        if let Ok(port) = parts[1].trim().parse::<u16>() {
            rpc_port = port;
        }
        // If port parse fails, keep default rpc_port
    }

    // RPC_PORT overrides if set
    let rpc_port_str = env::var(ENV_RPC_PORT).unwrap_or_else(|_| DEFAULT_RPC_PORT_STR.to_string());
    if let Ok(port) = rpc_port_str.parse::<u16>() {
        rpc_port = port;
    } else {
        return Err("Invalid RPC_PORT value".into());
    }

    let rpc_user = env::var(ENV_RPC_USER).map_err(|_| "rpcuser must be set")?;
    let rpc_password = env::var(ENV_RPC_PASSWORD).map_err(|_| "rpcpassword must be set")?;

    Ok((rpc_ip, rpc_port, rpc_user, rpc_password))
}

pub fn get_rpc_from_config() -> Result<Client, Box<dyn std::error::Error>> {
    // Path to bitcoin.conf (e.g., "~/.bitcoin/bitcoin.conf")
    let config_path = Path::new("./bitcoin.conf"); // Or env::home_dir().unwrap().join(".bitcoin/bitcoin.conf")

    let (rpc_ip, rpc_port, rpc_user, rpc_password) = read_bitcoin_conf(config_path)?;
    // Build RPC URL
    let rpc_url = format!("http://{}:{}", rpc_ip, rpc_port);

    // Connect to the RPC endpoint with credentials
    let rpc = Client::new(&rpc_url, Auth::UserPass(rpc_user, rpc_password))
        .expect("Failed to connect to Bitcoin Core RPC");

    Ok(rpc)
}
