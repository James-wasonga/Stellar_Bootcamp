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

#[test]
fn test_get_todos() {
    let env = Env::default();
    let contract_id = env.register(TodoList, ());
    let client = TodoListClient::new(&env, &contract_id);

    //first we create_todo
    let title = String::from_str(&env, "Stetllar Impact Bootcamp");
    let description = String::from_str(&env, "Soroban Smart Contract");

    let create_todo = client.create_todo (&title, &description);

    let todos = client.get_todos();
    assert_eq!(todos.len(), 1);
}

#[test]
fn test_mark_completed() {
    let env = Env::default();
    let contract_id = env.register(TodoList, ());
    let client = TodoListClient::new(&env, &contract_id);

    //first we create_todo
    let title = String::from_str(&env, "Stetllar Impact Bootcamp");
    let description = String::from_str(&env, "Soroban Smart Contract");

    let create_todo = client.create_todo (&title, &description);

    let id = 1;
    let mark_completed = client.mark_completed(&id);
    assert!(mark_completed);

}

#[test]
fn test_delete_todo() {
    let env = Env::default();
    let contract_id = env.register(TodoList, ());
    let client = TodoListClient::new(&env, &contract_id);

    //first we create_todo
    let title = String::from_str(&env, "Stetllar Impact Bootcamp");
    let description = String::from_str(&env, "Soroban Smart Contract");

    let create_todo = client.create_todo (&title, &description);

    let id = 1;
    let delete_todo = client.delete_todo(&id);
    assert!(delete_todo);
}
