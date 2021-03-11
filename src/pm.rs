pub fn powerful_match(x: &str) -> String
{
    //println!("{:?}", x);
    match x 
    {
        "me" => "Hello!".to_string(),
        "Jason" => "Hi Jason, you're the Author right?".to_string(),
        _ => format!("{}, {}","What ever you say...", x)
    }
}

use curl::easy::Easy;
use std::io::stdout;
use std::io::Write;
use std::i128;

pub fn get_url(u: &str) {
    let mut easy = Easy::new();
    easy.follow_location(true);
    easy.url(u).unwrap();
    easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("{}", easy.response_code().unwrap());
}

pub fn hex2int(h: &str)
{
    let t = &h[0..2];
    let hd = match t {
        "0x" => &h[2..h.len()],
        _ => h
    };
    println!("DEBUG: {:?}", hd);
    let a = i128::from_str_radix(&hd, 16);
    println!("{}", a.unwrap_or(0));
}




