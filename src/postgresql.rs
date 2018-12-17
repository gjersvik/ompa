use chrono::NaiveDateTime;
use native_tls::TlsConnector;
use postgres::tls::Stream;
use postgres::tls::TlsHandshake;
use postgres::tls::TlsStream;
use postgres::{Connection, TlsMode};
use std::error::Error;
use std::fmt;
use std::io;
use std::io::Read;
use std::io::Write;

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

pub fn test(uri: String) {
    let negotiator = TlsConnector::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let negotiator = NativeTls(negotiator);

    let conn = Connection::connect(uri, TlsMode::Prefer(&negotiator)).unwrap();

    for row in &conn.query("SELECT id, name, EXTRACT(EPOCH FROM frequency) AS frequency, last_done FROM public.chores", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let frequency:f64 = row.get(2);
        let last_done: Option<NaiveDateTime> = row.get(3);
        println!("Row:[{},{},{:?},{:?}]", id, name, frequency, last_done);
    }
}
