use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::thread::{self, JoinHandle};

mod dealer_error;

use self::dealer_error::DealerError;

/// # WebDealer
///
/// working to deliver HTTP Response by analizing listened HTTP Request.
///
/// # Input
///
/// - listening address
///
/// # Error
///
/// Can not connect to listening address.
pub struct WebDealer<T> {
    listener: Arc<TcpListener>,
    worker: JoinHandle<T>,
}
impl WebDealer<()> {
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
        let thread = thread::spawn(move || {});

        Ok(Self {
            listener,
            worker: thread,
        })
    }
}
