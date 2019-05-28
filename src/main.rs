enum Coin {
    ten,
    five(Option<i32>),
}

fn main() {
    let one_coin = Coin::five(Some(1));
    let mr = match one_coin {
        Coin::ten => 1,
        Coin::five(Some(1)) => 2,
        Coin::five(None) => 3,
        _ => 4,
    };
    println!("{}", mr);
    
}
