extern crate regex;
extern crate clipboard_win;

mod help;
mod replace;

use clipboard_win::{get_clipboard_string, set_clipboard};


use replace::Replace;
use std::env;

fn main() {
    println!(" \nUse -help: argumnet for more info.\n" );
    let attr = &env::args().nth(1).unwrap();

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


}
