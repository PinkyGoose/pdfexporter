mod ext;

use std::default::Default;
use std::fs;
use std::io::Write;
use std::time::Duration;
use headless_chrome::{Browser, LaunchOptions};
const FILE_TEMPLATE: &str="file://";

pub fn print_html_to_pdf(path_from: String,  path_to: String) -> i8 {
    match get_bytes_for_pdf(path_from) {
        Ok(bytes) => {
            let really_path=
            if let Some(_ind) = path_to.find(FILE_TEMPLATE){
                path_to.replace(FILE_TEMPLATE, "")
            }
            else { path_to };
            let file = fs::OpenOptions::new()
                .create(true)
                .write(true)
                .open(format!("{really_path}"));
            if let Err(e)= file{

                println!("{}", e);
                return 11
            }
            let result = file.unwrap().write_all(&bytes);
            if let Err(e)= result{

                println!("{}", e);
                return 12
            }
            0
        }
        Err(i) => {i}
    }
}
#[no_mangle]
pub fn get_bytes_for_pdf(path_from: String)-> Result<Vec<u8>, i8>{
    let browser = Browser::new(LaunchOptions{
        headless:true,
        idle_browser_timeout: Duration::from_secs(500000),
        ..Default::default()
    }) ;
    if let Err(e) = browser{

        println!("{}", e);
        return Err(1)
    }
    let browser = browser.unwrap();
    let tab = browser.new_tab();
    if let Err(e)= tab{

        println!("{}", e);
        return Err(2)
    }
    let tab = tab.unwrap();
    let really_path=
        if !path_from.contains("file://"){
            format!("file://{path_from}")
        }
        else { path_from.clone() };
    let _ = tab.navigate_to(format!("{really_path}").as_str());
    tab.set_default_timeout(Duration::from_secs(60000));
    let tab = tab.wait_until_navigated();
    if let Err(e) = tab {

        println!("{}", e);
        return Err(3)
    }
    let pdf = tab.unwrap().print_to_pdf(None);
    match pdf {
       Ok(bytes) => {Ok(bytes)}
       Err(e) => {
           println!("{}", e.to_string());
           Err(4)
       }
   }
}



#[cfg(test)]
mod tests {
    use crate::{get_bytes_for_pdf, print_html_to_pdf};

    #[test]
    fn print_html_when_path_contains_schema() {
        let res = print_html_to_pdf("file:///home/pinkygoose/pizza.html".to_string(),"file:///home/pinkygoose/result.pdf".to_string());
        assert_eq!(res, 0);
    }
    #[test]
    fn print_html_when_path_does_not_contains_schema() {
        let res = print_html_to_pdf("/home/pinkygoose/pizza.html".to_string(),"/home/pinkygoose/result.pdf".to_string());
        assert_eq!(res, 0);
    }
    #[test]
    fn get_bytes_when_path_contains_schema() {
        let res = get_bytes_for_pdf("file:///home/pinkygoose/pizza.html".to_string());
        if let Err(res) =res {
            panic!("{}", format!("ERROR {res}"))
        }
        if !res.unwrap().len()>0{
            panic!("EMPTY DOC")
        }
    }
    #[test]
    fn get_bytes_when_path_does_not_contains_schema() {
        let res = get_bytes_for_pdf("/home/pinkygoose/pizza.html".to_string());
        if let Err(res) =res {
            panic!("{}", format!("ERROR {res}"))
        }
        if !res.unwrap().len()>0{
            panic!("EMPTY DOC")
        }
    }
}
