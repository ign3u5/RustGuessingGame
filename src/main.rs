mod guessing_game;
mod loops;

fn main() {
    println!("Starting testing_loops");
    loops::testing_loops();
    println!("Ending testing_loops");
    println!();
    
    println!("Starting guessing_game");
    guessing_game::play();
    println!("Ending guessing_game");
    println!();
}