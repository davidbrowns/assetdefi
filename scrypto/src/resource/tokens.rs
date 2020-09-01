use sbor::{describe::Type, *};

use crate::constants::*;
use crate::resource::*;
use crate::rust::borrow::ToOwned;
use crate::types::*;

/// A bucket that holds tokens.
#[derive(Debug, Encode, Decode)]
pub struct Tokens {
    bid: BID,
}

impl From<BID> for Tokens {
    fn from(bid: BID) -> Self {
        Self { bid }
    }
}

impl Into<BID> for Tokens {
    fn into(self) -> BID {
        self.bid
    }
}

impl Tokens {
    pub fn check(&self, resource: Address) {
        assert!(self.resource() == resource);
    }

    pub fn new_empty(resource: Address) -> Self {
        BID::new_empty(resource).into()
    }

    pub fn put(&self, other: Self) {
        self.bid.put(other.bid);
    }

    pub fn take(&self, amount: U256) -> Self {
        self.bid.take(amount).into()
    }

    pub fn borrow(&self) -> TokensRef {
        self.bid.borrow().into()
    }

    pub fn amount(&self) -> U256 {
        self.bid.amount()
    }

    pub fn resource(&self) -> Address {
        self.bid.resource()
    }
}

impl Describe for Tokens {
    fn describe() -> Type {
        Type::Custom {
            name: SCRYPTO_NAME_TOKENS.to_owned(),
        }
    }
}
