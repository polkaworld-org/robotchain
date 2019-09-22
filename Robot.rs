#![cfg_attr(not(any(test, feature = "std")), no_std)]

// Import the `contract!` macro
use ink_lang::contract;
use ink_core::storage;
use ink_core::env::DefaultSrmlTypes;
use ink_core::memory::format;

contract! {
    #![env = DefaultSrmlTypes]

    struct Robot {
        // Storage Declaration
        owner: storage::Value<AccountId>,
        body_image_url: storage::Value<String>,
        ai_procedure: storage::Value<AIProcedure>,
    }

    impl Deploy for Robot {
        fn deploy(&mut self, upload_time: u64, body_image_url: String) {
            // Contract Constructor
            self.owner.set(env.caller());
            self.body_image_url.set(body_image_url);
        }
    }

    impl Robot {
        pub(external) fn bind_ai_procedure(&mut self, ai_procedure: &AIProcedure) {
            self.ai_procedure.set(ai_procedure);
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
