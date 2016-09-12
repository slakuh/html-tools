extern crate clipboard_win;
extern crate regex;
extern crate user;

mod constants;
mod help;
mod replace;

use clipboard_win::{get_clipboard_string, set_clipboard};
use replace::Replace;
use std::env::{self};

fn main() {

    user::user();    

    println!(" \nUse -help: argumnet for more info.\n" );
    let args_string: Vec<String> = env::args()
                                .skip(1)
                                .filter(|x|x.starts_with(constants::ARG_PREFFIX))
                                .collect(); 

    let mut args: Vec<&str> = Vec::new();
    for item in &args_string {
            args.push(&item);
    }

    match get_clipboard_string() {
        Err(e) => println!("{}", e),
        Ok(v) => {                       
            let mut rep = Replace::new(v, args);
            match rep.run_functions() {
                Err(e) => print!("{}", e),
                Ok(_) => {
                    match set_clipboard(&rep.clipboard) {
                        Err(e) => println!("{}", e),
                        Ok(_) => (),
                    };
                }
            }
        },
    };


}
