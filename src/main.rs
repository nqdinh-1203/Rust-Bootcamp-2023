fn main() {
    exercise5();
}

#[derive(Debug, Clone)]
struct Foo {
    str_val: String,
    int_val: i32,
}

fn exercise5() {
    let mut foos = Vec::new();
    foos.push(Foo {
        str_val: "ten".to_string(),
        int_val: 10,
    });
    foos.push(Foo {
        str_val: "twenty".to_string(),
        int_val: 20,
    });

    let moved = foos[0].clone();

    let moved_field = foos[0].str_val.clone();

    println!("moved = {:#?}", moved);
    println!("moved = {}", foos[0].str_val);
}
