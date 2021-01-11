use near_sdk::{near_bindgen, env, AccountId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::Vector;
use near_sdk::json_types::U64;
use serde::{Deserialize, Serialize};

#[global_allocator]
static ALLOC: near_sdk::wee_alloc::WeeAlloc = near_sdk::wee_alloc::WeeAlloc::INIT;

static EVENT_MINT: &str = "Mint";
static EVENT_TRANSFER: &str = "Transfer";
static EVENT_TRANSFER_BATCH: &str = "TransferBatch";
static EVENT_APPROVAL: &str = "Approval";
static EVENT_MARKET_DELETE: &str = "MarketDelete";

pub type TokenId = u64;
pub type EditionNumber = u8;
pub type TokenPrice = u128;
pub type CollectionId = u64;
pub type AccountIdHash = Vec<u8>;
pub type Fee = f32;
pub type Allow = bool;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NonFungibleToken {
    pub events: Vector<Event>,
}

impl Default for NonFungibleToken {
    fn default() -> Self {
        panic!("NFT should be initialized before usage")
    }
}

#[near_bindgen]
impl NonFungibleToken {
    #[init]
    pub fn new() -> Self {
        assert!(!env::state_exists(), "Already initialized");
        Self {
            events: Vector::new(b"ev".to_vec()),
        }
    }

    pub fn insert(&mut self) {
        let token_id: TokenId = 19;
        let edition_number: EditionNumber = 91;
        let new_owner_id = "mike.near".to_string();

        self.events.push(&Event::new(
            EVENT_MARKET_DELETE.to_string(),
            env::predecessor_account_id(),
            env::predecessor_account_id(),
            env::predecessor_account_id(),
            token_id,
            edition_number,
            0
        ));
        self.events.push(
            &Event::new(EVENT_TRANSFER.to_string(),
            env::predecessor_account_id(),
             env::predecessor_account_id(),
            new_owner_id.to_string(),
            token_id,
            edition_number,
            0
        ));
        self.events.push(
            &Event::new(EVENT_TRANSFER_BATCH.to_string(),
            env::predecessor_account_id(),
             env::predecessor_account_id(),
            new_owner_id.to_string(),
            token_id,
            edition_number,
            0
        ));
        self.events.push(
            &Event::new(EVENT_APPROVAL.to_string(),
            env::predecessor_account_id(),
             env::predecessor_account_id(),
            new_owner_id.to_string(),
            token_id,
            edition_number,
            0
        ));
        self.events.push(
            &Event::new(EVENT_MINT.to_string(),
            env::predecessor_account_id(),
             env::predecessor_account_id(),
            new_owner_id.to_string(),
            token_id,
            edition_number,
            0
        ));
    }

    pub fn get_index(&self, idx: U64) -> Event {
        self.events.get(idx.into()).unwrap()
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Serialize, Deserialize, Debug)]
pub struct Event {
    block: String,
    date: String,
    event: Vec<String>,
}

impl Event {
    pub fn new(event_name: String, sender: AccountId, from: AccountId, to: AccountId, token_id: TokenId, edition_id: EditionNumber, price: TokenPrice) -> Event {
        let block = env::block_index().to_string();
        let date = env::block_timestamp().to_string();
        let mut event = Vec::new();
        event.push(event_name);
        event.push(sender);
        event.push(from);
        event.push(to);
        event.push(token_id.to_string());
        event.push(edition_id.to_string());
        event.push(price.to_string());
        Event {
            block,
            date,
            event,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext, AccountId};

    fn get_context(input: Vec<u8>, is_view: bool, predecessor: AccountId) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "mike.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: predecessor,
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    #[test]
    fn test_hashing() {
        let context = get_context(vec![], false, "robert.testnet".to_string());
        testing_env!(context);

        let mut contract = NonFungibleToken::new();
        contract.insert();
        let first = contract.get_index(U64(0));
        println!("first one is {:?}", first);
        let second = contract.get_index(U64(1));
        println!("second one is {:?}", second);
        let third = contract.get_index(U64(2));
        println!("third one is {:?}", third);
        let forth = contract.get_index(U64(3));
        println!("forth one is {:?}", forth);
        let fifth = contract.get_index(U64(4));
        println!("fifth one is {:?}", fifth);
    }
}

