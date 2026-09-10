use std::io::{Read, Write, stdout};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{ClientConfig, RootCertStore, Stream};

pub fn create_tcp_stream(to_addr: String) -> TcpStream {
    let ret_stream =  TcpStream::connect(to_addr);

    match ret_stream {
        Ok(v) => return v,
        Err(e) => panic!("Fatal Error: Could not form TCP connection.\n({e})"),
    }
}

pub fn create_arc_conn(sv_name: String, out: &mut Vec, conf: ClientConfig) {
    let mut conn = Arc::new(conf)
        .connect(sv_name)
        .build(out);
}