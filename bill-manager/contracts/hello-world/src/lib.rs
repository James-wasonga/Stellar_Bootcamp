#![no_std]

use soroban_sdk::{
    contract,
    contractimpl,
    contracttype,
    symbol_short,
    Env,
    Symbol,
    Vec,
    Map,
    String,
};

#[derive(Clone)]
#[contracttype]
pub struct Bill {
    pub id: u32,
    pub name: String,
    pub amount: i128,
}

const BILL_COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct BillManagerContract;

#[contractimpl]
impl BillManagerContract {

    // Add Bill
    pub fn add_bill(
        env: Env,
        name: String,
        amount: i128,
    ) -> u32 {

        let mut count: u32 = env
            .storage()
            .instance()
            .get(&BILL_COUNT)
            .unwrap_or(0);

        count += 1;

        let bill = Bill {
            id: count,
            name,
            amount,
        };

        env.storage()
            .instance()
            .set(&count, &bill);

        env.storage()
            .instance()
            .set(&BILL_COUNT, &count);

        count
    }

    // Get Single Bill
    pub fn get_bill(env: Env, id: u32) -> Bill {
        env.storage()
            .instance()
            .get(&id)
            .unwrap()
    }

    // Remove Bill
    pub fn remove_bill(env: Env, id: u32) {
        env.storage()
            .instance()
            .remove(&id);
    }

    // Update/Edit Bill
    pub fn update_bill(
        env: Env,
        id: u32,
        name: String,
        amount: i128,
    ) {

        let exists: bool = env
            .storage()
            .instance()
            .has(&id);

        if !exists {
            panic!("Bill does not exist");
        }

        let updated_bill = Bill {
            id,
            name,
            amount,
        };

        env.storage()
            .instance()
            .set(&id, &updated_bill);
    }

    // Get All Bills
    pub fn get_all_bills(env: Env) -> Vec<Bill> {

        let count: u32 = env
            .storage()
            .instance()
            .get(&BILL_COUNT)
            .unwrap_or(0);

        let mut bills = Vec::new(&env);

        let mut i: u32 = 1;

        while i <= count {

            if env.storage().instance().has(&i) {

                let bill: Bill = env
                    .storage()
                    .instance()
                    .get(&i)
                    .unwrap();

                bills.push_back(bill);
            }

            i += 1;
        }

        bills
    }
}