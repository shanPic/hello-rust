fn main() {
    // =========================== error begin ========================== //
        // let x = 5; //error! rust的变量默认不可变
    // println!("The value of x is {}", x);
    // x = "correct";
    // println!("The value of x is {}", x);
    // =========================== error end ============================ //

    // =========================== correct begin ========================= //
    let x = 5; //rust的变量默认不可变
    println!("The value of x is {}", x);
    let x = x + 1; // 第二次declare，x的值被shadowing，first value is shadowed by the new value.
    println!("The value of x is {}", x); 
    let x = "correct"; 
    println!("The value of x is {}", x);
    // =========================== correct end =========================== //

    // =========================== correct begin ========================= //
    // let mut x = 5;
    // println!("The value of x is {}", x);
    // x = 6; // 如果声明了mut，但没有改变变量的值，编译器将给出一条 warning
    // println!("The value of x is {}", x);
    // =========================== correct end =========================== //

    // =========================== correct begin ========================= //
    const MAX_CNT : u32 = 100_000;  // 常量，使用const 且必须指定类型
                                    // 数字可以使用"_"分割，以便于阅读
    println!("The MAX_CNT is {}", MAX_CNT);
    // =========================== correct end =========================== //

}
