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
    pub msgSender: Sender<Message>,

    pub statusReceiver: Receiver<Status>,
    pub statusSender: Sender<Status>
}


pub enum Message {
    RequestStatus,
    RequestCancel
}

impl Process {
    pub fn start (id: String) -> Process {
        let (msgS,msgR) = channel::<Message>();
        let (statusS,statusR) = channel::<Status>();
        let outp = statusS.clone();

        let h = thread::spawn(move || {
            let mut cond = true;
            while cond  {
                let msg = msgR.recv().unwrap();
                match msg {
                    Message::RequestStatus => {
                        outp.send(Status{progress: 10, context : "nada".to_string()});
                        cond = true;
                    },
                    Message::RequestCancel => cond = false,
                }
            }
        });

        return Process{
            id : id,
            //msgReceiver : msgR,
            msgSender : msgS,
            statusReceiver : statusR,
            statusSender : statusS
        };
    }
}
