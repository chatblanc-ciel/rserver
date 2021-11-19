use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, ToSocketAddrs};

use std::path::Path;
use std::sync::Arc;
use std::thread::{self, JoinHandle};

pub mod dealer_error;
pub mod http_def;

use crate::web_dealer::http_def::*;

use self::dealer_error::DealerError;

use diesel::{
    pg::PgConnection,
    prelude::*,
    r2d2::{self, ConnectionManager},
};
use tera::{Context, Tera};

/// # WebDealer
///
/// working to deliver Http Response by analizing listened Http Request.
///
pub struct WebDealer<F>
where
    F: 'static
        + FnMut(
            HttpRequest,
            String,
            Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
            Option<Tera>,
        ) -> Result<HttpResponse, HttpError>
        + Sync
        + Send
        + Clone,
{
    _listener: Arc<TcpListener>,
    _worker: JoinHandle<()>,
    _route: Arc<Vec<Route<F>>>,
    _resource: Arc<(
        Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
        Option<Tera>,
    )>,
}
impl<F> WebDealer<F>
where
    F: 'static
        + FnMut(
            HttpRequest,
            String,
            Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
            Option<Tera>,
        ) -> Result<HttpResponse, HttpError>
        + Sync
        + Send
        + Clone,
{
    /// Generate ``WebDealer``.
    ///
    /// # Input
    ///
    /// - listening address
    ///
    /// # Error
    ///
    /// Can not connect to listening address.
    pub fn new<A>(
        addr: A,
        route: Vec<Route<F>>,
        resource: (
            Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
            Option<Tera>,
        ),
    ) -> Result<Self, DealerError>
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
        let _route = Arc::new(route);
        let route_cp = Arc::clone(&_route);
        let _resource = Arc::new(resource);
        let resource_cp = Arc::clone(&_resource);

        let thread = thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();

                Self::handle_connection(stream, route_cp.clone(), resource_cp.clone());
            }
        });

        Ok(Self {
            _listener,
            _worker: thread,
            _route,
            _resource,
        })
    }

    /// analyze Http Request
    ///
    fn handle_connection(
        mut stream: TcpStream,
        route: Arc<Vec<Route<F>>>,
        resource: Arc<(
            Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
            Option<Tera>,
        )>,
    ) {
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

        let get_routing = route
            .iter()
            .cloned()
            .filter(|r| r.method == HttpRequestMethod::Get)
            .collect::<Vec<Route<F>>>();
        let post_routing = route
            .iter()
            .cloned()
            .filter(|r| r.method == HttpRequestMethod::Post)
            .collect::<Vec<Route<F>>>();
        let delete_routing = route
            .iter()
            .cloned()
            .filter(|r| r.method == HttpRequestMethod::Delete)
            .collect::<Vec<Route<F>>>();
        let put_routing = route
            .iter()
            .cloned()
            .filter(|r| r.method == HttpRequestMethod::Put)
            .collect::<Vec<Route<F>>>();

        let response = match &request.method {
            HttpRequestMethod::Get => {
                let mut result = Err(HttpError::FailGetControl);
                for mut route_dir in get_routing {
                    if route_dir.req_dir == request.target {
                        if let Some(res) = route_dir.res_dir {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                res,
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        } else {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                request.target.clone(),
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        }
                    }
                }
                result
            }
            HttpRequestMethod::Post => {
                let mut result = Err(HttpError::FailGetControl);
                for mut route_dir in post_routing {
                    if route_dir.req_dir == request.target {
                        if let Some(res) = route_dir.res_dir {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                res,
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        } else {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                request.target.clone(),
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        }
                    }
                }
                result
            }
            HttpRequestMethod::Delete => {
                let mut result = Err(HttpError::FailGetControl);
                for mut route_dir in delete_routing {
                    if request.target.contains(&route_dir.req_dir) {
                        if let Some(res) = route_dir.res_dir {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                res,
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        } else {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                request.target.clone(),
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        }
                    }
                }
                result
            }
            HttpRequestMethod::Put => {
                let mut result = Err(HttpError::FailGetControl);
                for mut route_dir in put_routing {
                    if request.target.contains(&route_dir.req_dir) {
                        if let Some(res) = route_dir.res_dir {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                res,
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        } else {
                            let serve = &mut route_dir.service;
                            result = serve(
                                request.clone(),
                                request.target.clone(),
                                resource.clone().0.clone(),
                                resource.clone().1.clone(),
                            )
                        }
                    }
                }
                result
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

pub fn get_service(
    request: HttpRequest,
    route_dir: String,
    _: Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
    _: Option<Tera>,
) -> Result<HttpResponse, HttpError> {
    let mut filename = String::from(".");
    filename += &route_dir;

    let mut file = File::open(filename).map_err(|_| HttpError::FailGetControl)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    Ok(HttpResponse {
        state: HttpResponseState::Complete,
        ver: request.ver,
        body: contents,
    })
}

pub type RouteService = fn(
    HttpRequest,
    String,
    Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
    Option<Tera>,
) -> Result<HttpResponse, HttpError>;

#[derive(Debug, Clone)]
pub struct Route<F>
where
    F: FnMut(
            HttpRequest,
            String,
            Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
            Option<Tera>,
        ) -> Result<HttpResponse, HttpError>
        + Clone,
{
    method: HttpRequestMethod,
    req_dir: String,
    res_dir: Option<String>,
    service: F,
}
impl<F> Route<F>
where
    F: 'static
        + FnMut(
            HttpRequest,
            String,
            Option<r2d2::Pool<ConnectionManager<PgConnection>>>,
            Option<Tera>,
        ) -> Result<HttpResponse, HttpError>
        + Clone,
{
    pub fn new(
        method: String,
        dir: (String, Option<String>),
        service: F,
    ) -> Result<Self, HttpError> {
        Ok(Self {
            method: method.try_into()?,
            req_dir: dir.0,
            res_dir: dir.1,
            service,
        })
    }
}

pub trait Resource {}
#[derive(Debug)]
pub struct WebResource<T: ?Sized>(Arc<T>);
impl<T> WebResource<T> {
    pub fn new(data: T) -> WebResource<T> {
        Self(Arc::new(data))
    }

    pub fn get_ref(&self) -> &T {
        self.0.as_ref()
    }

    pub fn into_inner(self) -> Arc<T> {
        self.0
    }
}
impl<T: ?Sized + 'static> Resource for WebResource<T> {}
