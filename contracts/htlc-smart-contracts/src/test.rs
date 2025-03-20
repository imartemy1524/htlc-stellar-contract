#![cfg(test)]
extern crate std;

use super::*;
use soroban_sdk::testutils::Ledger;
use soroban_sdk::token::TokenClient;
use soroban_sdk::{ BytesN, Env};
use soroban_sdk::testutils::{Address as AddrTest};
use token::StellarAssetClient as TokenAdminClient;

fn create_token_contract<'a>(e: &Env, admin: &Address) -> (TokenClient<'a>, TokenAdminClient<'a>) {
    let sac = e.register_stellar_asset_contract_v2(admin.clone());
    (
        token::Client::new(e, &sac.address()),
        token::StellarAssetClient::new(e, &sac.address()),
    )
}

struct HTLCContractDataTest<'a> {
    pub env: Env,
    pub from_user: Address,
    pub to_user: Address,
    pub client: ContractClient<'static>,
    pub token: (TokenClient<'a>, TokenAdminClient<'a>),
    pub bytes: Bytes,
}



impl<'a> HTLCContractDataTest<'a> {
    fn setup() -> Self {
            let env = Env::default();
            env.mock_all_auths();
            env.ledger().set_timestamp(10);

            let from_user = Address::generate(&env);
            let to_user = Address::generate(&env);
            let contract_id = env.register(Contract, ());
            let client = ContractClient::new(&env, &contract_id);

            let random_bytes = env.as_contract(&contract_id, || {
                env.prng().gen::<BytesN<112>>()
            });
            let bytes = Bytes::from_array(&env, &random_bytes.to_array());
            let token = create_token_contract(&env, &from_user);
            // mint 100 tokens
            token.1.mint(&from_user, &100);
            HTLCContractDataTest {
                env,
                from_user,
                to_user,
                client,
                bytes,
                token,
                // bytes_arr: random_bytes
            }
        }
}



#[test]
fn test_repay() {
    let data = HTLCContractDataTest::setup();

    let hash = U256::from_be_bytes(&data.env, &Bytes::from_array(&data.env, &data.env.crypto().sha256(&data.bytes).to_array()));
    let id = data.client.create(
        &data.from_user,
        &data.to_user,
        &data.token.0.address,
        &100,
        &1000,
        &hash,
    );
    // check that id is 0
    assert_eq!(id, 1);

    let balance = data.token.0.balance(&data.client.address);
    assert_eq!(balance, 100);
    let balance_owner = data.token.0.balance(&data.from_user);
    assert_eq!(balance_owner, 0);



    // repay the contract
    let ans = data.client.provide_data(&id, &data.bytes);
    assert!(ans);

    let balance = data.token.0.balance(&data.client.address);
    assert_eq!(balance, 0);
    let balance_owner = data.token.0.balance(&data.to_user);
    assert_eq!(balance_owner, 100);
}

#[test]
fn test_repay_with_wrong_data() {
    let data = HTLCContractDataTest::setup();

    let hash = U256::from_be_bytes(&data.env, &Bytes::from_array(&data.env, &data.env.crypto().sha256(&data.bytes).to_array()));
    let id = data.client.create(
        &data.from_user,
        &data.to_user,
        &data.token.0.address,
        &100,
        &1000,
        &hash,
    );
    // should panic
    assert_eq!(data.client.try_provide_data(&id, &Bytes::from_array(&data.env, &[0; 112])), Err(Ok(Error::InvalidSignature)));
}
#[test]
fn test_expired() {
    let data = HTLCContractDataTest::setup();

    let hash = U256::from_be_bytes(&data.env, &Bytes::from_array(&data.env, &data.env.crypto().sha256(&data.bytes).to_array()));
    let id = data.client.create(
        &data.from_user,
        &data.to_user,
        &data.token.0.address,
        &100,
        &5,
        &hash,
    );
    // try to provide data after expiration
    let err = data.client.try_provide_data(&id, &data.bytes);
    assert_eq!(err, Err(Ok(Error::AlreadyExpired)));
    let balance = data.token.0.balance(&data.client.address);
    assert_eq!(balance, 100);
    let balance_owner = data.token.0.balance(&data.to_user);
    assert_eq!(balance_owner, 0);
    let balance_from_user = data.token.0.balance(&data.from_user);
    assert_eq!(balance_from_user, 0);

    // withdraw funds from expired contract
     data.client.cancel_expired(&id);
    let balance = data.token.0.balance(&data.client.address);
    assert_eq!(balance, 0);
    let balance_owner = data.token.0.balance(&data.to_user);
    assert_eq!(balance_owner, 100);
    let balance_from_user = data.token.0.balance(&data.from_user);
    assert_eq!(balance_from_user, 0);

}

#[test]

fn test_not_expired_yet(){
    let data = HTLCContractDataTest::setup();

    let hash = U256::from_be_bytes(&data.env, &Bytes::from_array(&data.env, &data.env.crypto().sha256(&data.bytes).to_array()));
    let id = data.client.create(
        &data.from_user,
        &data.to_user,
        &data.token.0.address,
        &100,
        &50000,
        &hash,
    );


    // withdraw funds from expired contract
    let val = data.client.try_cancel_expired(&id);
    assert_eq!(val, Err(Ok(Error::NotExpiredYet)));

}
