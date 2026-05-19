pub fn use_vector() {
    let mut numbers = vec![1,2,3];
    numbers.push(4);
    numbers.len();

    println!("The vector is {:?}", numbers);

    let mut numbers2 = Vec::new();
    numbers2.push(1);
    numbers2.push(2);
    numbers2.push(4);
    numbers2.push(3);
    numbers2.pop();

    println!("The vector is {:?}", numbers2);

}


// println!("The vector is {}")