#![cfg_attr(not(feature = "std"), no_std)]

use scrypto::abi;
use scrypto::buffer::*;
use scrypto::prelude::*;
use serde::Serialize;
use serde_json::{json, to_value, Value};

blueprint! {
    struct Simple {
        state: u32,
    }

    impl Simple {
        pub fn new() -> Component {
            Self {
                state: 0
            }.instantiate()
        }

        pub fn get_state(&self) -> u32 {
            self.state
        }

        pub fn set_state(&mut self, new_state: u32) {
            self.state = new_state;
        }

        pub fn custom_types_1() -> (Amount, Address, H256, Bid, Rid, Mid, Vid) {
            todo!()
        }

        pub fn custom_types_2() -> (Package, Blueprint, Component, LazyMap) {
            todo!()
        }

        pub fn custom_types_3() -> (Bucket, BucketRef, Vault) {
            todo!()
        }
    }
}

fn assert_json_eq<T: Serialize>(actual: T, expected: Value) {
    assert_eq!(to_value(&actual).unwrap(), expected);
}

#[test]
fn test_simple_abi() {
    let ptr = Simple_abi();
    let abi: (Vec<abi::Function>, Vec<abi::Method>) =
        unsafe { scrypto_consume(ptr, |slice| scrypto_decode(slice).unwrap()) };

    assert_json_eq(
        abi,
        json!([
            [
                {
                    "name": "new",
                    "inputs": [],
                    "output": {
                        "type": "Custom",
                        "name": "scrypto::core::Component"
                    }
                },
                {
                    "name": "custom_types_1",
                    "inputs": [],
                    "output": {
                        "type": "Tuple",
                        "elements": [
                            {
                                "type": "Custom",
                                "name": "scrypto::types::Amount"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::types::Address"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::types::H256"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::types::Bid"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::types::Rid"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::types::Mid"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::types::Vid"
                            }
                        ]
                    }
                },
                {
                    "name": "custom_types_2",
                    "inputs": [],
                    "output": {
                        "type": "Tuple",
                        "elements": [
                            {
                                "type": "Custom",
                                "name": "scrypto::core::Package"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::core::Blueprint"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::core::Component"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::core::LazyMap"
                            }
                        ]
                    }
                },
                {
                    "name": "custom_types_3",
                    "inputs": [],
                    "output": {
                        "type": "Tuple",
                        "elements": [
                            {
                                "type": "Custom",
                                "name": "scrypto::resource::Bucket"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::resource::BucketRef"
                            },
                            {
                                "type": "Custom",
                                "name": "scrypto::resource::Vault"
                            }
                        ]
                    }
                }
            ],
            [
                {
                    "name": "get_state",
                    "mutability": "Immutable",
                    "inputs": [],
                    "output": {
                        "type": "U32"
                    }
                },
                {
                    "name": "set_state",
                    "mutability": "Mutable",
                    "inputs": [
                        {
                            "type": "U32"
                        }
                    ],
                    "output": {
                        "type": "Unit"
                    }
                }
            ]
        ]),
    );
}
