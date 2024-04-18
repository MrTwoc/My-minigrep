use std::env;
use std::fs;

fn main() {
    // 接收输入的参数
    let args :Vec<String> = env::args().collect();
    let config = Config::new(&args);
    let contents = fs::read_to_string(config.file_name).expect("读取文件失败");
    println!("文件内容：\n{}", contents);
}

struct Config{
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Config{
        // query, file_name :  查找内容, 文件名
        let query = args[1].clone();
        let file_name = args[2].clone();
    
        Config{query, file_name}
    }
}
