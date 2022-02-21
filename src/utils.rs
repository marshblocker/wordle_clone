use std::{thread, time};

pub fn sleep_sec(sec: u64) {
    let sec = time::Duration::from_secs(sec);
    thread::sleep(sec);
}

pub fn sleep_ms(ms: u64) {
    let ms = time::Duration::from_millis(ms);
    thread::sleep(ms);
}

pub fn clear_screen() {
    println!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}