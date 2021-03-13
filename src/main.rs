#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
#![allow(unused_imports)]

mod pm;

fn main() {
    //println!("{}", json!({"token": "4768c0dacbad724ce311e761930398a6147a7f64"}).to_string());
    //pm::hello_who();
    pm::get_vault_token("2de68856100b8283a6f500795c3cb460aa2850cf","https://vault-ho.autobahn.comcast.com:8200/v1/auth/github_cdn-analytics/login");
    //pm::get_url("https://or.vault.comcast.com/v1/auth/github_cdn-analytics/login");
    //pm::hex2int("0xb757b1");
    //s.7h6uMgh4zsra8lg3yyrD9KK7
}
