use std::{
    sync::mpsc,
    thread::{self, sleep, ThreadId},
    time::Duration,
};
#[derive(Debug)]
struct Message<T> {
    value: T,
    id: ThreadId,
}

fn main() {
    let (sender, reciever) = mpsc::channel();
    let sender2 = sender.clone();
    thread::spawn(move || {
        for i in 0..100 {
            println!("Sending the number: {}", i);
            sender
                .send(Message {
                    value: i,
                    id: thread::current().id(),
                })
                .unwrap();
            sleep(Duration::from_secs(1))
        }
    });
    thread::spawn(move || {
        for i in 0..100 {
            println!("Sending the number: {}", i);
            sender2
                .send(Message {
                    value: i,
                    id: thread::current().id(),
                })
                .unwrap();
            sleep(Duration::from_secs(1))
        }
    });
    for recieved in reciever {
        println!("Value recieved: {:?}", recieved)
    }
}
