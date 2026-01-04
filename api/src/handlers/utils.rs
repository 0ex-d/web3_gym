use std::str::FromStr;
use std::sync::Arc;

use sui_sdk::rpc_types::SuiObjectDataOptions;
use sui_sdk::types::base_types::{ObjectID, SuiAddress};
use sui_sdk::types::object::Owner;
use sui_sdk::types::transaction::{Command, TransactionData, TransactionDataAPI, TransactionKind};

use redis::AsyncCommands;

use crate::errors::ApiError;
use crate::state::AppState;
use crate::utils::{arg_object_id, verified_key};

pub(crate) fn parse_address(address: &str) -> Result<SuiAddress, ApiError> {
    SuiAddress::from_str(address).map_err(|_| ApiError::bad_request("invalid address"))
}

pub(crate) fn parse_object_id(value: &str) -> Result<ObjectID, ApiError> {
    ObjectID::from_str(value).map_err(|_| ApiError::bad_request("invalid object id"))
}

pub(crate) async fn ensure_verified(
    state: &Arc<AppState>,
    address: SuiAddress,
) -> Result<(), ApiError> {
    let mut redis = state.redis.clone();
    let key = verified_key(address);
    let verified: Option<String> = redis.get(&key).await.map_err(ApiError::Redis)?;
    if verified.is_none() {
        return Err(ApiError::Unauthorized("challenge not verified".to_string()));
    }
    Ok(())
}

pub(crate) async fn ensure_membership_owner(
    state: &Arc<AppState>,
    membership_id: ObjectID,
    address: SuiAddress,
) -> Result<(), ApiError> {
    let response = state
        .sui
        .read_api()
        .get_object_with_options(membership_id, SuiObjectDataOptions::new().with_owner())
        .await
        .map_err(ApiError::Sui)?;
    let owner = response
        .owner()
        .ok_or_else(|| ApiError::bad_request("object has no owner"))?;
    match owner {
        Owner::AddressOwner(owner) if owner == address => Ok(()),
        _ => Err(ApiError::Unauthorized(
            "membership not owned by address".to_string(),
        )),
    }
}

pub(crate) fn validate_tx_call(
    state: &Arc<AppState>,
    tx_data: &TransactionData,
) -> Result<(), ApiError> {
    let TransactionKind::ProgrammableTransaction(pt) = tx_data.kind() else {
        return Err(ApiError::bad_request("unexpected transaction kind"));
    };

    if pt.commands.len() != 1 {
        return Err(ApiError::bad_request("unexpected command count"));
    }

    let Command::MoveCall(call) = &pt.commands[0] else {
        return Err(ApiError::bad_request("expected move call"));
    };

    if call.package != state.config.package_id
        || call.module != state.config.module
        || call.function != state.config.verify_fn
    {
        return Err(ApiError::bad_request("unexpected move call"));
    }

    let inputs = &pt.inputs;
    if call.arguments.len() != 2 {
        return Err(ApiError::bad_request("unexpected argument count"));
    }

    let _membership = arg_object_id(&call.arguments[0], inputs)
        .ok_or_else(|| ApiError::bad_request("missing membership object"))?;
    let clock = arg_object_id(&call.arguments[1], inputs)
        .ok_or_else(|| ApiError::bad_request("missing clock object"))?;

    if clock != state.config.clock_id {
        return Err(ApiError::bad_request("unexpected clock id"));
    }

    Ok(())
}
