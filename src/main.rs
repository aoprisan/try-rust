extern crate iron;
extern crate router;
extern crate rustc_serialize;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;
use std::io::Read;
use std::sync::{Mutex, Arc};

use std::thread;
use std::time::Duration;
use std::collections::HashMap;

mod concurrent;
use concurrent::Status;

#[derive(RustcEncodable, RustcDecodable)]
struct Greeting {
    msg: String
}

fn main() {
    let greeting = Arc::new(Mutex::new(Greeting { msg: "Hello, World".to_string() }));
    let greeting_clone = greeting.clone();

    let status: Arc<Mutex<HashMap<u64, Status>>> = Arc::new(Mutex::new(HashMap::new()));
    let status_clone = status.clone();

    let mut router = Router::new();

    let c = status.clone();
    thread::spawn(move || {
        for i in 1..100 {
            {
                let m = &mut c.lock().unwrap();
                m.insert(0, Status{ progress: i + 100000, context: "in progress".to_string()});
            }
            thread::sleep(Duration::from_secs(1));
        }
    });

    router.get("/", move |r: &mut Request| hello_world(r, &greeting.lock().unwrap(), &status.lock().unwrap()));
    router.post("/set", move |r: &mut Request|
        set_greeting(r, &mut greeting_clone.lock().unwrap(), &mut status_clone.lock().unwrap()));

    fn hello_world(_: &mut Request, greeting: &Greeting, statuses: & HashMap<u64,Status>) -> IronResult<Response> {
        let payload = json::encode(&statuses.get(&0)).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_greeting(request: &mut Request, greeting: &mut Greeting, statuses: &mut HashMap<u64,Status>) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        *greeting = json::decode(&payload).unwrap();
        println!("{:?}", statuses.get(&0));
        Ok(Response::with(status::Ok))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
