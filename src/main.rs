use std::io;

fn main() {
    exercise8();
}

fn exercise7() {
    let mut v: Vec<String> = Vec::new();
    {
        let chars = [b'x', b'y', b'z'];
        let s: &str = std::str::from_utf8(&chars).unwrap();
        v.push(s.to_owned());
    }
    println!("{:?}", v);
}

fn exercise8() {
    let mut accounting: Vec<String> = vec!["Alice".to_string(), "Ben".to_string()];

    loop {
        let mut add_input = String::from("");

        io::stdin()
            .read_line(&mut add_input)
            .expect("Failed to read line");

        let add_vec: Vec<&str> = add_input.trim()[..].split_whitespace().collect();

        if add_vec.len() < 1 {
            println!("Incorrect input, try again");
            continue;
        }

        let person = add_vec[0];
        accounting.push(person.to_string());
    }
}
