enum Coin {
    Ten,
    Five(Option<i32>),
}

fn main() {
    let one_coin = Coin::Five(Some(1));
    let mr = match one_coin {
        Coin::Ten => 1,
        Coin::Five(Some(1)) => 2,
        Coin::Five(None) => 3,
        _ => 4,
    };
    println!("{}", mr);
    
}
