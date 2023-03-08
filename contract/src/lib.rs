// Find all our documentation at https://docs.near.org
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, log, near_bindgen, AccountId, Gas, Promise, PromiseError};

#[ext_contract(contract_near1)]
trait ExtNear1 {
    fn get_greeting(&self) -> String;
    fn get_address(&self) -> String;
}
const GAS: Gas = Gas(5_000_000_000_000);
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    account_id: AccountId,
}

// Define the default, which automatically initializes the contract
impl Default for Contract {
    fn default() -> Self {
        Self {
            account_id: AccountId::new_unchecked("near1.tranchinhwalletnear.testnet".to_string()),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn call_all_fun(&self) {
        let promise1 = Promise::new(self.account_id.clone()).function_call(
            "get_greeting".to_string(),
            b"{}".to_vec(),
            0,
            GAS,
        );
        log!("gas_dasudungtrongmain1: {:?}", env::used_gas());

        let promise2 = Promise::new(self.account_id.clone()).function_call(
            "get_address".to_string(),
            b"{}".to_vec(),
            0,
            GAS,
        );
        log!("gas_dasudungtrongmain2: {:?}", env::used_gas());

        promise1.and(promise2).then(
            Self::ext(env::current_account_id())
                .with_static_gas(GAS)
                .execute_callback(),
        );
        log!("gas_dasudungtrongmain: {:?}", env::used_gas());
    }
    pub fn get_gas(&self) -> Gas {
        log!("gas: {:?}", env::prepaid_gas());
        return env::prepaid_gas();
    }
    pub fn execute_callback(
        &mut self,
        #[callback_result] value1: Result<String, PromiseError>,
        #[callback_result] value2: Result<String, PromiseError>,
    ) {
        let value1 = value1.unwrap_or("Loi".to_string());
        let value2 = value2.unwrap_or("Loi".to_string());
        log!("gas_dasudung: {:?}", env::used_gas());
        log!("gas_conlai: {:?}", env::prepaid_gas());
        log!("{} que o {}", value1, value2)
    }
}
