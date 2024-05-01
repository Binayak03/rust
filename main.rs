struct Person {
    name: String,
    age: u32,
}

fn main() {
    let binayak = Person {
        name: String::from("Bhattarai"), // Corrected capitalization of String
        age: 21,
    };
    
    println!("The name is {}", binayak.name);
    println!("The age is {}", binayak.age);
}
