#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
#[macro_use] extern crate rocket;
use std::path::PathBuf;
use rocket::response::NamedFile;

#[get("/page/<path..>")]
fn get_page(path: PathBuf) ->  &'static str { 
    /* ... */ 
    return "5"
}

#[get("/ello/<name>")]
fn ello(name : &RawStr) -> String {
    format!("Hello, {}!",name.as_str())
   
}

#[get("/hello/<name>")]
fn hello(name : &RawStr) -> String {
    format!("Hello, {}!",name.as_str())
   
}
#[get("/llo/<name>/<age>/<cool>")]
fn llo(name: String, age: u8, cool: u8) -> String {
    
    if cool>50 {
        format!("active connection {} in database server, {}! try to contect tutree administrator \"MANISH\" try few hours ago", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

fn main() {
    rocket::ignite().mount("/", routes![ello,llo,hello, get_page]).launch();
    
}