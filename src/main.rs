//#[plugin]
mod concurrent;

fn main() {
    let p1 = concurrent::Process::start("p1".to_string());
    let mr = p1.msgSender.clone();
    let rs = p1.statusReceiver;

    mr.send(concurrent::Message::RequestStatus).unwrap();
    let r1 = rs.recv().unwrap();
    println!("{:?}", r1);
}
