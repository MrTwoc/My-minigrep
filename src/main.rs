use std::env;
use std::process;
use minigrep::Config;

fn main() {
    // 接收输入的参数
    let args :Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("错误：{}", err);
        process::exit(1);
    });
    
    if let Err(e)= minigrep::run(config){
        eprintln!("应用错误：{}", e);
        process::exit(1);
    }
    
}


