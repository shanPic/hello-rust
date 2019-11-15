fn main() {
	let s = String::from("hello,world");

	// print_string(s);
	// println!("{}", s); // error, String的默认行为为移动，移动进函数后，原变量失效

	print_string(s.clone());
	println!("{}", s);   // correct，使用clone方法，可生成string的拷贝（包括其堆上内容）
}

fn print_string(str : String) {
	println!("{}", str);
}