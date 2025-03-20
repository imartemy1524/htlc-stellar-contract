use soroban_sdk::{contracttype, Address, U256, Env, Symbol, symbol_short};

pub(crate) const DAY_IN_LEDGERS: u32 = 17280;
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;
pub(crate) const DATA_BYTES_LENGTH: u32 = 112;

pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)]
#[contracttype]
pub struct DataItem {
    pub from: Address,
    pub to: Address,
    pub token: Address,
    pub expired_at: u64,
    pub hash: U256,
    pub amount: i128
}
pub fn read_item(e: &Env, id: u64) -> Option<DataItem> {
    if let Some(item) = e.storage().persistent().get::<u64, DataItem>(&id) {
        e.storage()
            .persistent()
            .extend_ttl(&id, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        Some(item)
    } else { None }
}
pub fn set_item(e: &Env, id: u64, item: &DataItem) {
     e.storage().persistent().set::<u64, DataItem>(&id, item);
     e.storage()
            .persistent()
            .extend_ttl(&id, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}
pub fn delete_item(e: &Env, id: u64){
    e.storage().persistent().remove::<u64>(&id);
}
const COUNTER: Symbol = symbol_short!("COUNTER");

pub fn get_max_count(e: &Env) -> u64{
    let count: u64 = e
        .storage()
        .instance()
        .get(&COUNTER)
        .unwrap_or(0) + 1; // If no value set, assume 0.

    e.storage()
        .instance()
        .set(&COUNTER, &count);

    count
}
