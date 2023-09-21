mod binary_reader;
use std::env;
use std::process;

fn main(){
    //Usage-
    //compile filename.mixal: Compiles a MIXAL file into a binary 
        //.mix file which can be executed by the mix vm
    //run filename.mix: Runs the mix file
    //
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        process::exit(1);
    });

    run(config);
}


fn run(config: Config){
    //TODO: Add file check

    println!("{}", config.filename);
    if config.query == "compile" {
    }
    
    else if config.query == "run"{
    }

    else{
        panic!("Error in file handling");
    }
}

struct Config{
    query: String,
    filename: String,
}


impl Config{
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 3{
            return Err("Wrong number of command line arguments. usage: 'compile file.mixal', or 'run file.mix' ")
        }
        if args[1] != "compile" && args[1] != "run"{
            return Err("Usage: run the assembler by calling 'compile filename.mixal', or run a mix file with 'run filename.mix'")
        }
        if args[1] == "compile"{
            let last_six_chars = &args[2][args[2].len() - 6..];
            if last_six_chars != ".mixal"{
                println!("{}", last_six_chars);
                return Err("Please provide a .mixal file to assemble");
            }
        }

        if args[1] == "run"{
            let last_four_chars = &args[2][args[2].len() - 7..];
            if last_four_chars != ".mixexe"{
                return Err("MIX VM can only execute .mixexe files");
            }
        }
        
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config{ query, filename})
    }
}
