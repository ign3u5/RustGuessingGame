use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Starting testing_loops");
    testing_loops();
    println!("Ending testing_loops");

    println!("Starting guessing_game");
    guessing_game();
    println!("Ending guessing_game");
}

//Testing loops
fn testing_loops() {
    let mut counter = 0;
    let a = [10, 20, 30, 40, 50];

    //Standard loop - Expression
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; //Semicolon doesn't matter here
        }
    };

    //While loop - Statement
    while counter < 30 {
        counter += 1;
    };
    
    //For loop - Statement
    for element in a.iter() {
        println!("The value is: {}", element);
    }

    println!("The result is: {}", result);
}

//Guessing game
fn guessing_game() {
    println!("{}", welcome_message_builder("Chris"));
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You entered an invalid number!");
                continue;
            },
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

fn welcome_message_builder(input: &str) -> String {
    let mut welcome_message = String::from("Hello ");
    welcome_message.push_str(input);
    welcome_message
}
