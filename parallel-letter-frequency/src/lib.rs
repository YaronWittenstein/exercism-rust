use std::collections::HashMap;
use std::thread;

pub fn frequency(input: &[&'static str], worker_count: usize) -> HashMap<char, usize> {
    let mut chunks = vec![Vec::<&str>::new(); worker_count];

    for (i, text) in input.iter().enumerate() {
        chunks[i % worker_count].push(text);
    }

    let mut handles = Vec::with_capacity(worker_count);

    for chunk in chunks {
        let handle = thread::spawn(move || {
            let mut stats = HashMap::<char, usize>::new();

            for s in chunk {
                for c in s.to_lowercase().chars() {
                    if c.is_alphanumeric() && !c.is_digit(10) {
                        let counter = stats.entry(c).or_insert(0);
                        *counter += 1;
                    }
                }
            }

            stats
        });

        handles.push(handle);
    }

    let mut acc = HashMap::new();

    for handle in handles {
        let thread_stats = handle.join().unwrap();

        for (k, v) in thread_stats {
            let counter = acc.entry(k).or_insert(0);
            *counter += v;
        }
    }
    acc
}
