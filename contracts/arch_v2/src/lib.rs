#![no_std]
use soroban_sdk::contracttype;
use soroban_sdk::{
    contract, contractimpl, map, symbol_short, vec, Address, Env, Map, String, Symbol, Vec, I256,
};
// use soroban_sdk::testutils::Address as TraitAddress;

// const ID: Symbol = symbol_short!("ID");
// const USER: Symbol = symbol_short!("USER");
// const CLEAR: Symbol = symbol_short!("CLEAR");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReturnMessage {
    TRANSACTION(Map<Vec<Symbol>, I256>),
    TRANSACTIONIDS(Vec<Symbol>),
    TRANSACTIONS(Vec<Map<Vec<Symbol>, I256>>),
    USERUSAGE(Map<Symbol, I256>),
    CLEAR(Map<Symbol, I256>),
    ERROR(String),
    ACCOUNT(Vec<Address>),
}

#[contracttype]
pub enum DataKey {
    RECORD(Address),
    USERUSAGE(Address),
    CLEAR(Address),
    ACCOUNT(Address),
    FULLCLEAR(Address),
}

#[contract]
pub struct ArchContract;

#[contractimpl]
impl ArchContract {
    pub fn add_transaction(
        env: Env,
        auth: Address,
        user: Symbol,
        telecom_pay: Symbol,
        telecom_receive: Symbol,
        usage: i32,
    ) -> ReturnMessage {
        auth.require_auth();

        let key = DataKey::RECORD(auth.clone());
        let mut telecom_storage: Vec<Map<Vec<Symbol>, I256>> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        let transaction: Map<Vec<Symbol>, I256> = map![
            &env,
            (
                vec![&env, user, telecom_pay, telecom_receive],
                I256::from_i32(&env, usage)
            )
        ];
        telecom_storage.push_back(transaction.clone());

        env.storage().persistent().set(&key, &telecom_storage);

        return ReturnMessage::TRANSACTION(transaction);
    }

