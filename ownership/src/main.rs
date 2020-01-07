fn main() {

    // correct
    let mut str = String::from("string");
    change(&mut str);
    println!("{}", str);
    
    let slice = first_word(&str);
    println!("{}", slice);
}

fn change(str : &mut String) -> usize {
    str.push_str("_push_str");
    return str.len();
}

fn first_word(str : &str) -> &str {
    return str;
}