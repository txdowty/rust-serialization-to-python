use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

use std::io::prelude::*;
use std::os::unix::net::UnixStream;

// use systemstat::{Platform, System};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct TestData1 {
    x: i64,
    y: f32,
    z: Vec<u8>,
}

fn main() {
    let mut data = TestData1 { x: 100, y: 200.0, z: [1, 2, 3].to_vec() };

    let path = "/home/dowty/rust/ts/ipc/ipc.sock";

    let mut stream = match UnixStream::connect(path) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Couldn't connect: {e:?}");
            return;
        }
    };

    let (tx, rx) = channel();

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(3));
            data.x += 1;
            let json_string = serde_json::to_string(&data).unwrap();
            let _ = tx.send(json_string);
        }
    });

    loop {
        let _ = rx
            .try_recv()
            .map(|reply| stream.write_all(reply.as_bytes()));
    }
}
