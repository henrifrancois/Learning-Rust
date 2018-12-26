use std::env;
use std::process;

use rtgrep;
use rtgrep::Config;


fn main() {
   let args: Vec<String> = env::args().collect();

   let config = Config::new(&args).unwrap_or_else(|err| {
       eprintln!("error parsing arguments: {}", err);
       process::exit(1);
   });


   if let Err(e) = rtgrep::run(config){
     eprintln!("Application Error: {}", e);
     process::exit(1);
   };
}
