use crate::{ArchContract, ArchContractClient};
use soroban_sdk::{symbol_short, vec, map, Env, I256};

#[test]
fn arch() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ArchContract);
    let client = ArchContractClient::new(&env, &contract_id);

    let transaction_1 = client.add_transaction(&symbol_short!("test_1"), &symbol_short!("Alice"), &symbol_short!("Telecom1"), &symbol_short!("Telecom2"), &1000);
    let transaction_2 = client.add_transaction(&symbol_short!("test_2"), &symbol_short!("Bob"), &symbol_short!("Telecom2"), &symbol_short!("Telecom1"), &2000);
    let transaction_3 = client.add_transaction(&symbol_short!("test_3"), &symbol_short!("Sam"), &symbol_short!("Telecom3"), &symbol_short!("Telecom2"), &3000);

    assert_eq!(client.get_transaction_ids(), vec![&env, symbol_short!("test_1"), symbol_short!("test_2"), symbol_short!("test_3")]);
    assert_eq!(client.get_all_transactions(), vec![&env, transaction_1.clone(), transaction_2.clone(), transaction_3.clone()]);
    assert_eq!(client.get_transaction(&symbol_short!("test_1")), transaction_1);
    assert_eq!(client.get_transaction(&symbol_short!("test_2")), transaction_2);
    assert_eq!(client.get_transaction(&symbol_short!("test_3")), transaction_3);

    client.user_usage();
    assert_eq!(client.get_user_usage(), map![&env, (symbol_short!("Alice"), I256::from_i32(&env, 1000)), (symbol_short!("Bob"), I256::from_i32(&env, 2000)), (symbol_short!("Sam"), I256::from_i32(&env, 3000))]);
    
    client.clear();
    assert_eq!(client.get_clear_record(), map![&env, (symbol_short!("Telecom1"), I256::from_i32(&env, 100000)), (symbol_short!("Telecom2"), I256::from_i32(&env, 200000)), (symbol_short!("Telecom3"), I256::from_i32(&env, -300000))]);

    // client.remove_transaction(&symbol_short!("test_3"));
    // assert_eq!(client.get_transaction_ids(), vec![&env, symbol_short!("test_1"), symbol_short!("test_2")]);
    // assert_eq!(client.get_transaction(&symbol_short!("test_3")), vec![&env]);
    // client.remove_ids();
    // assert_eq!(client.get_transaction_ids(), vec![&env]);
    // assert_eq!(client.get_transaction(&symbol_short!("test_1")), vec![&env]);
}
