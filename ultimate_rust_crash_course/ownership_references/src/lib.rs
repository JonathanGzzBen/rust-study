pub fn inspect(string: &String) {
    if string.ends_with("s") {
        println!("{} is plural", string);
    } else {
        println!("{} is singular", string);
    }
}

pub fn change(string: &mut String) {
    if !string.ends_with("s") {
        string.push_str("s")
    }
}

pub fn eat(string: String) -> bool {
    string.starts_with("b") && string.contains("a")
}

pub fn bedazzle(string: &mut String) {
    *string = "sparkly".to_string();
}
