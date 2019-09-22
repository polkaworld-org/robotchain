#![cfg_attr(not(any(test, feature = "std")), no_std)]

// Import the `contract!` macro
use ink_lang::contract;
use ink_core::storage;
use ink_core::env::DefaultSrmlTypes;
use ink_core::memory::format;

contract! {
    #![env = DefaultSrmlTypes]

    struct AIProcedure {
        // Storage Declaration
        owner: storage::Value<AccountId>,
        upload_time: storage::Value<u64>,
        store_url: storage::Value<String>,
        hash_value: storage::Value<Hash>,
    }

    impl Deploy for AIProcedure {
        fn deploy(&mut self, upload_time: u64, store_url: String, hash_value: Hash) {
            // Contract Constructor
            self.owner.set(env.caller());
            self.upload_time.set(env.now());
            self.store_url.set(store_url);
            self.hash_value.set(hash_value);
        }
    }

    impl AIProcedure {
        pub(external) fn get_owner(&self) -> AccountId {
            *self.owner
        }
        pub(external) fn get_upload_time(&self) -> u64 {
            *self.upload_time
        }
        pub(external) fn get_store_url(&self) -> String {
            *self.store_url
        }
        pub(external) fn get_hash_value(&self) -> Hash {
            *self.hash_value
        }
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn test_one_case() {
      
    }
}
