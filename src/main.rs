mod days;

mod thread_pool;
mod worker;

use std::sync::mpsc::{self, Sender};
use thread_pool::ThreadPool;

fn load_thread_pool(thread_pool: &ThreadPool, sender: Sender<(usize, String)>) {

    for day in days::get_days() {
        let day_clone = day.clone();
        let sender_clone = sender.clone();

        thread_pool.execute(move || {
            let day_clone = day_clone.lock().unwrap();
            
            let message = format!("{day_clone}");
            let day_number = usize::from(day_clone.day_count());

            let _ = sender_clone.send((day_number, message));
        });
    }
}

fn main() {
    let (sender, receiver) = mpsc::channel::<(usize, String)>();
    let thread_pool = ThreadPool::new(5);

    load_thread_pool(&thread_pool, sender);

    let mut messages: [Option<String>; days::day_count::NUMBER_DAYS] = Default::default();
    let mut day_count = 0;

    for (day_number, message) in receiver {
        messages[day_number - 1] = Some(format!("{message}\n"));

        if let Some(message) = messages[day_count].clone() {
            print!("{}", message);
            day_count += 1;
        }
    }

    for i in day_count..days::day_count::NUMBER_DAYS {
        if let Some(message) = messages[i].clone() {
            print!("{}", message);
        }
    }
}
