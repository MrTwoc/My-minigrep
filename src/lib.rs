use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;
    println!("文件内容：\n{}", contents);
    Ok(())
}

pub struct Config{
    query: String,
    file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str>{
        if args.len() < 3 {
            return Err("参数不足");
        }
        // query, file_name :  查找内容, 文件名
        let query = args[1].clone();
        let file_name = args[2].clone();
    
        Ok(Config{query, file_name})
    }
}