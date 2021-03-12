#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_imports)]




fn string()
{
  let s = "Hello There!".to_string();
  let a = &s[6..]; // substring
  let mut k = String::from(a); // str to string
  k.push_str("sbb"); // push_str() is only for String

  println!("{}",s);
  println!("{}", s.chars().nth(2).unwrap_or('_'));

}
