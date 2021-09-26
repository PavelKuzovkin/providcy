use exonum::crypto::{Hash, hash};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Utils{}

impl Utils {
    pub fn hash_by_params(insurer: &str, policy_number: &str) -> Hash {
        let mut hash_string = "".to_owned();
        hash_string.push_str(insurer);
        hash_string.push_str(policy_number);
        hash(hash_string.as_bytes())
    }
    pub fn now() -> u64 {
        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_secs()
    }
}