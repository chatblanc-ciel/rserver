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
        let _request = String::from_utf8_lossy(&buffer)
            .split("\r\n\r\n")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let request: HttpRequest = String::from_utf8_lossy(&buffer)
            .to_string()
            .try_into()
            .unwrap();
        let _status_line = String::new();
        let _filename = String::new();

        let response = match &request.method {
            HttpRequestMethod::Get => Self::get_handling(request.clone()),
            HttpRequestMethod::Post => {
                unimplemented!()
            }
            _ => Err(HttpError::UndifineMethod),
        };

        let response = response.unwrap_or_else(|_| {
            let mut contents = String::new();
            File::open("./static/404.html")
                .unwrap()
                .read_to_string(&mut contents)
                .unwrap();

            HttpResponse {
                state: HttpResponseState::NotFound,
                ver: request.ver,
                body: contents,
            }
        });
        println!("{}", String::from(response.clone()));
        stream.write_all(String::from(response).as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn get_handling(request: HttpRequest) -> Result<HttpResponse, HttpError> {
        let mut filename = String::from(".");
        if request.target == "/" {
            filename += &String::from("/static/index.html");
        } else {
            filename += &request.target;
        }

        let mut file = File::open(filename).map_err(|_| HttpError::FailGetControl)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        Ok(HttpResponse {
            state: HttpResponseState::Complete,
            ver: request.ver,
            body: contents,
        })
    }
}
