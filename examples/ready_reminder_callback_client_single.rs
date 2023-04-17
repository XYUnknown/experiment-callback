use std::time::{SystemTime, Duration};
use experiment_callback::reminder::ready_reminder_callback_server::{ReadyReminderServer};
use experiment_callback::reminder::callback::{CallBack};

/**
 * A program that specifies times at which some events will become ready, then
 * gets notified when they become ready.
 * 
 */
fn main () {
    let mut r = ReadyReminderServer::new();
    r.submit_event(CallBack::new("Goodbye World!".to_string()), SystemTime::now() + Duration::new(6, 0));
    r.submit_event(CallBack::new("Hello World!".to_string()), SystemTime::now() + Duration::new(3, 0));
    r.run();
}