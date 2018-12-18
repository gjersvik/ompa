use super::messages::{Chore, Chores, LoadChores};
use actix::{Actor, Handler, SyncContext};
use chrono::{DateTime, Duration, NaiveDateTime, Utc};
use native_tls::TlsConnector;
use postgres::{
    tls::{Stream, TlsHandshake, TlsStream},
    Connection, TlsMode,
};
use std::{
    collections::HashMap,
    error::Error,
    fmt, io,
    io::{Read, Write},
};

pub struct NativeTls(TlsConnector);

impl fmt::Debug for NativeTls {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("NativeTls").finish()
    }
}

impl TlsHandshake for NativeTls {
    fn tls_handshake(
        &self,
        domain: &str,
        stream: Stream,
    ) -> Result<Box<TlsStream>, Box<Error + Send + Sync>> {
        let stream = self.0.connect(domain, stream)?;
        Ok(Box::new(TestStream(stream)))
    }
}

#[derive(Debug)]
struct TestStream(native_tls::TlsStream<Stream>);

impl TlsStream for TestStream {
    fn get_ref(&self) -> &Stream {
        self.0.get_ref()
    }

    fn get_mut(&mut self) -> &mut Stream {
        self.0.get_mut()
    }
}

impl Read for TestStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        self.0.read(buf)
    }
}

impl Write for TestStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> Result<(), io::Error> {
        self.0.flush()
    }
}

fn connect(uri: String) -> Option<Connection> {
    let negotiator = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let negotiator = NativeTls(negotiator);

    Connection::connect(uri, TlsMode::Prefer(&negotiator))
        .map_err(|e| {
            eprintln!("Database connection error: {}", e);
        })
        .ok()
}

pub struct Database {
    conn: Option<Connection>,
}

impl Database {
    pub fn new(uri: String) -> Database {
        Database { conn: connect(uri) }
    }
}

impl Actor for Database {
    type Context = SyncContext<Self>;
}

impl Handler<LoadChores> for Database {
    type Result = Chores;

    fn handle(&mut self, _: LoadChores, _: &mut Self::Context) -> Self::Result {
        if let Some(conn) = &self.conn {
            let mut chores = HashMap::new();
            for row in &conn.query("SELECT id, name, EXTRACT(EPOCH FROM frequency) AS frequency, last_done FROM public.chores", &[]).unwrap() {
                let id: i32 = row.get(0);
                let name: String = row.get(1);
                let frequency:f64 = row.get(2);
                let last_done: Option<NaiveDateTime> = row.get(3);

                let chore = Chore {
                    id, name, frequency: Duration::seconds(frequency as i64), last_done: last_done.map(|n| DateTime::from_utc(n,Utc)),
                };

                chores.insert(chore.id, chore);
            }

            Chores(chores)
        } else {
            return Default::default();
        }
    }
}
