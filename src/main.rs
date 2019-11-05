#[macro_use]
extern crate tower_web;

use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use tower_web::middleware::log::LogMiddleware;
use tower_web::ServiceBuilder;

struct MyApp;

impl_web! {
    impl MyApp {
        #[post("/hook")]
        fn hook(&self, body: Value) -> Result<&'static str, ()> {
            println!("{:#?}", body);
            let mut f = OpenOptions::new()
                .create(true)
                .append(true)
                .open("/hookdata/requests.txt")
                .unwrap();
            write!(&mut f, "{}\n", serde_json::to_string_pretty(&body).unwrap()).unwrap();
            Ok("ok")
        }
    }
}

fn main() {
    let _ = env_logger::try_init();

    let addr = "0.0.0.0:9050".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(MyApp)
        .middleware(LogMiddleware::new("hello_world::web"))
        .run(&addr)
        .unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_my_app() {
        // let mut w =
        let app = MyApp {};
        assert_eq!(app.hook(serde_json::json! {null}), Ok("ok"));
    }
}
