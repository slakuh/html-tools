use regex::{Error, Regex};

#[derive(Debug)]
pub struct Replace {//(String);
    html: String,
    args: Vec<(String, Vec<String>)>,
}

impl Replace {
    pub fn new(html: &str, argument: &str) -> Replace {
        Replace {
            html: html.to_string(),
            args: Replace::parse_argument(argument),
        }
    } 

    fn parse_argument(arg: &str) -> Vec<(String, Vec<String>)>  {
        let arg = arg.to_string();
        let mut attr_vec: Vec<String> = Vec::with_capacity(2);// = arg.split("-").skip(1).collect();
        for item in arg.split("-").skip(1) {
            attr_vec.push(item.to_string());
        }
        let mut func_values = Vec::new();
        for item in attr_vec {
            func_values.push(Replace::argument(item));
        }
        func_values
    }

    fn argument(attr: String) ->(String, Vec<String>) {
        let mut attribute: Vec<String> = Vec::new();// = attr.split(":").collect();
        for item in attr.split(":") {
            attribute.push(item.to_string());
        }        
        let function_name = attribute[0].clone();
        let mut input_values: Vec<String> = Vec::new();// = attribute[1].trim().split(" ").collect::<Vec<String>>();
        for item in attribute[1].trim().split(" ") {
            input_values.push(item.to_string());
        }    
        (function_name, input_values)
    }

    pub fn run_functions(&mut self) -> Result<(), Error> {
        for argument in &self.args {
            let function_name = argument.0;
            match function_name {
                "pbr".to_string() => try!(self.p_to_br()),
                _ => print!("Unsupported argument {}", function_name),
            };
        }
        Ok(())
    }

    fn p_to_br(&mut self) -> Result<(), Error> {
        //let strin = "[pP]";
        //let re_p1 = &(r"<)".to_string() + strin +".*?>");
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

    pub fn replace_openning_tag(tag: &str, replace: &str) {}

    pub fn replace_closing_tag(tag: &str, replace: &str) {}

    pub fn set_attribute(tag: &str, attribute: &str, value: &str) {}

    pub fn remove_attribute(tag: &str, attribute: &str) {}

    pub fn change_attribute_value(tag: &str, attribute: &str, value: &str) {}
}
