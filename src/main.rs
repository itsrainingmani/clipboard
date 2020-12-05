#[macro_use]
extern crate lazy_static;
use rdev::{listen, Event, EventType, Key};
use std::collections::VecDeque;
use std::process::Command;
use std::sync::Mutex;

// Have to use a Mutex to be able to modify EVNT_Q
lazy_static! {
    static ref EVNT_Q: Mutex<VecDeque<EventType>> = Mutex::new(VecDeque::new());
    static ref CLIP_Q: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
}

fn pbpaste_output() -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("pbpaste")
        .output()
        .expect("failed to execute process");
    let str_output = std::str::from_utf8(&output.stdout).unwrap().trim();
    println!("Contents of Clipboard: {:#?}", str_output);
    String::from(str_output)
}

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(x) => {
            EVNT_Q.lock().unwrap().push_back(EventType::KeyPress(x));
        }
        EventType::KeyRelease(k) => {
            if k == Key::KeyC {
                EVNT_Q.lock().unwrap().pop_back().unwrap();
                let prev_key = EVNT_Q.lock().unwrap().back().unwrap().clone();
                match prev_key {
                    EventType::KeyPress(k1) => {
                        if k1 == Key::MetaLeft {
                            println!("We got Jackpot!");
                            EVNT_Q.lock().unwrap().clear();
                            CLIP_Q.lock().unwrap().push_back(pbpaste_output());
                        }
                    }
                    _ => {}
                };
            }
        }
        _ => {}
    }
}

fn main() {
    println!("Starting the CLIpboard Utility");

    // Initialize the CLIP_Q with the current contents of the clipboard
    CLIP_Q.lock().unwrap().push_back(pbpaste_output());

    loop {
        // This will block.
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
    }
}
