use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_name)?;
    for line in search(&config.query, &contents){
        println!("{}", line);
    }
    // println!("文件内容：\n{}", contents);
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

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();

    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn on_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}