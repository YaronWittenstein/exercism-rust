use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let new_input = input
        .iter()
        .map(|s| s.to_lowercase())
        .collect::<Vec<String>>();

    let input_arc = Arc::new(new_input);
    let mut handles = Vec::with_capacity(worker_count);

    for worker in 0..worker_count {
        let input_clone = Arc::clone(&input_arc);

        let handle = thread::spawn(move || {
            let mut stats = HashMap::<char, usize>::new();

            for (i, s) in input_clone.iter().enumerate() {
                if i % worker_count == worker {
                    for c in s.chars() {
                        if c.is_alphanumeric() && !c.is_digit(10) {
                            let counter = stats.entry(c).or_insert(0);
                            *counter += 1;
                        }
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
