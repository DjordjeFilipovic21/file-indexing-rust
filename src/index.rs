use std::collections::HashMap;
use crate::GLOBAL_MAP;

pub fn worker_index(data: String, file_name: String) {
    let mut local_map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();

    for sentence in data.split(|c| ".!?".contains(c)) {
        for word in sentence.split(|c| " ,.;:!?()\n\r\t".contains(c)) {
            let word_entry = local_map.entry(word.to_string()).or_insert_with(HashMap::new);
            let context_entry = word_entry.entry(file_name.to_string()).or_insert_with(Vec::new);
            context_entry.push(sentence.trim().to_string());
        }
    }

    let mut global_map = GLOBAL_MAP.lock().unwrap();
    for (word, file_map) in local_map {
        let global_entry = global_map.entry(word).or_insert_with(HashMap::new);
        for (file, contexts) in file_map {
            let global_contexts = global_entry.entry(file).or_insert_with(Vec::new);
            global_contexts.extend(contexts);
        }
    }
}