use std::env;
use std::process;
use a_p_i_rust::Config;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = a_p_i_rust::run(config) {
        // --snip--
        println!("Application error: {}", e);

        process::exit(1);
    }
}








/*#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
#[macro_use] extern crate rocket;
use std::path::PathBuf;
use rocket::response::NamedFile;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
#[get("/fi")]
fn fi() -> std::io::Result<()>{
    let mut file = File::open("/home/manish/Desktop/file1.txt")?;
    let mut buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        println!("{}", line?);
    }
    
    Ok(())
}


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


#[get("/llo/<name>/<age>/<cool>")] //http://localhost:8000/llo/tutree_database/51/51
fn llo(name: String, age: u8, cool: u8) -> String {
    
    if cool>50 {
        format!("active connection {} in database server, {}! try to contect tutree administrator \"MANISH\" try few hours ago", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[get("/hello?wave&<name>")] //http://localhost:8000/hello?wave&name=hj
fn he(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())

}
use rocket::response::content;

#[get("/")]
fn json() -> content::Json<&'static str> {
    content::Json("{ 'hi': 'world' }")
}

use std::io::{Write,   BufRead, Error};
fn main() -> Result<(), Error>  {
    let mut file = File::open("/home/manish/Desktop/file1.txt")?;
    let mut buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        println!("{}", line?);
    }
    
    Ok(())
    //rocket::ignite().mount("/", routes![fi]).launch();
    
}


/*
fn main() -> std::io::Result<()> {
    let mut file = File::open("/home/manish/Desktop/file1.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    assert_eq!(contents, "Hello, world!\n");
    Ok(())
  
    
    
}*/
*/