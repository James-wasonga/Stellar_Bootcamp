use std::collections::HashMap;
use std::io;

//Bill Structure
#[derive(Debug, Clone)]
struct Bill {
    name: String,
    amount: f64,
}

//Read user input
fn read_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

//Add bill
fn add_bill(bills: &mut HashMap<u32, Bill>, next_Id: &mut u32) {
    println!("\n Enter bill name:");
    let name = read_input();

    println!("Enter Bill Amount:");
    let amount_input = read_input();

    let amount: f64 = match amount_input.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid amount!");
            return;
        }
    };

    let new_bill = Bill {name, amount};
    bills.insert(*next_Id, new_bill);

    println!("Bill added successfully with ID: {}", next_Id);

    *next_Id += 1;

}

// View bills
fn view_bills(bills: &HashMap<u32, Bill>) {

    if bills.is_empty() {
        println!("\nNo bills found.");
        return;
    }

    println!("\n ========== ALL BILLS =============");

    for (id, bill) in bills {
        println!(
            "ID: {} | Name: {} | Amount: ${:.2}",
            id,
            // bill,
            bill.name,
            bill.amount
        )
    }

    println!("=======================================\n");
 }

// Remove bill
fn remove_bill(bills: &mut HashMap<u32, Bill>) {

    println!("\n Enter bill ID to remove:");

    let id_input = read_input();

    let id: u32 = match id_input.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid ID!");
            return;
        }
    };

    match bills.remove(&id) {
        Some(_) => println!("Bill removed successfully."),
        None => println!("Bill not found."),
    }
}

// Edit bill
fn edit_bill(bills: &mut HashMap<u32, Bill>) {

    println!("\n Enter bill ID to edit:");

    let id_input = read_input();

    let id: u32 = match id_input.parse() {
        Ok(value) => value,
        Err(_) => {
            println!("Invalid ID");
            return;
        }
    };

    if let Some(existing_bill) = bills.get(&id) {

        println!("\nCurrent Bill Details:");
        println!(
            "Name: {} | Amount: ${:.2}",
            existing_bill.name,
            existing_bill.amount
        );

        println!("\nDo you want to continue editing ?");
        println!("Type 'yes' to continue or anything else to cancel.");

        let confirmation = read_input();

        if confirmation.to_lowercase() != "yes" {
            println!("Edit cancelled.");
            return;
        }

        println!("\n Enter new bill name:");
        let new_name = read_input();

        println!("Enter new bill amount:");
        let amount_input = read_input();

        let new_amount: f64 = match amount_input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Invalid amount");
                return;
            }
        };

        let updated_bill = Bill {
            name: new_name,
            amount: new_amount,
        };

        bills.insert(id, updated_bill);

        println!("Bill updated successfully.");
    } else {
        println!("Bill not found.");
    }
}

fn main() {
    let mut bills: HashMap<u32, Bill> = HashMap::new();

    let mut next_Id: u32 = 1;

    loop {
        println!("\n======== BILL MANAGER =========");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Edit Bill");
        println!("5. Exit");
        println!("====================================");
        println!("Enter your choice:");


        let choice = read_input();

        match choice.as_str() {
            "1" => add_bill(&mut bills, &mut next_Id),
            "2" => view_bills(&bills),
            "3" => remove_bill(&mut bills),
            "4" => edit_bill(&mut bills),
            "5" => {
                println!("Existing program...");
                break;
            }

            _=> println!("Invalid choice! Please try again."),

        }
    }
}