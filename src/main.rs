use std::env;
use std::fs;

fn main() {
    // 接收输入的参数
    let arge :Vec<String> = env::args().collect();
    
    // query, file_name :  查找内容, 文件名
    let query = &arge[1];
    let file_name = &arge[2];
    
    // println!("查找内容：{}", query);
    // println!("文件名：{}", file_name);

    let contents = fs::read_to_string(file_name)
    .expect("读取文件失败");

    println!("文件内容：\n{}", contents);
}