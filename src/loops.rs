pub fn testing_loops() {
    let arr = [10, 20, 30, 40, 50];
    
    for_loop(&arr);
    while_loop();
    standard_loop();
}

fn for_loop (arr: &[i32]) {
    let mut counter = 0;

    for number in arr.iter() {
        println!("The value is {}", number);
        counter += 1;
    }

    println!("Result of for loop is {}", counter);
}

fn while_loop() {
    let mut counter = 0;

    while counter < 30 { //Statement type as it does NOT return a value
        counter += 1;
    }

    println!("Result of while loop is {}", counter);
}

fn standard_loop() { 
    let mut counter = 0;

    let result = loop { //Expression type as it returns a value
        counter += 1;

        if counter == 10 {
            break counter * 2; //Semicolon doesn't matter here
        }
    };

    println!("Result of standard loop is {}", result);
}