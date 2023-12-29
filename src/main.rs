mod days;

use std::thread::{self, JoinHandle};
use std::sync::mpsc::{self, Sender};

fn create_thread(sender: Sender<(usize, String)>) -> Vec<JoinHandle<()>> {
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    for day in days::get_days() {
        let day_clone = day.clone();
        let sender_clone = sender.clone();

        let handle = thread::spawn(move || {
            let day_clone = day_clone.lock().unwrap();
            
            let message = format!("{day_clone}");
            let day_number = usize::from(day_clone.day_count());

            let _ = sender_clone.send((day_number, message));
        });

        handles.push(handle);
    }

    handles
}

fn main() {
    let (sender, receiver) = mpsc::channel::<(usize, String)>();
    let handles = create_thread(sender);

    for handle in handles {
        handle.join().unwrap();
    }

    let mut messages: [String; days::day_count::NUMBER_DAYS] = Default::default();
    for (day_number, message) in receiver {
        messages[day_number - 1] = format!("{message}\n");
    }    

    for message in messages {
        print!("{}", message);
    }
}