    pub fn get_all_transactions(env: Env, auth: Address) -> ReturnMessage {
        let key = DataKey::RECORD(auth.clone());
        let telecom_storage: Vec<Map<Vec<Symbol>, I256>> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));
        if telecom_storage.len() == 0 {
            return ReturnMessage::ERROR(String::from_slice(
                &env,
                "The Account Has No Transaction Exist!",
            )); // Return from the main function
        }

        ReturnMessage::TRANSACTIONS(telecom_storage)
    }

    pub fn remove_transactions(env: Env, auth: Address) -> ReturnMessage {
        auth.require_auth();

        let key = DataKey::RECORD(auth.clone());
        let telecom_storage: Vec<Map<Vec<Symbol>, I256>> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));
        if telecom_storage.len() == 0 {
            return ReturnMessage::ERROR(String::from_slice(
                &env,
                "The Account Has No Transaction Exist!",
            )); // Return from the main function
        }

        env.storage().persistent().remove(&key);

        ReturnMessage::TRANSACTIONS(telecom_storage)
    }

    pub fn user_usage(env: Env, auth: Address) -> ReturnMessage {
        auth.require_auth();

        let fn_result: ReturnMessage = Self::get_all_transactions(env.clone(), auth.clone());
        match fn_result {
            ReturnMessage::TRANSACTIONS(transactions) => {
                let key = DataKey::USERUSAGE(auth.clone());
                let mut user_usage_record: Map<Symbol, I256> = env
                    .storage()
                    .persistent()
                    .get(&key)
                    .unwrap_or(Map::new(&env));

                for transaction in transactions.iter() {
                    let user: Symbol = transaction
                        .keys()
                        .get(0)
                        .unwrap_or(Vec::new(&env))
                        .get(0)
                        .unwrap_or(symbol_short!(""));
                    let usage: I256 = transaction
                        .values()
                        .get(0)
                        .unwrap_or(I256::from_i32(&env, 0));

                    user_usage_record.set(
                        user.clone(),
                        user_usage_record
                            .get(user.clone())
                            .unwrap_or(I256::from_i32(&env, 0))
                            .add(&usage),
                    );
                }

                env.storage().persistent().set(&key, &user_usage_record);

                ReturnMessage::USERUSAGE(user_usage_record)
            }
            _ => ReturnMessage::ERROR(String::from_slice(
                &env,
                "The Account Has No Transaction Exist!",
            )),
        }
    }

    pub fn get_user_usage(env: Env, auth: Address) -> ReturnMessage {
        auth.require_auth();

        let key = DataKey::USERUSAGE(auth.clone());
        let user_usage_record: Map<Symbol, I256> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Map::new(&env));
        if user_usage_record == Map::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "User Usage Record Not Exist!"));
            // Return from the main function
        }

        ReturnMessage::USERUSAGE(user_usage_record)
    }

    pub fn remove_user_usage(env: Env, auth: Address) {
        auth.require_auth();
        let key = DataKey::USERUSAGE(auth.clone());
        env.storage().persistent().remove(&key);
    }

    pub fn clear(env: Env, auth: Address) -> ReturnMessage {
        auth.require_auth();

        let fn_result: ReturnMessage = Self::get_all_transactions(env.clone(), auth.clone());
        match fn_result {
            ReturnMessage::TRANSACTIONS(transactions) => {
                let key = DataKey::CLEAR(auth.clone());
                let mut clear_record: Map<Symbol, I256> = env
                    .storage()
                    .persistent()
                    .get(&key)
                    .unwrap_or(Map::new(&env));

                for transaction in transactions.iter() {
                    let telecom_pay: Symbol = transaction
                        .keys()
                        .get(0)
                        .unwrap_or(Vec::new(&env))
                        .get(1)
                        .unwrap_or(symbol_short!(""));
                    let telecom_receive: Symbol = transaction
                        .keys()
                        .get(0)
                        .unwrap_or(Vec::new(&env))
                        .get(2)
                        .unwrap_or(symbol_short!(""));
                    let usage: I256 = transaction
                        .values()
                        .get(0)
                        .unwrap_or(I256::from_i32(&env, 0));
                    let cost: I256 = usage.mul(&I256::from_i32(&env, 100));

                    clear_record.set(
                        telecom_pay.clone(),
                        clear_record
                            .get(telecom_pay.clone())
                            .unwrap_or(I256::from_i32(&env, 0))
                            .sub(&cost),
                    );
                    clear_record.set(
                        telecom_receive.clone(),
                        clear_record
                            .get(telecom_receive.clone())
                            .unwrap_or(I256::from_i32(&env, 0))
                            .add(&cost),
                    );
                }

                env.storage().persistent().set(&key, &clear_record);

                ReturnMessage::CLEAR(clear_record)
            }
            _ => ReturnMessage::ERROR(String::from_slice(&env, "The Account Has No Transaction Exist!")),
        }
    }

    pub fn get_clear_record(env: Env, auth: Address) -> ReturnMessage {
        let key = DataKey::CLEAR(auth.clone());
        let clear_record: Map<Symbol, I256> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Map::new(&env));
        if clear_record == Map::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "Clear Record Not Exist!"));
            // Return from the main function
        }

        ReturnMessage::CLEAR(clear_record)
    }

    pub fn remove_clear_record(env: Env, auth: Address) {
        auth.require_auth();
        let key = DataKey::CLEAR(auth.clone());
        env.storage().persistent().remove(&key);
    }

    pub fn full_clear_record(env: Env, neutral_auth: Address) -> ReturnMessage {
        neutral_auth.require_auth();

        let account_key = DataKey::ACCOUNT(neutral_auth.clone());
        let accounts: Vec<Address> = env.storage().persistent().get(&account_key).unwrap_or(Vec::new(&env));

        let mut full_clear: Map<Symbol, I256> = Map::new(&env);

        for account in accounts.iter() {
            let clear_fn_result: ReturnMessage = Self::get_clear_record(env.clone(), account.clone());

            match clear_fn_result {
                ReturnMessage::CLEAR(clear_record) => {
                    for (telecom, cost) in clear_record.iter() {
                        full_clear.set(
                        telecom.clone(),
                        full_clear
                            .get(telecom.clone())
                            .unwrap_or(I256::from_i32(&env, 0))
                            .add(&cost),
                        );
                    }
                },
                _ => continue,
            }
        }
        
        let clear_key = DataKey::FULLCLEAR(neutral_auth.clone());
        env.storage().persistent().set(&clear_key, &full_clear);

        ReturnMessage::CLEAR(full_clear)
    }

    pub fn add_account(env: Env, neutral_auth: Address, account: Address) -> ReturnMessage {
        neutral_auth.require_auth();

        let key = DataKey::ACCOUNT(neutral_auth.clone());
        let mut accounts: Vec<Address> = env.storage().persistent().get(&key).unwrap_or(Vec::new(&env));
        accounts.push_back(account.clone());

        env.storage().persistent().set(&key, &accounts);

        ReturnMessage::ACCOUNT(accounts)
    }

    pub fn remove_all(env: Env, auth: Address) {
        auth.require_auth();
        Self::remove_user_usage(env.clone(), auth.clone());
        Self::remove_clear_record(env.clone(), auth.clone());
        Self::remove_transactions(env.clone(), auth.clone());
    }
}

#[cfg(test)]
mod test;
