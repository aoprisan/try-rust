//#[plugin]
mod concurrent;
mod restserver;

fn main() {
    let p1 = concurrent::Process::start("p1".to_string());
    let mr = p1.msg_sender.clone();
    let rs = p1.status_receiver;

    mr.send(concurrent::Message::RequestStatus).unwrap();
    let r1 = rs.recv().unwrap();
    println!("{:?}", r1);

    mr.send(concurrent::Message::RequestStatus).unwrap();
    let r2 = rs.recv().unwrap();
    println!("{:?}", r2);

    restserver::start();
}
