use rand::RngCore;
use rand::rngs::OsRng;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::types::transaction::{Argument, CallArg, ObjectArg};

pub(crate) fn generate_nonce() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub(crate) fn challenge_key(address: SuiAddress) -> String {
    format!("challenge:{}", address)
}

pub(crate) fn verified_key(address: SuiAddress) -> String {
    format!("verified:{}", address)
}

pub(crate) fn prepared_key(address: SuiAddress, digest: &str) -> String {
    format!("prepared:{}:{}", address, digest)
}

pub(crate) fn arg_object_id(arg: &Argument, inputs: &[CallArg]) -> Option<ObjectID> {
    let Argument::Input(idx) = arg else {
        return None;
    };
    let input = inputs.get(*idx as usize)?;
    match input {
        CallArg::Object(ObjectArg::ImmOrOwnedObject(obj_ref)) => Some(obj_ref.0),
        CallArg::Object(ObjectArg::SharedObject { id, .. }) => Some(*id),
        CallArg::Object(ObjectArg::Receiving(obj_ref)) => Some(obj_ref.0),
        _ => None,
    }
}
