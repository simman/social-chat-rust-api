
fn test_thread() {
    // 1.
    thread::Builder::new()
        .name("Thread for Rust Chat with ws-rs".into())
        // 2.
        .spawn(|| {
            for num in 1..1000 {
                thread::sleep(std::time::Duration::from_secs(2));
                println!("ok...{}", num);
            }
        })
        .unwrap();
}