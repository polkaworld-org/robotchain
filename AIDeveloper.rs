#![cfg_attr(not(any(test, feature = "std")), no_std)]

// Import the `contract!` macro
use ink_lang::contract;
use ink_core::storage;
use ink_core::env::DefaultSrmlTypes;
use ink_core::memory::format;

contract! {
    #![env = DefaultSrmlTypes]

    struct AIDeveloper {
        // Storage Declaration
        name: storage::Value<String>,
        org: storage::Value<String>,
        owner: storage::Value<AccountId>,
        aiProcedures: storage::Vec<AIProcedure>,
    }

    impl Deploy for AIDeveloper {
        fn deploy(&mut self, name: String, org: String) {
            // Contract Constructor
            self.owner.set(env.caller());
            self.name.set(name);
            self.org.set(org);
        }
    }

    impl AIDeveloper {
        // Implementation of Contract Functions
        pub(external) fn add_procedure(&mut self, upload_time: u64, store_url: String, hash_value: Hash) {
            let aiProcedure = AIProcedure::deploy_mock(upload_time, store_url, hash_value);
            self.aiProcedures.push(aiProcedure);
        }

        pub(external) fn num_of_procedures(&self) -> u32 {
          self.aiProcedures.len()
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
