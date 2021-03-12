#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_imports)]

mod pm;
use curl::easy::Easy;

fn main() {
    use curl::easy::Easy;

// First write everything into a `Vec<u8>`
let mut data = Vec::new();
let mut handle = Easy::new();
handle.url("http://url.com").unwrap();
{
    let mut transfer = handle.transfer();
    transfer.write_function(|new_data| {
        data.extend_from_slice(new_data);
        Ok(new_data.len())
    }).unwrap();
    transfer.perform().unwrap();
}

// Convert it to `String`
let body = String::from_utf8(data).expect("body is not valid UTF8!");
println!("{:?}",body);
    //println!("{}", json!({"token": "4768c0dacbad724ce311e761930398a6147a7f64"}).to_string());
    //pm::hello_who();
    //pm::get_vault_token("790353980f93e631f183c25deac0caf517d31dea","https://vault-ho.autobahn.comcast.com:8200/v1/auth/github_cdn-analytics/login");
    //pm::get_url("https://or.vault.comcast.com/v1/auth/github_cdn-analytics/login");
    //pm::hex2int("0xb757b1");
    //s.7h6uMgh4zsra8lg3yyrD9KK7
}
