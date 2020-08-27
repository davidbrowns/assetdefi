#![no_main]

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use scrypto::prelude::*;

blueprint! {
    struct Greeting {
        cnt:  u32
    }

    impl Greeting {
        pub fn new() -> Address {
            let component = Self {
                cnt: 0
            }.instantiate();
            debug!("New component: {}", component.address());
            component.into()
        }

        pub fn say_hello(&mut self) {
            info!("Hello, visitor #{}.", self.cnt);
            self.cnt += 1;
        }
    }
}
