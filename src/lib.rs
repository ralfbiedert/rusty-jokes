pub mod laughable_lifetimes {
    use std::time::Duration;

    pub fn set_joke_lifetime(joke: &str, lifetime: Duration) {
        println!("Setting the lifetime of joke '{}' to {} seconds.", joke, lifetime.as_secs());
    }
}

pub mod comedic_concurrency {
    use std::sync::{Arc, Mutex};
    use std::thread;

    pub fn concurrent_laughter(joke: &str, num_threads: u32) {
        let shared_joke = Arc::new(Mutex::new(joke.to_owned()));

        let mut handles = vec![];

        for i in 0..num_threads {
            let cloned_joke = Arc::clone(&shared_joke);
            let handle = thread::spawn(move || {
                let joke = cloned_joke.lock().unwrap();
                println!("Thread {}: Laughing at '{}'", i + 1, joke);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}

pub mod punchline_pointers {
    pub fn deliver_punchline(joke: &str, punchline: &str) {
        let punchline_ptr = punchline.as_ptr();
        let len = punchline.len();
        unsafe {
            let raw_punchline = std::slice::from_raw_parts(punchline_ptr, len);
            let punchline_str = std::str::from_utf8_unchecked(raw_punchline);
            println!("Joke: {}\nPunchline: {}", joke, punchline_str);
        }
    }
}

pub fn chuckle_norris() -> String {
    let joke = "Why do Rust programs never have buffer overflows?";
    let punchline = "Because Chuck Norris guards the memory!";
    format!("{} {}", joke, punchline)
}

// HUMAN NOTE - My supervisor choked here.
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test
