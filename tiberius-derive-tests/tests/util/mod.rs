#![allow(dead_code)]

use once_cell::sync::Lazy;
use tiberius::Client;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncWriteCompatExt};

static CONN_STR: Lazy<String> = Lazy::new(|| {
    std::env::var("TIBERIUS_TEST_CONNECTION_STRING").unwrap_or_else(|_| {
        "server=tcp:localhost,1433;User Id=localuser;Password=123456;IntegratedSecurity=true;TrustServerCertificate=true".to_owned()
    })
});

pub async fn connect_localhost() -> Result<Client<Compat<TcpStream>>, tiberius::error::Error> {
    let config = tiberius::Config::from_ado_string(&CONN_STR)?;
    let tcp = tokio::net::TcpStream::connect(config.get_addr()).await?;
    tcp.set_nodelay(true)?;
    let client = tiberius::Client::connect(config, tcp.compat_write()).await?;
    Ok(client)
}
