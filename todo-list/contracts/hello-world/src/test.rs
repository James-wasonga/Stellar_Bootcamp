#![cfg(test)]
use crate::todo::*; 
use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn test_create_todo() {
    let env = Env::default();
    let contract_id = env.register(TodoList, ());
    let client = TodoListClient::new(&env, &contract_id);

    let title = String::from_str(&env, "Stetllar Impact Bootcamp");
    let description = String::from_str(&env, "Soroban Smart Contract");     

    let todo_creation = client.create_todo (&title, &description);
    assert_eq!(todo_creation.title, title);
    assert_eq!( todo_creation.description, description );

    // assert_eq!(
    //     // words,
    //     // vec![
    //     //     &env,
    //     //     String::from_str(&env, "Hello"),
    //     //     String::from_str(&env, "Dev"),
    //     // ]
    // );
}

#[test]
fn test_update_todo() {
    let env = Env::default();
    let contract_id = env.register(TodoList, ());
    let client = TodoListClient::new(&env, &contract_id);

    //first we create_todo
    let title = String::from_str(&env, "Stetllar Impact Bootcamp");
    let description = String::from_str(&env, "Soroban Smart Contract");     

    let todo_creation = client.create_todo (&title, &description);
    
    //update the todo
    let new_title = String::from_str(&env, "New Stellar Impact Bootcamp");
    let new_description = String::from_str(&env, "New Soroban Smart Contract");
    let id = 1;

    let todo_update = client.update_todo(&id, &new_title, &new_description);

    assert!(todo_update);

}