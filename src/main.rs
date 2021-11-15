use rserver::web_dealer::WebDealer;

fn main() {
    let addr = String::from("127.0.0.1:8000");
    let _dealer = WebDealer::new(&addr).unwrap();

    loop {
        println!("listening to {}", addr);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
