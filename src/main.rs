use rserver::web_dealer::get_service;
use rserver::web_dealer::{Route, WebDealer};

fn main() {
    //    let addr = String::from("0.0.0.0:8000");      // In docker container
    let addr = String::from("localhost:8080");

    let route = vec![
        Route::new(
            "GET".to_string(),
            (String::from("/"), Some(String::from("/static/index.html"))),
            get_service,
        )
        .unwrap(),
        Route::new(
            "GET".to_string(),
            (String::from("/static/post_example.html"), None),
            get_service,
        )
        .unwrap(),
    ];
    let _dealer = WebDealer::new(&addr, route).unwrap();

    loop {
        println!("listening to {}", addr);
        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
}
