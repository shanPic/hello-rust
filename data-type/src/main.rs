fn main() {
    let tuple_one : (i8) = (0);
    let tuple_two : (i8, i16) = (0, 1);
    let tuple_three : (i32, u32, i128) = (2, 3, 4);
    let tuple_four = (1, 2, 3, 4); // tuple: 可容纳多个不同的数据类型

    let (x, y, z) = tuple_three; 
    let (_, x, _) = tuple_three; // tuple的解构（destruct）

    println!("in tupel_three, first element is: {}, second: {}, thrid: {}", tuple_three.0, tuple_three.1, tuple_three.2);

    let array : [i32; 3] = [1, 2, 3];
}
