struct Student {
    name: String,
    age: u32
}

impl Student {
    fn new() -> Self {
        Self { name: "Dinh".to_string(), age: 18 }
    }
}

fn main() {
    let s1 = Student::new();
 
    let s2 = &s1.name;

    println!("{}", s1.name);
    println!("{}", s2);
    println!("{}", s1.age);

}
