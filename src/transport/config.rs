use std::time::Duration;

use solana_program::pubkey::Pubkey;

#[derive(Debug, Clone)]
pub enum Mode {
    Inproc,
    Emulator,
    Validator,
}

#[derive(Debug, Clone)]
pub struct TransportConfig {
    pub root : Pubkey,
    pub timeout : Duration,
    pub confirm_transaction_initial_timeout : Duration,
    pub retries : usize
}

impl TransportConfig {
    pub fn new(root : Pubkey, timeout: Duration, confirm_transaction_initial_timeout: Duration, retries: usize) -> TransportConfig {
        TransportConfig {
            root,
            timeout,
            confirm_transaction_initial_timeout,
            retries
        }
    }

    pub fn default_with_root(root : Pubkey) -> TransportConfig {
        let mut config = TransportConfig::default();
        config.root = root;
        config
    }
}

impl Default for TransportConfig {
    fn default() -> Self {
        TransportConfig {
            root : Pubkey::default(),
            timeout : Duration::from_secs(60u64),
            confirm_transaction_initial_timeout : Duration::from_secs(5u64),
            retries : 2
        }
    }
}
