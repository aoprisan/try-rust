use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;

#[derive(Debug)]
pub struct Status {
    pub progress: u64,
    pub context: String
}

pub struct Process {
    pub id: String,
    //pub msgReceiver: Receiver<Message>,
    pub msg_sender: Sender<Message>,

    pub status_receiver: Receiver<Status>,
    pub status_sender: Sender<Status>
}


pub enum Message {
    RequestStatus,
    RequestCancel
}

impl Process {
    pub fn start (id: String) -> Process {
        let (msg_s,msg_r) = channel::<Message>();
        let (status_s,status_r) = channel::<Status>();
        let outp = status_s.clone();

        let h = thread::spawn(move || {
            let mut cond = true;
            while cond  {
                let msg = msg_r.recv().unwrap();
                match msg {
                    Message::RequestStatus => {
                        let sr = outp.send(Status{progress: 10, context : "nada".to_string()});
                        cond = true;
                    },
                    Message::RequestCancel => cond = false,
                }
            }
        });

        return Process{
            id : id,
            msg_sender : msg_s,
            status_receiver : status_r,
            status_sender : status_s
        };
    }
}
