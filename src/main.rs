struct Person {
    name: String,
    age: u8,
    height: i32,
    weight: i32,
}

fn main() {
    let _person = Person {
        name: String::from("Alice"),
        age: 30,
        height: 20,
        weight: 20,
    };
    println!("{} is {} years old", _person.name, _person.age);
    println!("{} has area {}", _person.name, _person.area())
}

impl Person {
    fn area(&self) -> i32 {
        return self.height * self.weight;
    }
}
