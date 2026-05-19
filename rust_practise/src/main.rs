mod vector;
use crate::vector::use_vector;

fn sum (a: u32, b: u32) -> u32 {
   let mut sum1 = a + b;
   sum1 = sum1 + 1;
   sum1
}

fn subtract (a: u32, b: u32) -> u32 {
    let shud = a - b;
    let final_shud = shud - 2;
    return final_shud
}

fn multiplication (x: u32, y: u32) -> u32 {
    return x * y
}

fn division (x: u32, y: u32) -> u32 {
    return x / y
}

// fn for if else
fn control_flow() {
    let age = 20;
    if age > 30 {
        println!("You are Old");
    } else {
        println!("You are completely young");
    }
}

// fn for match
fn use_match() {
    let weight = true;
    match weight {
        true => println!("True"),
        false => println!("False"), 
        _=> println!("No Value"),
    }
}

// fn for struct
fn use_struct() {
    struct Person {
        name: String,
        email: String,
        age: i32,
        place: String,
    }

    // let person1 = Person{ name: String::from("james"), email: String::from("james@gmail.com"), age: 20, place: String::from("Kisumu")};c
    let person1 = Person{ name: "james".to_string(), email: "james@gmail.com".to_string(), age: 20, place: "Kisumu".to_string()};
    println!("The name is {}", person1.name);
}


// fn for vector


fn main() {
    let add = sum(4, 5);
    println!("The add answer is {}", add);
    let subt = subtract(8,2);
    println!("The Sub answer is {}", subt);
    let mult = multiplication(2,3);
    println!("The Mult answer is {}", mult);
    let div = division(16,2);
    println!("The Div answer is {}", div);

    // loop{
    //     println!("again");
    // }

    // fn for control_flow
    control_flow();

    // fn for match
    use_match();

    //fn for struct
    use_struct();

    use_vector();
}
