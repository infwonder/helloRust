use std::io::stdin;
use std::time::Duration;
use std::thread;
use std::process::Command;

use curl::easy::Easy;
//use curl::easy::List;
use std::io::stdout;
use std::io::Read;
use std::io::Write;
use std::i128;
use serde_json::Value;
use serde_json::json;
use serde::{Deserialize, Serialize};
use serde_with::json::nested;
use std::str::from_utf8;
use std::cell::RefCell;

fn powerful_match(x: &str) -> String
{
    //println!("{:?}", x);
    match x 
    {
        "me" => "Hello!".to_string(),
        z @ "Jason" => format!("{}, you're the Author right?", z),
        "bye" => "and see you later!".to_string(),
        _ => format!("{}, {}","What ever you say...", x)
    }
}

pub fn hello_who()
{
  let mut a = String::new();
  loop {
        if a.trim_end() == "bye" {
            break; 
        }
        //Command::new("clear").status().unwrap();
        //print!("{}[2J", 27 as char);
        println!("Please tell me who you are...");
        a.clear();
        stdin().read_line(&mut a);
        println!("Hello, {}!", powerful_match(&mut a.trim_end()));
        thread::sleep(Duration::from_millis(2000));
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
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

pub fn get_vault_token(t: &'static str, u: &str)
{
    let data =json!({"token": t}).to_string();
    let mut easy = Easy::new();
    let mut tout : Vec<u8> = Vec::new();

    #[derive(Deserialize, Debug)]
    struct Obj {
      #[serde(with = "serde_with::json::nested")]
      auth: Issue
    }

    #[derive(Deserialize, Debug)]
    struct Issue {
      client_token: String
    }

    easy.url(u).unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.as_bytes().len() as u64).unwrap();

    easy.read_function(move|buf| {
        Ok(data.as_bytes().read(buf).unwrap_or(0))
    }).unwrap();

    let mut token : String = String::new();
    easy.transfer().write_function(|dato| {
        tout.extend_from_slice(dato);
        let s = match from_utf8(&tout[..]) {
           Ok(v) => v,
           Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
        };

        let j : Value = serde_json::from_str(&s.trim_end()).unwrap();
        token = j["auth"]["client_token"].to_string();
        Ok(dato.len())
    }).unwrap();

    println!("{:?}", token);
    easy.perform().unwrap();
}

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




