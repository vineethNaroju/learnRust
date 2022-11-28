pub mod solution {
    use std::{
        collections::HashMap,
        sync::{Arc, Mutex},
        thread,
    };

    pub fn single_thread(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
        let mut mp: HashMap<char, usize> = HashMap::new();

        for &ele in input {
            for curr in (*ele).char_indices() {
                if !curr.1.is_alphabetic() {
                    continue;
                }

                match mp.get_mut(&(curr.1.to_ascii_lowercase())) {
                    Some(val) => {
                        *val += 1;
                    }
                    None => {
                        mp.insert(curr.1, 1);
                    }
                }
            }
        }

        return mp;
    }

    pub fn multi_threaded_a(input: &'static [&str], worker_count: usize) -> HashMap<char, usize> {
        let mut mp: Arc<Mutex<HashMap<char, usize>>> = Arc::new(Mutex::new(HashMap::new()));

        let line_count = input.len();

        // 0, 1, 2, 3, 4, 5, 6
        // worker_count = 3
        // 7 -> 2, 2, 1
        // sz = line_count / worker_count

        let lines_per_worker = if line_count % worker_count == 0 {
            line_count / worker_count
        } else {
            1 + (line_count / worker_count)
        };

        let mut idx = 0;

        let mut worker_handles = Vec::with_capacity(worker_count);

        let input_arc = Arc::new(input);

        for _ in 0..worker_count {
            let line_count_clone = line_count.clone();
            let width = lines_per_worker.clone();
            let start_idx = idx.clone();

            let input_arc_clone = Arc::clone(&input_arc);
            let mut mp_arc_clone = Arc::clone(&mp);

            worker_handles.push(thread::spawn(move || {
                println!(
                    "start_idx:{},width:{},input_arc_clone:{}",
                    start_idx,
                    width,
                    input_arc_clone.len()
                );

                for i in 0..width {
                    if i + start_idx >= line_count_clone {
                        break;
                    }

                    for curr in (*input_arc_clone[i + start_idx]).char_indices() {
                        if !curr.1.is_alphabetic() {
                            continue;
                        }

                        let val = curr.1.to_ascii_lowercase();
                        let mut mpx = mp_arc_clone.lock().unwrap();

                        match mpx.get_mut(&val) {
                            Some(val) => {
                                *val += 1;
                            }
                            None => {
                                mpx.insert(val, 1);
                            }
                        }
                    }
                }
            }));

            idx += lines_per_worker;
        }

        for wh in worker_handles {
            wh.join().unwrap();
        }

        return mp.lock().unwrap().clone();
    }

    pub fn test_multi_threaded_map() {
        let mp: Arc<Mutex<HashMap<i32, i32>>> = Arc::new(Mutex::new(HashMap::new()));

        let thread_count = 10;
        let mut thread_handles = Vec::with_capacity(thread_count);

        for _ in 0..thread_count {
            let mp_clone = Arc::clone(&mp);

            let handle = thread::spawn(move || {
                let mut mpx = mp_clone.lock().unwrap();

                for i in 0..100 {
                    match mpx.get_mut(&i) {
                        Some(val) => {
                            *val += i;
                        }
                        None => {
                            mpx.insert(i, i);
                        }
                    }
                }
            });

            thread_handles.push(handle);
        }

        for handle in thread_handles {
            handle.join().unwrap();
        }

        for curr in mp.lock().unwrap().iter() {
            println!("{}:{}", curr.0, curr.1);
        }
    }
}
