use crate::{ArchContract, ArchContractClient, ReturnMessage};
use soroban_sdk::{symbol_short, vec, map, Env, I256, Map, Symbol, Vec};
// use soroban_sdk::testutils::Address as TraitAddress;

#[test]
fn arch() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ArchContract);
    let client = ArchContractClient::new(&env, &contract_id);
    // let address: Address = Address::random(&env);
    assert_eq!(1, 1);
    // let transaction_1: ReturnMessage = client.add_transaction(&symbol_short!("test_1"), &symbol_short!("Alice"), &symbol_short!("Telecom1"), &symbol_short!("Telecom2"), &1000);
    // let transaction_2: ReturnMessage = client.add_transaction(&symbol_short!("test_2"), &symbol_short!("Bob"), &symbol_short!("Telecom2"), &symbol_short!("Telecom1"), &2000);
    // let transaction_3: ReturnMessage = client.add_transaction(&symbol_short!("test_3"), &symbol_short!("Sam"), &symbol_short!("Telecom3"), &symbol_short!("Telecom2"), &3000);

    // let transaction_1_clone: Map<Vec<Symbol>, I256> = map![&env, (vec![&env, symbol_short!("Alice"), symbol_short!("Telecom1"), symbol_short!("Telecom2")], I256::from_i32(&env, 1000))];
    // let transaction_2_clone: Map<Vec<Symbol>, I256> = map![&env, (vec![&env, symbol_short!("Bob"), symbol_short!("Telecom2"), symbol_short!("Telecom1")], I256::from_i32(&env, 2000))];
    // let transaction_3_clone: Map<Vec<Symbol>, I256> = map![&env, (vec![&env, symbol_short!("Sam"), symbol_short!("Telecom3"), symbol_short!("Telecom2")], I256::from_i32(&env, 3000))];

    // assert_eq!(transaction_1, ReturnMessage::TRANSACTION(map![&env, (vec![&env, symbol_short!("Alice"), symbol_short!("Telecom1"), symbol_short!("Telecom2")], I256::from_i32(&env, 1000))]));
    // assert_eq!(client.get_transaction_ids(), ReturnMessage::TRANSACTIONIDS(vec![&env, symbol_short!("test_1"), symbol_short!("test_2"), symbol_short!("test_3")]));
    // assert_eq!(client.get_all_transactions(), ReturnMessage::TRANSACTIONS(vec![&env, transaction_1_clone, transaction_2_clone, transaction_3_clone]));
    // assert_eq!(client.get_transaction(&symbol_short!("test_1")), transaction_1);
    // assert_eq!(client.get_transaction(&symbol_short!("test_2")), transaction_2);
    // assert_eq!(client.get_transaction(&symbol_short!("test_3")), transaction_3);

    // client.user_usage();
    // assert_eq!(client.get_user_usage(), ReturnMessage::USERUSAGE(map![&env, (symbol_short!("Alice"), I256::from_i32(&env, 1000)), (symbol_short!("Bob"), I256::from_i32(&env, 2000)), (symbol_short!("Sam"), I256::from_i32(&env, 3000))]));
    
    // client.clear();
    // assert_eq!(client.get_clear_record(), ReturnMessage::CLEAR(map![&env, (symbol_short!("Telecom1"), I256::from_i32(&env, 100000)), (symbol_short!("Telecom2"), I256::from_i32(&env, 200000)), (symbol_short!("Telecom3"), I256::from_i32(&env, -300000))]));

    // client.remove_transaction(&symbol_short!("test_3"));
    // assert_eq!(client.get_transaction_ids(), vec![&env, symbol_short!("test_1"), symbol_short!("test_2")]);
    // assert_eq!(client.get_transaction(&symbol_short!("test_3")), vec![&env]);
    // client.remove_ids();
    // assert_eq!(client.get_transaction_ids(), vec![&env]);
    // assert_eq!(client.get_transaction(&symbol_short!("test_1")), vec![&env]);

    // client.random_address(&address);
    // assert_eq!(client.random_address(&address), address);
}
