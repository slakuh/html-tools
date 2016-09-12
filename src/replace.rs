use constants;
use help;
use regex::{Error, Regex};

#[derive(Debug)]
pub struct Replace<'a, 'b> {
    pub clipboard: String,
    args: Vec<(&'a str, Vec<&'a str>)>,
    str_empty: &'b str,
    str_space: &'b str,
}

impl<'a, 'b> Replace<'a, 'b> {
    pub fn new(clipboard: String, argument: Vec<&'a str>) -> Replace<'a, 'b> {
        Replace {
            clipboard: clipboard,
            args: Replace::parse_arguments(argument),
            str_empty: "",
            str_space: " ",
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
        
        let argument_split: Vec<&str> = arg.split(constants::ARG_VAL_SEPARATOR).collect();
        let function_name = argument_split[0];        
        let mut input_values = Vec::new();
        for vals in &argument_split[1..] {
            input_values.push(*vals);
        }
        
        (function_name, input_values)
    }

    pub fn run_functions(&mut self) -> Result<(), Error> {
        for argument in self.args.clone() {
            let function_name = argument.0;
            match function_name {
                // mjenja p tag u br
                "-pbr" => try!(self.p_to_br()),
                // mijenja jedan string u drugi
                "-repl" => self.replace_string(argument.1[0], argument.1[1]),
                // mijenja string koji odgovara regularnom izrazu
                "-replre" => try!(self.replace_string_regex(argument.1[0], argument.1[1])),
                // uklanja ponavljajuće stringove, slova...
                "-remd" => self.remove_double(argument.1[0]),
                // uklanja prazne tagove (p|h1|h2|div)
                "-rets" => try!(self.remove_empty_tags()),    
                "-raa" => try!(self.remove_atributes_all()),
                "-help" => print!("{}", help::HELP),
                _ => println!("Unsupported argument {}", function_name),
            };
        }
        
        Ok(())
    }

    fn replace_string(&mut self, from: &'a str, to: &'a str) {
        let from = self.replace_special_arg(from);
        let to = self.replace_special_arg(to);
        self.clipboard = self.clipboard.replace(&from, &to);
    }

    fn replace_string_regex(&mut self, regex: &'a str, to: &'a str) -> Result<(), Error> {
        let regex = self.replace_special_arg(regex);
        //ovdje nisma uklanjao specijalne znakove u "to" jer je javljao gešku
        let re = try!(Regex::new(&regex));
        self.clipboard = re.replace_all(&self.clipboard, to);
        //zato su specialni znakovu uklonjeni ovdje
        self.clipboard = self.replace_special_arg(&self.clipboard);
        Ok(())
    }

    fn replace_special_arg(&self, value: &'b str) -> String {
        let mut value = value.replace(constants::SPECIAL_SPACE, self.str_space);
        value = value.replace(constants::SPECIAL_EMPTY, self.str_empty);
        value      
    }

    fn remove_double(&mut self, val: &'a str) {        
        let val = &self.replace_special_arg(val);        
        let double = val.to_string() + val;        
        while self.clipboard.contains(&double) {
           self.clipboard = self.clipboard.replace(&double, val);
        }
    }    

    fn p_to_br(&mut self) -> Result<(), Error> {
        let re_p1 = r"<[pP].*?>";
        let re = try!(Regex::new(re_p1));
        self.clipboard = re.replace_all(&self.clipboard,"");

        let re_p2 = r"</[pP]>";
        let re = try!(Regex::new(re_p2));
        self.clipboard = re.replace_all(&self.clipboard,"<br />");

        Ok(())
    } 


    fn re_tag(tag: &str) -> String {
        unimplemented!()
    }

    fn remove_empty_tags(&mut self)  -> Result<(), Error> {
        let re_str = r"<(p|h1|h2|div)>[&nbsp;\s]*?</(p|h1|h2|div)>";
        let re = try!(Regex::new(re_str));
        self.clipboard = re.replace_all(&self.clipboard, "");
        Ok(())
    }

    fn remove_atributes_all(&mut self) -> Result<(), Error> {
        let re_str = r"<(\w+)\s+.*?>"; //r"<(\w+).*?>";
        let re = try!(Regex::new(re_str));
        for capture in &mut re.captures_iter(&self.clipboard.clone()) {
            //println!("{:?}", capture.at(1));
            let tag = "<".to_string() + capture.at(1).unwrap() + ">";
            self.clipboard = self.clipboard.replace(capture.at(0).unwrap(), &tag);
        }
        Ok(())
    }

    pub fn replace_openning_tag(tag: &str, replace: &str) {}

    pub fn replace_closing_tag(tag: &str, replace: &str) {}

    pub fn set_attribute(tag: &str, attribute: &str, value: &str) {}

    pub fn remove_attribute(tag: &str, attribute: &str) {}

    pub fn change_attribute_value(tag: &str, attribute: &str, value: &str) {}
}
