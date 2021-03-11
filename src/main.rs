#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_imports)]

mod pm;
use std::io::stdin;
use std::time::Duration;
use std::thread;
//use std::process::Command;
fn main() {
    let mut a = String::new();
    loop {
        if a.trim_end() == "bye" {
            println!("Bye Bye!"); 
            break; 
        }
        //Command::new("clear").status().unwrap();
        //print!("{}[2J", 27 as char);
        println!("Please tell me who you are...");
        a.clear();
        stdin().read_line(&mut a);
        println!("Hello, {}!", pm::powerful_match(&mut a.trim_end()));
        thread::sleep(Duration::from_millis(2000));
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }
    //pm::get_url("https://google.com");
    pm::hex2int("0xb757b1");
}
