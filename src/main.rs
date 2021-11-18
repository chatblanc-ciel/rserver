use rserver::web_dealer::WebDealer;

fn main() {
//    let addr = String::from("0.0.0.0:8000");      // In docker container
    let addr = String::from("localhost:8080"); 
    let _dealer = WebDealer::new(&addr).unwrap();

    loop {
        println!("listening to {}", addr);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
