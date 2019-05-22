fn main() {
    // let x = 5; //error! rust的变量默认不可变
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 10;
    println!("The value of x is {}", x);
}
