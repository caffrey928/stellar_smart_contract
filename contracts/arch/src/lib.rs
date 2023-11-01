#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Vec, vec, Map, I256, map, String};
use soroban_sdk::contracttype;
// use soroban_sdk::testutils::Address as TraitAddress;

const ID: Symbol = symbol_short!("ID");
const USER: Symbol = symbol_short!("USER");
const CLEAR: Symbol = symbol_short!("CLEAR");

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ReturnMessage {
    TRANSACTION(Map<Vec<Symbol>, I256>),
    TRANSACTIONIDS(Vec<Symbol>),
    TRANSACTIONS(Vec<Map<Vec<Symbol>, I256>>),
    USERUSAGE(Map<Symbol, I256>),
    CLEAR(Map<Symbol, I256>),
    ERROR(String),
}

#[contract]
pub struct ArchContract;

#[contractimpl]
impl ArchContract {
    pub fn add_transaction(env: Env, id: Symbol, user: Symbol, telecom_pay: Symbol, telecom_receive: Symbol, usage: i32) -> ReturnMessage {
        let find_transaction: Map<Vec<Symbol>, I256> = env.storage().persistent().get(&id).unwrap_or(Map::new(&env));
        if find_transaction != Map::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "Replicated ID")); // Return from the main function
        }

        let mut transaction_ids: Vec<Symbol> = env.storage().persistent().get(&ID).unwrap_or(Vec::new(&env));
        transaction_ids.push_back(id.clone());

        let transaction: Map<Vec<Symbol>, I256> = map![&env, (vec![&env, user, telecom_pay, telecom_receive], I256::from_i32(&env, usage))];
        // let transaction: Vec<Symbol> = vec![&env, user, telecom_pay, telecom_receive];

        env.storage().persistent().set(&ID, &transaction_ids);
        env.storage().persistent().set(&id, &transaction);

        env.storage().persistent().bump(&ID, 100, 500);
        env.storage().persistent().bump(&id, 100, 500);

        return ReturnMessage::TRANSACTION(transaction);
    }

    pub fn get_transaction_ids(env: Env) -> ReturnMessage {
        let transaction_ids: Vec<Symbol> = env.storage().persistent().get(&ID).unwrap_or(Vec::new(&env));
        if transaction_ids == Vec::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "No Transaction Exist!")); // Return from the main function
        }

        ReturnMessage::TRANSACTIONIDS(transaction_ids)
    }

    pub fn get_transaction(env: Env, id: Symbol) -> ReturnMessage {
        let transaction: Map<Vec<Symbol>, I256> = env.storage().persistent().get(&id).unwrap_or(Map::new(&env));
        if transaction == Map::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "Transaction Not Exist!")); // Return from the main function
        }

        ReturnMessage::TRANSACTION(transaction)
    }

    pub fn get_all_transactions(env: Env) -> ReturnMessage {
        let transaction_ids: Vec<Symbol> = env.storage().persistent().get(&ID).unwrap_or(Vec::new(&env));
        if transaction_ids == Vec::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "No Transaction Exist!")); // Return from the main function
        }

        let mut transactions: Vec<Map<Vec<Symbol>, I256>> = Vec::new(&env);

        for id in transaction_ids.iter() {
            let transaction: Map<Vec<Symbol>, I256> = env.storage().persistent().get(&id).unwrap_or(Map::new(&env));
            transactions.push_back(transaction);
        }

        ReturnMessage::TRANSACTIONS(transactions)
    }

    pub fn remove_transaction(env: Env, id: Symbol) -> ReturnMessage {
        let transaction: Map<Vec<Symbol>, I256> = env.storage().persistent().get(&id).unwrap_or(Map::new(&env));
        if transaction == Map::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "Transaction Not Exist!")); // Return from the main function
        }

        let mut transaction_ids: Vec<Symbol> = env.storage().persistent().get(&ID).unwrap_or(Vec::new(&env));
        
        for (index, transaction_id) in transaction_ids.iter().enumerate() {
            if transaction_id == id {
                transaction_ids.remove(index as u32);
            }
        }

        env.storage().persistent().remove(&id);

        env.storage().persistent().set(&ID, &transaction_ids);
        env.storage().persistent().bump(&ID, 100, 500);

        ReturnMessage::TRANSACTION(transaction)
    }

    pub fn remove_ids(env: Env) -> ReturnMessage {
        let transaction_ids: Vec<Symbol> = env.storage().persistent().get(&ID).unwrap_or(Vec::new(&env));
        if transaction_ids == Vec::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "No Transaction Exist!")); // Return from the main function
        }

        for id in transaction_ids.iter() {
            env.storage().persistent().remove(&id);
        }

        env.storage().persistent().remove(&ID);

        ReturnMessage::TRANSACTIONIDS(transaction_ids)
    }

    pub fn user_usage(env: Env) -> ReturnMessage {
        let fn_result: ReturnMessage = Self::get_all_transactions(env.clone());
        match fn_result {
            ReturnMessage::TRANSACTIONS(transactions) => {
                let mut user_usage_record: Map<Symbol, I256> = env.storage().persistent().get(&USER).unwrap_or(Map::new(&env));

                for transaction in transactions.iter() {
                    let user: Symbol = transaction.keys().get(0).unwrap_or(Vec::new(&env)).get(0).unwrap_or(symbol_short!(""));
                    let usage: I256 = transaction.values().get(0).unwrap_or(I256::from_i32(&env, 0));

                    user_usage_record.set(user.clone(), user_usage_record.get(user.clone()).unwrap_or(I256::from_i32(&env, 0)).add(&usage));
                }

                env.storage().persistent().set(&USER, &user_usage_record);
                env.storage().persistent().bump(&USER, 100, 500);

                ReturnMessage::USERUSAGE(user_usage_record)
            },
            _ => ReturnMessage::ERROR(String::from_slice(&env, "No Transactions Exist!")),
        }
    }

    pub fn get_user_usage(env: Env) -> ReturnMessage {
        let user_usage_record: Map<Symbol, I256> = env.storage().persistent().get(&USER).unwrap_or(Map::new(&env));
        if user_usage_record == Map::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "User_Usage Record Not Exist!")); // Return from the main function
        }

        ReturnMessage::USERUSAGE(user_usage_record)
    }

    pub fn remove_user_usage(env: Env) {
        env.storage().persistent().remove(&USER);
    }

    pub fn clear(env: Env) -> ReturnMessage {
        let fn_result: ReturnMessage = Self::get_all_transactions(env.clone());
        match fn_result {
            ReturnMessage::TRANSACTIONS(transactions) => {
                let mut clear_record: Map<Symbol, I256> = env.storage().persistent().get(&CLEAR).unwrap_or(Map::new(&env));

                for transaction in transactions.iter() {
                    let telecom_pay: Symbol = transaction.keys().get(0).unwrap_or(Vec::new(&env)).get(1).unwrap_or(symbol_short!(""));
                    let telecom_receive: Symbol = transaction.keys().get(0).unwrap_or(Vec::new(&env)).get(2).unwrap_or(symbol_short!(""));
                    let usage: I256 = transaction.values().get(0).unwrap_or(I256::from_i32(&env, 0));
                    let cost: I256 = usage.mul(&I256::from_i32(&env, 100));

                    clear_record.set(telecom_pay.clone(), clear_record.get(telecom_pay.clone()).unwrap_or(I256::from_i32(&env, 0)).sub(&cost));
                    clear_record.set(telecom_receive.clone(), clear_record.get(telecom_receive.clone()).unwrap_or(I256::from_i32(&env, 0)).add(&cost));
                }

                env.storage().persistent().set(&CLEAR, &clear_record);
                env.storage().persistent().bump(&CLEAR, 100, 500);

                Self::remove_ids(env.clone());

                ReturnMessage::CLEAR(clear_record)
            }
            _ => ReturnMessage::ERROR(String::from_slice(&env, "No Transactions Exist!")),
        }
    }

    pub fn get_clear_record(env: Env) -> ReturnMessage {
        let clear_record: Map<Symbol, I256> = env.storage().persistent().get(&CLEAR).unwrap_or(Map::new(&env));
        if clear_record == Map::new(&env) {
            return ReturnMessage::ERROR(String::from_slice(&env, "Clear Record Not Exist!")); // Return from the main function
        }

        ReturnMessage::CLEAR(clear_record)
    }

    pub fn remove_clear_record(env: Env) {
        env.storage().persistent().remove(&CLEAR);
    }

    // pub fn random_address(address: Address) -> Address {
    //     // let address: Address = <Address as TraitAddress>::random(&env);

    //     address
    // }
}

#[cfg(test)]
mod test;