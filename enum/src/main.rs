#[derive(Debug)]
enum Years {
    Year2000,
    Year2010
}

enum Coin {
    Ten,
    Five(Option<i32>),
    One(Years)
}

fn main() {

    // Matching Option<T> in match expression
    let first_coin = Coin::Five(Some(1));
    let mr = match first_coin {
        Coin::Ten => 1,
        Coin::Five(Some(1)) => 2,
        Coin::Five(None) => 3,
        _ => 4,
    };
    println!("{}", mr);

    // Patterns that bind to values
    let second_coin = Coin::One(Years::Year2000);
    let mr = match second_coin {
        Coin::One(year) => {
            println!("the coin's year is {:?}", year);
            2000
        },
    _ => 0,
    };

    println!("year is {}", mr)
    
}
