use rdev::{self, *};
use windows::Win32::UI::Input::KeyboardAndMouse::{keybd_event, KEYBD_EVENT_FLAGS};
use std::process::exit;
use std::thread;
use std::sync::mpsc;
use std::time::Duration;


// Fps-based
const SPACE_FPS: u64 = 250;
const SPACE_DELAY_MS: Duration = Duration::from_millis(1000 / SPACE_FPS);

// Delay-based
// const SPACE_DELAY_MS: Duration = Duration::from_millis(4);

const INPUT_KEY: Key = Key::ShiftLeft;
// const OUTPUT_KEY: Key = Key::Space;

enum ToggleState {
    Start,
    Stop
}

fn main() {
    // set handler for ctrl+c
    let res = ctrlc::set_handler(|| {
        println!("Exiting");
        exit(0);
    });
    if res.is_err() {
        println!("[Error] - Unable to set handler for SIGTERM interrupt\nExiting");
        return;
    }

    // make two objects that can communicate with each other across threads
    let (tx, rx) = mpsc::channel::<ToggleState>();
    
    // second thead to handle sending space repeatedly (never exits)
    thread::spawn(move || {
        loop {
            // idle
            // wait forever for other thread to send something
            // if it receives "Start", it exits, otherwise it loops again
            loop {
                if let Ok(ToggleState::Start) = rx.recv() {
                    break;
                }
            }

            // send output key rapidly
            loop {
                // Win32's `keybd_event` works inside games unlike rdev's `simulate`
                unsafe {
                    // TODO: figure out the proper mapping for arbitrary keys like python keyboard does:
                    // https://github.com/boppreh/keyboard/blob/master/keyboard/_winkeyboard.py
                    keybd_event(32, 57, KEYBD_EVENT_FLAGS(0), 0);
                    keybd_event(32, 57, KEYBD_EVENT_FLAGS(2), 0);
                };

                // wait some time for the other thread to send the "Stop" signal
                if let Ok(ToggleState::Stop) = rx.recv_timeout(SPACE_DELAY_MS) {
                    break;
                }
            }
        }
    });

    // function that gets called when key is pressed
    let control_other_thread = move |event: Event| -> Option<Event> {
        match event.event_type {
            EventType::KeyPress(INPUT_KEY) => {
                if tx.send(ToggleState::Start).is_err() {
                    println!("[Error] - Problem communicating with other thread. Proceeding");
                }
                // suppress event by returning None
                return None;
            },
            EventType::KeyRelease(INPUT_KEY) => {
                if tx.send(ToggleState::Stop).is_err() {
                    println!("[Error] - Problem communicating with other thread. Proceeding");
                }
                return None;
            },
            _ => {}
        }

        Some(event)
    };

    if rdev::grab(control_other_thread).is_err() {
        println!("[Error] - Unable to register callback\nExiting");
    }
}
