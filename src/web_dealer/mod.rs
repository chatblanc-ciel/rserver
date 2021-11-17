use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use std::sync::Arc;
use std::thread::{self, JoinHandle};

mod dealer_error;
mod http_def;

use crate::web_dealer::http_def::*;

use self::dealer_error::DealerError;

/// # WebDealer
///
/// working to deliver Http Response by analizing listened Http Request.
///
pub struct WebDealer<T> {
    _listener: Arc<TcpListener>,
    _worker: JoinHandle<T>,
}
impl WebDealer<()> {
    /// Generate ``WebDealer``.
    ///
    /// # Input
    ///
    /// - listening address
    ///
    /// # Error
    ///
    /// Can not connect to listening address.
    pub fn new<A>(addr: A) -> Result<Self, DealerError>
    where
        A: ToSocketAddrs,
    {
        // First Step : Estublish TCP listener
        let listener;
        match TcpListener::bind(addr) {
            Ok(lis) => {
                listener = Arc::new(lis);
            }
            Err(_) => {
                return Err(DealerError::CannotListen);
            }
        }

        // Second Step : Generate worker thread
        let _listener = Arc::clone(&listener);
        let thread = thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                Self::handle_connection(stream);
            }
        });

        Ok(Self {
            _listener,
            _worker: thread,
        })
    }

    /// analyze Http Request
    ///
    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer).unwrap();
        println!("{}", String::from_utf8_lossy(&buffer));
        let request = String::from_utf8_lossy(&buffer)
            .split("\r\n\r\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let request: HttpRequest = String::from_utf8_lossy(&buffer)
            .to_string()
            .try_into()
            .unwrap();
        let mut status_line = String::new();
        let mut filename = String::new();

        match &request.method {
            HttpRequestMethod::Get => {
                status_line = format!("{} 200 OK\r\n\r\n", request.ver);

                filename = String::from(".");
                if request.target == "/" {
                    filename += &String::from("/static/index.html");
                } else {
                    filename += &request.target;
                }
            }
            HttpRequestMethod::Post => {
                unimplemented!()
            }
            _ => {
                status_line = format!("{} 404 NOT FOUND\r\n\r\n", request.ver);
                filename = String::from("./static/404.html");
            }
        }

        let mut file = File::open(filename).unwrap_or_else(|_| {
            status_line = format!("{} 404 NOT FOUND\r\n\r\n", request.ver);
            File::open("./static/404.html").unwrap()
        });
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
