extern crate actix_web;
extern crate env_logger;

use actix_web::{server, App, HttpRequest, Path, http};

use env_logger::{Builder, Target};


// fn index(_req: &HttpRequest) ->String {
fn greet(path: Path<String>) -> Option<String> {
    Some(format!("Hello {}, you scurvy dog", path))
}

fn goodbye(path: Path<String>) -> Option<String> {
    Some(format!("Fare thee well {} !", path))
}

fn main() {

    // Initialize logging
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();


    // server::new(|| App::new().resource("/greet/{name}", |r| r.f( index )))
    //     .bind("127.0.0.1:8088")
    //     .unwrap()
    //     .run();


    // Using parameters --- needs an extractor 

    // let my_app = App::new().resource(
    //     "/greet/{name}", 
    //     |r| r.method(http::Method::GET).with( greet )
    // ).finish();

    // Lets break this up !!
    // App::new() - creates Self (an App)

     server::new(|| App::new()
        .resource("/greet/{name}", |r| r.method(http::Method::GET).with( greet ))
        .resource("/goodbye/{name}", |r| r.method(http::Method::GET).with( goodbye ))
        )
        .bind("0.0.0.0:8080") // wildcard & 8080
        .unwrap()
        .run();
    ;
}