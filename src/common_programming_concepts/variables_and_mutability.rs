const MAX_POINTS: u32 = 100_000; //Number can be split up using _ for readability

pub fn run() {
    println!("The value of MAX_POINTS is {}", MAX_POINTS);
    println!("The value of x for normal mut is {}", normal_mut());
    println!("The value of x for shadowing is {}", shadowing());
}

fn normal_mut() -> i32 {
    let mut x = 5;
    x = 6;
    x
}

fn shadowing() -> i32{
    let x = 5;
    let x = 6;
    x
}