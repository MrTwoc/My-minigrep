use std::env;

fn main() {
    let arge :Vec<String> = env::args().collect();

    let query = &arge[1];
    let file_name = &arge[2];
    
    println!("查找内容：{}", query);
    println!("文件名：{}", file_name);
}