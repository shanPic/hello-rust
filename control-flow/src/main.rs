fn main() {

    let mut loop_num = 0;

    let x = loop {
        loop_num = loop_num + 1;

        if loop_num == 10 {
            break loop_num * 2;
        };
    };

    assert_eq!(x, 20);
    println!("{}", x);
}
