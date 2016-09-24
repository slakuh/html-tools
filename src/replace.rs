use constants;
use help;
use regex::{Error, Captures, Regex};

#[derive(Debug)]
pub struct Replace<'a, 'b> {
    pub clipboard: String,
    args: Vec<(&'a str, Vec<&'a str>)>,
    str_empty: &'b str,
    str_space: &'b str,
    str_new_line: &'b str,
    str_return: &'b str,
    str_tab: &'b str,
}

impl<'a, 'b> Replace<'a, 'b> {
    pub fn new(clipboard: String, argument: Vec<&'a str>) -> Replace<'a, 'b> {
        Replace {
            clipboard: clipboard,
            args: Replace::parse_arguments(argument),
            str_empty: "",
            str_space: " ",
            str_new_line: "\n",
            str_return: "\r",
            str_tab: "\t",
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
                "-r" => self.replace_string(argument.1[0], argument.1[1]),
                // mijenja string koji odgovara regularnom izrazu
                "-rre" => try!(self.replace_string_regex(argument.1[0], argument.1[1])),
                // uklanja ponavljajuće stringove, slova...
                "-rd" => self.remove_double(argument.1[0]),
                // uklanja prazne tagove (p|h1|h2|div)
                "-rets" => try!(self.remove_empty_tags()),
                // čisti tagove od atributa    
                "-raa" => try!(self.remove_atributes_all()),
                // uklanja tag
                "-rt" => try!(self.remove_tag(argument.1[0])),
                // Mjenja jedan tag u drugi
                "-ct" => self.change_tag(argument.1[0], argument.1[1]),
                // radi uri linkove
                "-ml" => try!(self.make_links(argument.1[0])),
                // radi email linkove
                "-me" => try!(self.make_emails()),
                // dodaje atribut tagu ili mijenja vrijednost postojećem atributu
                "-sa" => try!(self.set_attribute(argument.1[0], argument.1[1], argument.1[2])),
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
/*
    fn replace_string_regex_2(&mut self, regex: &'a str, to: &'a str) -> Result<(), Error> {
        let regex = self.replace_special_arg(regex);
        let re = try!(Regex::new(&regex));
        let mut unique: UniqueVec<(String, String)> = UniqueVec::new();
        if try!(self.is_supstitute(to)) {
            for captures in re.captures_iter(&self.clipboard) {
                let from = captures.at(0).unwrap();
                let to = self.replace_supstitute(captures, to);
                unique.push((from.to_string(), to));
            }

            for item in unique.vec {
                self.clipboard = self.clipboard.replace(&item.0, &item.1);
            }

        } else {
            //ovdje nisma uklanjao specijalne znakove u "to" jer je javljao gešku            
            self.clipboard = re.replace_all(&self.clipboard, to);
            //zato su specialni znakovu uklonjeni ovdje
            self.clipboard = self.replace_special_arg(&self.clipboard);
        }

        Ok(())
    }

    // vraća true ako se u replace stringu nalaze zankovi za supstitute 
    // [$0],[$1],[$2]...,
    fn is_supstitute(&self, rep: &str) -> Result<bool, Error> {
        let re_str = r"\[\$\d+\]"; // regex for [$4]-[$3]-[$2]-[$1]
        let re = try!(Regex::new(re_str));
        Ok(re.is_match(rep))


    }
    // koonstruira replace string iz regex capture i supstitte replace
    fn replace_supstitute(&self, captures: Captures, rep: &str) -> String {
        let mut rep = rep.to_string();
        for (i, capture) in captures.iter().enumerate() {
            let supst = "[$".to_string() + &i.to_string() + "]";
            rep = rep.replace(&supst,  &capture.unwrap());           
        }    
        println!("{}", rep);
        rep

    }
  */
    fn replace_special_arg(&self, value: &'b str) -> String {
        let mut value = value.replace(constants::SPECIAL_SPACE, self.str_space);
        value = value.replace(constants::SPECIAL_EMPTY, self.str_empty);
        value = value.replace(constants::SPECIAL_NEW_LINE, self.str_new_line);
        value = value.replace(constants::SPECIAL_RETURN, self.str_return);
        value = value.replace(constants::SPECIAL_TAB, self.str_tab);
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

    // kreira tag string u obliku tag|TAG
    fn re_tag(tag: &str) -> String {
        if tag.len() > 0 {
            tag.to_lowercase() + "|" + &tag.to_uppercase()
        } else {
            String::new()
        }
    }

    fn remove_tag(&mut self, tag: &'a str) -> Result<(), Error> {
        // "</?(tag|TAG).*?>
        let re_string = "</?(".to_string() + &Replace::re_tag(tag)+ ").*?>[\\s\\n\\r]*";
        println!("{:?}", re_string);
        let re = try!(Regex::new(&re_string));
        self.clipboard = re.replace_all(&self.clipboard, "");
        Ok(())
    }

    fn remove_empty_tags(&mut self)  -> Result<(), Error> {
        //let re_str = r"<(p|h1|h2|div)>[&nbsp;\s]*?</(p|h1|h2|div)>";
        let re_str = r"<(p|h1|h2|div).*?>[&nbsp;\s]*?</(p|h1|h2|div)>[\s\n\r]*";
        let re = try!(Regex::new(re_str));
        self.clipboard = re.replace_all(&self.clipboard, "");
        Ok(())
    }

    // TODO: moguć višestruki nepotrebni string.replace(), provjeriti  
    fn remove_atributes_all(&mut self) -> Result<(), Error> {
        let re_str = r"<(\w+)\s+.*?>";
        let re = try!(Regex::new(re_str));
        for capture in re.captures_iter(&self.clipboard.clone()) {
            let tag = "<".to_string() + capture.at(1).unwrap() + ">";
            // TODO: moguć višestruki nepotrebni string.replace(),  
            // jer kad jednom izmijeni onda izmjeni sve, pa ne treba ponovo pokušavati
            // radi ali nije optimalno
            self.clipboard = self.clipboard.replace(capture.at(0).unwrap(), &tag);
        }
        Ok(())
    }

    fn change_tag(&mut self, tag_from: &'a str, tag_to: &'a str) {   
        let from_open = "<".to_string() + tag_from;
        let to_open = "<".to_string() + tag_to;
        self.clipboard = self.clipboard.replace(&from_open, &to_open);
        let from_close = "</".to_string() + tag_from;
        let to_close = "</".to_string() + tag_to;
        self.clipboard = self.clipboard.replace(&from_close, &to_close);
    }
/*
    fn make_links(&mut self, target: &'a str) -> Result<(), Error> {
        let ref target = if target.len() > 0 {
            " target=\"".to_string() + target + "\""
        } else {
            target.to_string()
        };
        //let re_str = r"(http|ftp|https)://[\w-_]+(\.[\w-_]+)+([\w-\.,@?^=%&amp;:/~\+#]*[\w-@?^=%&amp;/~\+#])?";
        let re_str = "(https?|ftp)://[^\\s/$.?#].[^\\s\"]*"; // https://mathiasbynens.be/demo/url-regex @stephenhay
        let re = try!(Regex::new(re_str));
        // vektor sa jedinstvenim vrijednostima za replace
        let mut unique: UniqueVec<(String, String)> = UniqueVec::new();
        for capture in re.captures_iter(&self.clipboard.clone()) {
            let from = capture.at(0).unwrap();
            // replace je napravljen zbog više linkova sa istom adresom
            let ref from_mod = from.replace(".", "[~DOT~]");
            let to = "<a href=\"".to_string() + from_mod + "\"" + target +">" + from_mod + "</a>";
            self.clipboard = self.clipboard.replace(from, &to);
        }
        self.clipboard = self.clipboard.replace("[~DOT~]", ".");
        Ok(())
    }

    fn make_emails(&mut self) -> Result<(), Error> {
        //let re_str = r"[A-Za-z0-9](([_\.-]?[a-zA-Z0-9]+)*)@([A-Za-z0-9]+)(([\.-]?[a-zA-Z0-9]+)*)\.([A-Za-z]{2,})";
        let re_str = r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+"; // http://emailregex.com/ @python
        let re = try!(Regex::new(re_str));
        for capture in re.captures_iter(&self.clipboard.clone()) {
            let from = capture.at(0).unwrap();
            // replace je napravljen zbog više emailova sa istom adresom
            let ref from_mod = from.replace("@", "[~AT~]");
            let to = "<a href=\"mailto:".to_string() + from_mod + "\">" + from_mod + "</a>";
            self.clipboard = self.clipboard.replace(from, &to);
        }
        self.clipboard = self.clipboard.replace("[~AT~]", "@");
        Ok(())
    }
*/    
    
    fn make_links(&mut self, target: &'a str) -> Result<(), Error> {
        let ref target = if target.len() > 0 {
            " target=\"".to_string() + target + "\""
        } else {
            target.to_string()
        };

        let re_str = "(https?|ftp)://[^\\s/$.?#].[^\\s\"]*"; // https://mathiasbynens.be/demo/url-regex @stephenhay
        let re = try!(Regex::new(re_str));
        // vektor sa jedinstvenim vrijednostima za replace
        let mut unique: UniqueVec<(String, String)> = UniqueVec::new();

        for capture in re.captures_iter(&self.clipboard) {
            let from = capture.at(0).unwrap();
            let to = "<a href=\"".to_string() + from + "\"" + target +">" + from + "</a>";        
            unique.push((from.to_string(), to));
        }
        for item in &unique.vec {
            self.clipboard = self.clipboard.replace(&item.0, &item.1);
        }
        Ok(())
    }
    
    fn make_emails(&mut self) -> Result<(), Error> {        
        let re_str = r"[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+"; // http://emailregex.com/ @python
        let re = try!(Regex::new(re_str));
        // vektor sa jedinstvenim vrijednostima za replace
        let mut unique: UniqueVec<(String, String)> = UniqueVec::new();
        for capture in re.captures_iter(&self.clipboard) {
            let from = capture.at(0).unwrap();
            let to = "<a href=\"mailto:".to_string() + from + "\">" + from + "</a>";
            unique.push((from.to_string(), to));
        }        
        for item in &unique.vec {
            self.clipboard = self.clipboard.replace(&item.0, &item.1);
        }
        Ok(())
    }

    fn set_attribute(&mut self, tag: &str, attribute: &str, value: &str) -> Result<(), Error>{
        let ref re_str_tag = "<(".to_string() + &Replace::re_tag(tag) + r").*?>";
        let re_tag = try!(Regex::new(re_str_tag));

        let ref re_str_attr = "(".to_string() + &Replace::re_tag(attribute) + ")\\s*?=\\s*?\".+?\"";
        let re_attr = try!(Regex::new(re_str_attr));

        for capture in re_tag.captures_iter(&self.clipboard.clone()) {
            let tag = capture.at(0).unwrap();
            let tag_new = self.tag_new(tag, attribute, value, &re_attr);        
            self.clipboard = self.replace_special_arg(&self.clipboard.replace(tag, &tag_new));
        }
        Ok(())
    }

    fn tag_new(&self, tag: &str, attribute_name: &str, attribute_value: &str, re_attribute: &Regex) -> String {
        //  razmak je na početku da se kod čistog taga izbjegne spojen tag sa atributom
        let attribute_new = " ".to_string() + attribute_name + "=\"" + attribute_value + "\"";
        let mut tag_new;
        
        if tag.contains(&attribute_name.to_lowercase()) || tag.contains(&attribute_name.to_uppercase()) {
            let attribute_old = re_attribute.captures(tag).unwrap().at(0).unwrap();    
            tag_new = tag.replace(attribute_old, &attribute_new);
        } else {
            tag_new = tag.replace(">", &(attribute_new + ">"));
        }

        while tag_new.contains("  ") {
            tag_new = tag_new.replace("  ", " ");
        }
        tag_new  
    }

    fn parse_style_values(){}

    pub fn remove_tag_attributes(&mut self, tag: &str) {}
}

#[derive(Debug)]
struct UniqueVec<T> {
    pub vec: Vec<T>,
    //index: usize
}

impl<T: PartialEq> UniqueVec<T> {
    pub fn new() -> UniqueVec<T> {
        UniqueVec::<T>{vec: Vec::new()}
    }

    pub fn push(&mut self, item: T) {// where T: std::cmp::PartialEq<T> {
        if self.vec.contains(&item) {
            return;
        }
        self.vec.push(item);
    }

    
}
/*
impl<Iterator for UniqueVec<T>  {
    // we will be counting with usize
    type Item = T;

    // next() is the only required method
    fn next(&mut self) -> Option<T> {
        self.vec.iter().nth(Item)
    }
}
*/