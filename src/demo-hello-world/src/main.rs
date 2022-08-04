use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported
use std::thread;
use std::time::Duration;

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");

    let mut n: i32 = 0;

    loop {
        println!("Hello again {}", n);
        n = n + 1;
        thread::sleep(Duration::from_secs(3));
    }
}
