use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

mod dealer_error;
mod http_def;

use crate::web_dealer::http_def::HttpRequest;

use self::dealer_error::DealerError;

/// # WebDealer
///
/// working to deliver Http Response by analizing listened Http Request.
///
pub struct WebDealer<T> {
    listener: Arc<TcpListener>,
    worker: JoinHandle<T>,
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
        let listener_cp = Arc::clone(&listener);
        let thread = thread::spawn(move || {
            for stream in listener_cp.incoming() {
                let stream = stream.unwrap();

                Self::handle_connection(stream);
            }
        });

        Ok(Self {
            listener,
            worker: thread,
        })
    }

    /// analyze Http Request
    ///
    fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        let _ = stream.read(&mut buffer).unwrap();
		let request = String::from_utf8_lossy(&buffer);

        let root = format!("{}", HttpRequest::Get(String::from("/")));

        let (status_line, filename) = if request.starts_with(&root) {
            ("HTTP/1.1 200 OK\r\n\r\n", "./static/top.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./static/404.html")
        };

        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();

        file.read_to_string(&mut contents).unwrap();

        let response = format!("{}{}", status_line, contents);

        stream.write_all(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
