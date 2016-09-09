use help;
use regex::{Error, Regex};

#[derive(Debug)]
pub struct Replace<'a> {
    pub html: String,
    args: Vec<(&'a str, Vec<&'a str>)>,
}

impl<'a> Replace<'a> {
    pub fn new(html: String, argument: Vec<&'a str>) -> Replace<'a> {
        Replace {
            html: html,
            args: Replace::parse_arguments(argument),
        }
    } 
    
    fn parse_arguments(args: Vec<&'a str>) -> Vec<(&'a str, Vec<&'a str>)> {
        let mut arguments = Vec::new();
        for arg in args {
            arguments.push(Replace::parse_argument(&arg));
        }
        arguments
    }

    fn parse_argument(arg: &'a str) -> (&str, Vec<&'a str>)  {
        
        let argument_split: Vec<&str> = arg.split(":").collect();
        let function_name = argument_split[0];
        
        //println!("\t{}", function_name);
        
        let mut input_values = Vec::new();
        for item in &argument_split[1..] {
            
            //println!("\t{}", item);
            
            input_values.push(*item);
        }
        (function_name, input_values)
    }

    pub fn run_functions(&mut self) -> Result<(), Error> {
        for argument in self.args.clone() {
            let function_name = argument.0;
            match function_name {
                "-pbr" => try!(self.p_to_br()),
                "-rets" => try!(self.remove_empty_tags()),

                "-help" => print!("{}", help::HELP),
                _ => println!("Unsupported argument {}", function_name),
            };
        }
        Ok(())
    }

    fn p_to_br(&mut self) -> Result<(), Error> {
        let re_p1 = r"<[pP].*?>";
        let re = try!(Regex::new(re_p1));
        self.html = re.replace_all(&self.html,"");

        let re_p2 = r"</[pP]>";
        let re = try!(Regex::new(re_p2));
        self.html = re.replace_all(&self.html,"<br />");

        Ok(())
    } 


    fn re_tag(tag: &str) -> String {
        String::new()
    }

    fn remove_empty_tags(&mut self)  -> Result<(), Error> {
        let re_string = r"<(p|h1|h2|div)>[&nbsp;\s]*?</(p|h1|h2|div)>";
        let re = try!(Regex::new(re_string));
        self.html = re.replace_all(&self.html, "");
        Ok(())
    }

    pub fn replace_openning_tag(tag: &str, replace: &str) {}

    pub fn replace_closing_tag(tag: &str, replace: &str) {}

    pub fn set_attribute(tag: &str, attribute: &str, value: &str) {}

    pub fn remove_attribute(tag: &str, attribute: &str) {}

    pub fn change_attribute_value(tag: &str, attribute: &str, value: &str) {}
}
