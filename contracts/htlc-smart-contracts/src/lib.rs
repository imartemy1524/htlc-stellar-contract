#![no_std]
mod storage;
use crate::storage::{DataItem, get_max_count, set_item, read_item, DATA_BYTES_LENGTH};
use soroban_sdk::{contract, contracterror, contractimpl, token, Address, Bytes, Env, U256};
use storage::delete_item;
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    NotFound = 1,
    NotExpiredYet = 2,
    AlreadyExpired = 3,
    InvalidSignature = 4
}
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {

    pub fn create(env: Env, from: Address, to: Address, token: Address, amount: i128, expired_at: u64, hash: U256) -> u64 {
        from.require_auth();

        move_token(&env, &token, &from, &env.current_contract_address(), amount);
        let id = get_max_count(&env);
        // add the item
        set_item(&env, id, &DataItem{
            expired_at,
            token,
            from,
            to,
            amount,
            hash
        });

        id
    }

    pub fn cancel_expired(env: Env, id: u64) -> Result<bool, Error>{
        if let Some(item) = read_item(&env, id){
            let w: DataItem = item;
            if !check_expired(&env, w.expired_at) {
                return Err(Error::NotExpiredYet);
            }
            delete_item(&env, id);
            move_token(&env, &w.token, &env.current_contract_address(), &w.to, w.amount);
            Ok(true)
        } else{
            Err(Error::NotFound)
        }
    }
    pub fn provide_data(env: Env, id: u64, data: Bytes) -> Result<bool, Error> {
        if let Some(item) = read_item(&env, id){
            let DataItem { expired_at, token, from: _, to, amount, hash } = item;
            if check_expired(&env, expired_at) {
                return Err(Error::AlreadyExpired);
            }
            if !valid_signature(&env, &hash, &data){
                return Err(Error::InvalidSignature);
            }
            delete_item(&env, id);
            move_token(&env, &token, &env.current_contract_address(), &to, amount);
            Ok(true)
        } else{
            Err(Error::NotFound)
        }
    }

    pub fn get_event(env: Env, id: u64) -> Option<DataItem> {
        return read_item(&env, id);
    }
}

mod test;
fn check_expired(env: &Env, expired_at: u64) -> bool {
    let now = env.ledger().timestamp();
    return expired_at < now

}
fn move_token(
    env: &Env,
    token: &Address,
    from: &Address,
    to: &Address,
    amount: i128,
) {
    let token = token::Client::new(env, token);
    token.transfer(from, &to, &amount);
}
fn valid_signature(env: &Env, hash: &U256, data: &Bytes) -> bool {
    if data.len() != DATA_BYTES_LENGTH {
        return false;
    }
    let signature = env.crypto().sha256(data);
    let bytes = signature.to_bytes();
    let bytes_real = Bytes::from_slice(&env, &bytes.to_array());

    let val = U256::from_be_bytes(&env, &bytes_real);
    val == *hash
}
