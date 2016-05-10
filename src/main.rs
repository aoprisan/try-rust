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

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Status {
    pub progress: u64,
    pub context: String
}

#[derive(RustcEncodable, RustcDecodable)]
struct StartTask {
    id: u64
}

fn start_process(status: Arc<Mutex<HashMap<u64, Status>>>, task_id: u64) {
    let c = status.clone();
    thread::spawn(move || {
        for i in 1..100 {
            {
                let m = &mut c.lock().unwrap();
                m.insert(task_id, Status{ progress: i, context: "in progress".to_string()});
            }
            thread::sleep(Duration::from_secs(1));
        }
        let m = &mut c.lock().unwrap();
        m.insert(task_id, Status{ progress: 100, context: "done".to_string()});
    });
}

fn main() {
    let status: Arc<Mutex<HashMap<u64, Status>>> = Arc::new(Mutex::new(HashMap::new()));
    let status_clone: Arc<Mutex<HashMap<u64, Status>>> = status.clone();

    let mut router = Router::new();

    router.get("/:taskId", move |r: &mut Request| task_status(r, &status.lock().unwrap()));
    router.post("/start", move |r: &mut Request|
        start_task(r, status_clone.clone()));

    fn task_status(req: &mut Request, statuses: & HashMap<u64,Status>) -> IronResult<Response> {
        let ref task_id = req.extensions.get::<Router>().unwrap().find("taskId").unwrap_or("/").parse::<u64>().unwrap();
        let payload = json::encode(&statuses.get(&task_id)).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn start_task(request: &mut Request, statuses: Arc<Mutex<HashMap<u64, Status>>>) -> IronResult<Response> {
        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let task_start_request: StartTask = json::decode(&payload).unwrap();
        start_process(statuses, task_start_request.id);
        Ok(Response::with((status::Ok, json::encode(&task_start_request).unwrap())))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
