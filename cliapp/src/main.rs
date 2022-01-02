use std::env;
//use std::fs;
use std::process;
//use std::error::Error;
use cliapp::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    // let query:&String = &args[1];
    // let filename:&String = &args[2];
  

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing {}",err);
        process::exit(1);
    });

    if let  Err(e) = cliapp::run(config) {
        println!("Applicaiton error: {}", e);
        process::exit(1);
    }

  
}


/// Test
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn result() {
        let query = "fgf";
        let contents = "\
kskk:
ksk, kks, kkksk";

        assert_eq!(vec!["hjaksd, ksk, kss"], search(query, contents));
    }
}

