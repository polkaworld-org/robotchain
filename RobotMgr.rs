#![cfg_attr(not(any(test, feature = "std")), no_std)]

// Import the `contract!` macro
use ink_lang::contract;
use ink_core::storage;
use ink_core::env::DefaultSrmlTypes;
use ink_core::memory::format;

contract! {
    #![env = DefaultSrmlTypes]

    struct RobotMgr {
        // Storage Declaration
        robots: storage::Vec<Robot>,
    }

    impl Deploy for RobotMgr {
        fn deploy(&mut self, upload_time: u64, body_image_url: String) {
            // Contract Constructor
            self.owner.set(env.caller());
            self.body_image_url.set(body_image_url);
        }
    }

    impl RobotMgr {
    }
}

#[cfg(all(test, feature = "test-env"))]
mod tests {
    use super::*;

    #[test]
    fn test_one_case() {
      
    }
}
