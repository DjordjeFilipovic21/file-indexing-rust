mod index;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::ptr::read;
use std::sync::Mutex;

static GLOBAL_MAP: Lazy<Mutex<HashMap<String, HashMap<String, Vec<String>>>>> = Lazy::new(|| Mutex::new(HashMap::new()));

fn read_file(file_name: &str) -> Result<String, io::Error> {
    let file = File::open(file_name)?;
    let mut contents = String::new();
    BufReader::new(file).read_to_string(&mut contents)?;
    Ok(contents)
}
fn main() {
    let mut handles = Vec::new();

    loop {
        print!(">>> ");
        io::stdout().flush().expect("Failed to flush stdout"); // Ensure the output is displayed immediately
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if input.contains("index") {
            if let Some(file_name) = input.split(" ").last() {
                let contents = read_file(file_name);
                match contents {
                    Ok(data) => {
                        let handle = std::thread::spawn(move || index::worker_index(data, file_name.to_string()));
                        handles.push(handle);
                    }
                    Err(e) => println!("Failed to read file: {}", e),
                }
            } else {
                println!("Error: No file name provided after 'index'");
            }
        } else if input.contains("search") {
            if let Some(word) = input.split(" ").last() {
                let word = word.trim();
                for (fileName, context) in GLOBAL_MAP.lock().unwrap().get(word).unwrap() {
                    println!("\tRec {} nadjena je u fajlu {} {} puta", word, fileName, context.len());
                    let mut br = 0;
                    for sentence in context {
                        println!("\t\tSentence {}: {}", br, sentence);
                        br += 1;
                    }
                }
            }
            else{
                println!("Error: No file name provided after 'search'");
            }
        } else if input.eq("stop") {
            break;
        } else {
            println!("Invalid input");
        }
    }
    for handle in handles {
        handle.join().expect("Failed to join thread");
    }
}
