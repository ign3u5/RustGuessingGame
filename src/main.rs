mod guessing_game;
mod loops;
mod common_programming_concepts;
mod common_collections;
mod structs_with_methods;

fn main() {
    structs_with_methods::get_rectangle_area();
}

fn some_concepts() {
    println!("Starting variables and mutability");
    common_programming_concepts::variables_and_mutability::run();
    println!("Ending variables and mutability");
    println!();
}

fn loops() {
    println!("Starting testing_loops");
    loops::testing_loops();
    println!("Ending testing_loops");
    println!();
}

fn guessing_game() {
    println!("Starting guessing_game");
    guessing_game::play();
    println!("Ending guessing_game");
    println!();
}