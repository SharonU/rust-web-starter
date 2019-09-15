use rocket::http;
use rocket::request;
use rocket::Outcome;
use rocket::State;
use nats::Client;
use r2d2::PooledConnection;
use r2d2_nats::NatsConnectionManager;
use std::ops::Deref;
use std::ops::DerefMut;
use std::env;

type Pool = r2d2::Pool<NatsConnectionManager>;

pub fn init_pool() -> Pool {
    let manager =
        NatsConnectionManager::new(nats_url())
            .expect("connection manager");

    return r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();
}

fn nats_url() -> String {
    env::var("NATS_ADDRESS")
        .expect("NATS_ADDRESS environment variable must be set")
}

pub struct NatsConnection(pub PooledConnection<NatsConnectionManager>);

impl<'a, 'r> request::FromRequest<'a, 'r> for NatsConnection {
    type Error = ();

    fn from_request(request: &'a request::Request<'r>) -> request::Outcome<NatsConnection, ()> {
        let pool = request.guard::<State<Pool>>()?;

        match pool.get() {
            Ok(conn) => Outcome::Success(NatsConnection(conn)),
            Err(_) => Outcome::Failure((http::Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for NatsConnection {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NatsConnection {
    fn deref_mut(&mut self) -> &mut nats::Client {
        &mut self.0
    }
}
