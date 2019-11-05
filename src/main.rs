#[macro_use]
extern crate tower_web;

use serde_json::Value;
use tower_web::middleware::log::LogMiddleware;
use tower_web::ServiceBuilder;

struct MyApp;

impl_web! {
    impl MyApp {
        #[post("/hook")]
        fn hook(&self, body: Value) -> Result<&'static str, ()> {
            println!("{:#?}", body);
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
        let app = MyApp {};
        assert_eq!(app.index(), Ok("hello"));
    }
}
