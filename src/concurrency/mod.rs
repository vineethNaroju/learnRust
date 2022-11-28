
pub mod threads {
    use std::{thread, sync::{Arc, Mutex}, ops::Add};

    pub fn arc_mutex_threads() {

        // Arc<T> just keeps an atomic counter to track references 
        // but doesn't implement sync trait over T.

        let mut counter = Arc::new(Mutex::new(10));

        let mut thread_handles = Vec::with_capacity(1000);

        for i in 0..1000 {
            let counter_clone = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                let mut ccx = counter_clone.lock().unwrap();
                *ccx = *ccx + 1; 
                println!("i:{},ccx:{}", i, *ccx);
            });

            thread_handles.push(handle);
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        println!("{}", *counter.lock().unwrap());
    }

    pub mod message_passing {

    }




}

pub mod message_passing {
    use std::{sync::mpsc, thread, time::Duration};

    pub fn channels() {
        let (tx, rx) = mpsc::channel();

        
        
        let mut i = 1;


        loop {
            if i >= 3{
                break;
            }

            let ntx = tx.clone();

            thread::spawn(move || {
                let arr = vec!["yo".to_string(), String::from("killua")];

                for mut val in arr {
                    val.push_str(&i.to_string());
                    ntx.send(val).unwrap();
                    thread::sleep(Duration::from_secs(1 * i));
                }
            });

            i += 1;
        }

        for res in rx {
            println!("got:{}", res);
        }



        


    }
}