mod request;
mod response;

use near_sdk::store::LookupMap;
use near_sdk::{
    borsh::{BorshDeserialize, BorshSerialize},
    collections::UnorderedMap,
    env, ext_contract,
    json_types::U128,
    near_bindgen, require,
    serde::{Deserialize, Serialize},
    AccountId, BorshStorageKey, Gas, NearToken, PanicOnDefault, Promise, PromiseOrValue,
};

use crate::request::*;
use crate::response::*;

const MIN_ATTACHED_GAS: Gas = Gas::from_tgas(90);
const MIN_REMAINING_GAS: Gas = Gas::from_tgas(25);

#[derive(BorshSerialize, BorshStorageKey)]
#[borsh(crate = "near_sdk::borsh")]
enum StorageKey {
    Requests,
    Responses,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
#[borsh(crate = "near_sdk::borsh")]
pub struct Contract {
    requests: UnorderedMap<RequestId, VRequest>,
    responses: LookupMap<RequestId, VResponse>,
    num_requests: u32,
    request_cost: U128,
}

#[ext_contract(ext_self)]
trait ExtContract {
    fn await_response(
        &mut self,
        request_id: RequestId,
        originator_id: AccountId,
    ) -> PromiseOrValue<Response>;
    fn out_of_gas(&mut self);
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(request_cost: U128) -> Self {
        Self {
            requests: UnorderedMap::new(StorageKey::Requests),
            responses: LookupMap::new(StorageKey::Responses),
            num_requests: 0,
            request_cost,
        }
    }

    pub fn get_request_cost(&self) -> U128 {
        self.request_cost
    }

    pub fn get_request(&self, request_id: u32) -> Option<Request> {
        self.requests.get(&request_id).map(|v| v.into())
    }

    #[payable]
    pub fn request(&mut self, request: Request) -> Promise {
        require!(
            env::attached_deposit().as_yoctonear() >= self.request_cost.0,
            "Not enough attached deposit"
        );
        require!(
            remaining_gas() >= MIN_ATTACHED_GAS,
            "Not enough remaining gas to complete the request"
        );

        let request_id = self.num_requests;
        self.requests.insert(&request_id, &(request.into()));
        self.num_requests += 1;
        ext_self::ext(env::current_account_id())
            .await_response(request_id, env::predecessor_account_id())
    }

    #[private]
    pub fn await_response(
        &mut self,
        request_id: u32,
        originator_id: AccountId,
    ) -> PromiseOrValue<Response> {
        let response: Option<Response> = self.responses.get(&request_id).map(|v| v.clone().into());
        if remaining_gas() < MIN_REMAINING_GAS {
            self.requests.remove(&request_id);
            Promise::new(originator_id).transfer(NearToken::from_yoctonear(self.request_cost.0));
            return ext_self::ext(env::current_account_id()).out_of_gas().into();
        }

        if let Some(response) = response {
            self.responses.remove(&request_id);
            PromiseOrValue::Value(response)
        } else {
            ext_self::ext(env::current_account_id())
                .await_response(request_id, originator_id)
                .into()
        }
    }

    #[private]
    pub fn out_of_gas(&mut self) {
        panic!("Out of gas while awaiting response. Deposit refunded.");
    }

    #[private]
    pub fn respond(&mut self, request_id: RequestId, response: Response) {
        if self.requests.remove(&request_id).is_none() {
            panic!("Request ID not found");
        }
        self.responses.insert(request_id, response.into());
    }

    pub fn get_all_requests(&self) -> Vec<(RequestId, Request)> {
        self.requests.iter().map(|(k, v)| (k, v.into())).collect()
    }
}

fn remaining_gas() -> Gas {
    Gas::from_gas(env::prepaid_gas().as_gas() - env::used_gas().as_gas())
}
