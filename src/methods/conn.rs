use std::io::{Read, Write, stdout};
use std::net::TcpStream;
use std::sync::Arc;

use rustls::{ClientConfig, RootCertStore, VecInput};
use rustls_util::Stream;
use rustls::pki_types::ServerName;
use rustls_aws_lc_rs;

pub fn create_server_name(sv_str: String) -> ServerName<'static> {
    let sv_name = ServerName::try_from(sv_str);

    match sv_name {
        Ok(v) => return v,
        Err(e) => panic!("Fatal Error, could not resolve ServerName from input address.\n({e})"),
    }
}

pub fn create_tcp_conn(sv_string: String) -> TcpStream {
    let sock = TcpStream::connect(sv_string);

    match sock {
        Ok(v) => return v,
        Err(e) => panic!("Fatal Error, could not create TCP connection.\n({e})"),
    }
}

pub fn build_to_arc(cfg: ClientConfig, sv_name: ServerName<'static>) -> Arc<ClientConfig> {
    let mut conn = Arc::new(cfg)
        .connect(sv_name)
        .build();

    match conn {
        Ok(v) => return v,
        Err(e) => panic!("Fatal Error, unable to build Arc.\n({e})"),
    }
}

pub fn create_config(sv_str: String) -> Arc<ClientConfig> {
    let root_store = RootCertStore {
        roots: webpki_roots::TLS_SERVER_ROOTS.into(),
    };

    let sv_name = create_server_name(sv_str);

    let mut config = ClientConfig::builder(rustls_aws_lc_rs::DEFAULT_PROVIDER.into())
        .with_root_certificates(root_store)
        .with_no_client_auth();

    match config {
        Ok(v) => return build_to_arc(v, sv_name),
        Err(e) => panic!("Fatal Error, unable to generate ClientConfig.\n({e})"),
    }
}

pub fn create_tcp_connection(sv_string: String) ->Option<Stream<'static>> {
    let mut split_sv = sv_string.split(":");
    let sv_addr = split_sv.next()?;
    let sv_port = split_sv.next()?;

    let mut sock = create_tcp_conn(sv_string);

    let mut input = VecInput::default();

    let mut received_plaintext = Vec::new();

    let mut output = Vec::new();

    let mut conn = create_config(sv_addr.to_string());

    let tls = Stream::new(&mut input, &mut conn, &mut sock,);


}