pub fn main() {
    env_logger::init();

    let responder = libmdns::Responder::new().unwrap();
    let _svc = responder.register(
        "_http._tcp".to_owned(),
        "libmdns Web Server".to_owned(),
        80,
        &["path=/"],
	120
    );

    loop {
        ::std::thread::sleep(::std::time::Duration::from_secs(10));
    }
}
