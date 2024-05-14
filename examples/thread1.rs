use anyhow::{anyhow, Result};
use std::{sync::mpsc, thread, time::Duration};

const PRODUCE_NUM: usize = 4;

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    data: usize,
}

impl Msg {
    fn new(idx: usize, data: usize) -> Self {
        Self { idx, data }
    }
}

fn produce(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let data = rand::random::<usize>();
        let msg = Msg::new(idx, data);
        tx.send(msg).unwrap();
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        if rand::random::<u8>() % 5 == 0 {
            println!("produce {} exit", idx);
            break;
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    for i in 0..PRODUCE_NUM {
        let tx = tx.clone();
        thread::spawn(move || produce(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consume: {:?}", msg);
        }
        println!("consumer exit");
    });

    consumer
        .join()
        .map_err(|e| anyhow!("Thread join error: {:?}", e))?;

    Ok(())
}
