use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn frequency<'a>(input: &'a [&'a str], worker_count: usize) -> HashMap<char, usize> {
    if input.len() == 0 {
        return HashMap::new();
    }
    if worker_count > input.len() {
        return frequency(input, input.len());
    }
    let chunk_size = input.len()/worker_count;
    let chunks = input.chunks(chunk_size);
    let result_map = HashMap::<char, usize>::new();
    let result = Arc::new(Mutex::new(result_map));
    std::thread::scope(|s| {
        for chunk in chunks {
            let result = Arc::clone(&result);
            s.spawn(move || {
                let mut partial_map: HashMap<char, usize> = HashMap::new();
                for item in chunk {
                    let item_map = frequency_in_str(item);
                    for (k, v) in item_map {
                        partial_map.entry(k).and_modify(|val| *val += v).or_insert(v);
                    }
                }
                let mut result_ref = result.lock().unwrap();
                for (k, v) in partial_map {
                    result_ref.entry(k).and_modify(|val| *val += v).or_insert(v);
                }
            });
        }
    });
    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn frequency_in_str(input: &str) -> HashMap<char, usize> {
    let mut results: HashMap<char, usize> = HashMap::new();
    for ch in input.chars() {
        if ch.is_alphabetic() {
            results.entry(ch.to_ascii_lowercase()).and_modify(|counter| *counter += 1).or_insert(1);
        }
    }
    results
}
