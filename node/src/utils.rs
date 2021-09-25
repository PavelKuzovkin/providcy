use exonum::crypto::{Hash, hash};

pub struct Utils{}

impl Utils {
    pub fn hash_by_params(insurer: &str, policy_number: &str) -> Hash {
        let mut hash_string = "".to_owned();
        hash_string.push_str(insurer);
        hash_string.push_str(policy_number);
        hash(hash_string.as_bytes())
    }
}