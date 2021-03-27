pub fn run() {
    vectors();
    updating_a_vector();
    getting_elements();
    altering_elements();
    vector_of_different_types();
}

fn vectors() {
    let newVector: Vec<i32> = Vec::new();

    let initialisedVector = vec![1, 2, 3];
}

fn updating_a_vector() {
    let mut newMutableVector = Vec::new();

    //notice that the vector type doesn't need to be specified
    //because it can be infered from what we are pushing.
    newMutableVector.push(1);
    newMutableVector.push(2);
    newMutableVector.push(3);
}

fn getting_elements() {
    let v = vec![1, 2, 3, 4];

    let third = &v[2];
    println!("The thid element is {}", third);

    match v.get(2) {
        Some(last) => println!("The thid element is {}", last),
        None => println!("There is no third element"),
    }
}

fn altering_elements() {
    let mut v = vec![100, 75, 29];
    for i in &mut v {
        *i += 50;
    }

    println!("The array now looks like {:#?}", v); //To print inline us {:?}
}

#[derive(Debug)]
enum VTypes {
    Int(i32),
    Float(f64),
    String(String),
}

fn vector_of_different_types() {
    let v = vec![
        VTypes::Int(5),
        VTypes::String(String::from("Hello")),
        VTypes::Int(7),
    ];

    println!("The V Types vector looks like {:#?}", v);
}