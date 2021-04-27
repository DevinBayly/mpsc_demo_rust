use std::sync::mpsc;
use std::env::args;
use std::io::prelude::*;
use std::thread;
use std::time::{Duration,SystemTime};
use std::thread::JoinHandle;
use rand::prelude::*;

fn process_data(val: f32) -> f32 {
    val.powf(5.0)
}

struct ThreadChannel {
    t: JoinHandle<()>,
    chan: mpsc::Sender<f32>,
}

fn main() {
    println!("Hello, world!");
    let (thread_trans_orig, orig_rec) = mpsc::channel();
    let mut thread_holder = vec![];
    let num_threads = 4;
    for i in 0..num_threads {
        //clone the thread transmitter
        let thread_trans = thread_trans_orig.clone();
        let (orig_trans, thread_rec) = mpsc::channel();
        let t = thread::spawn(move || {
            // receive data on the thread_rec and transmit with the thread_trans
            loop {
                // wait for the data
                let data = match thread_rec.try_recv() {
                    Ok(data) => {
                        // return the data
                        data
                    }
                    _ => {
                        thread::sleep(Duration::from_secs(1));
                        // restart loop
                        continue
                    }
                };
                // run the proc
                let result_data = process_data(data);
                // send it back
                thread_trans.send(result_data).unwrap();
            }
        });
        thread_holder.push(ThreadChannel {
            t,
            chan: orig_trans,
        });
    }

    // send them data
    let mut rand_thread = thread_rng();
    let mut cmd_line = args();
    let data_len =cmd_line.nth(1).unwrap().parse::<usize>().unwrap();
    println!("data_len is {:?}",data_len);

    let timer = SystemTime::now();
    for i in 0..data_len {
        // alternate which thread we send the data to
        thread_holder[i%num_threads].chan.send(rand_thread.gen::<f32>()*2.0).unwrap();
    }
    // receive and add to a vector to save or transmit
    let mut result_vec =vec![];
    for v in orig_rec {
        result_vec.push(v);
        if result_vec.len() == data_len {
            break
        }
    }

    //println!("got back {:?}",result_vec);
    println!("took {:?}",timer.elapsed());




}
