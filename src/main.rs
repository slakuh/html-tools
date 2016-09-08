extern crate regex;
extern crate clipboard_win;

mod help;
mod replace;

use clipboard_win::{get_clipboard_string, set_clipboard};
use replace::Replace;
use std::env::{self};

fn main() {
    // |fn|qrg1:arg2:arg3
    println!(" \nUse -help: argumnet for more info.\n" );
    let args: Vec<String> = env::args().skip(1).filter(|x|x.starts_with("-")).collect(); 
    for argument in args {
        println!("{}", argument);
    }


/*    
    match get_clipboard_string() {
        Err(e) => println!("{}", e),
        Ok(v) => {                       
            let mut rep = Replace::new(v,attr);
            match rep.run_functions() {
                Err(e) => print!("{}", e),
                Ok(_) => {
                    match set_clipboard(&rep.html) {
                        Err(e) => println!("{}", e),
                        Ok(_) => (),
                    };
                }
            }
        },
    };
*/

}
