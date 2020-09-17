use sbor::{describe::Type, *};

use crate::constants::*;
use crate::kernel::*;
use crate::rust::borrow::ToOwned;
use crate::types::*;

/// Represents a reference to a bucket.
#[derive(Debug, Encode, Decode)]
pub struct BucketRef {
    rid: RID,
}

impl From<RID> for BucketRef {
    fn from(rid: RID) -> Self {
        Self { rid }
    }
}

impl From<BucketRef> for RID {
    fn from(a: BucketRef) -> RID {
        a.rid
    }
}
impl Describe for BucketRef {
    fn describe() -> Type {
        Type::Custom {
            name: SCRYPTO_NAME_BUCKET_REF.to_owned(),
        }
    }
}

impl BucketRef {
    pub fn amount(&self) -> U256 {
        let input = GetAmountRefInput {
            reference: self.rid,
        };
        let output: GetAmountRefOutput = call_kernel(GET_AMOUNT_REF, input);

        output.amount
    }

    pub fn resource(&self) -> Address {
        let input = GetResourceRefInput {
            reference: self.rid,
        };
        let output: GetResourceRefOutput = call_kernel(GET_RESOURCE_REF, input);

        output.resource
    }

    pub fn drop(self) {
        let input = DropReferenceInput {
            reference: self.rid,
        };
        let _: DropReferenceOutput = call_kernel(DROP_REFERENCE, input);
    }
}
