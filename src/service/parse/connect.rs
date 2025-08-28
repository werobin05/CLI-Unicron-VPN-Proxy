use crate::structs::connection::Connection;
use url::Url;

pub fn parse_connection_url(url: &str) -> Option<Connection> {
    let parsed = Url::parse(url).ok()?;

    let protocol = parsed.scheme().to_string();
    let host = parsed.host_str()?.to_string();
    let port = parsed.port().unwrap_or(443);

    let user = parsed.username().to_string();
    let pass = parsed.password().unwrap_or("").to_string();

    let fragment = parsed.fragment().unwrap_or("Unnamed").to_string();

    let pbk = parsed
        .query_pairs()
        .find(|(k, _)| k == "pbk")
        .map(|(_, v)| v.to_string());
    let sni = parsed
        .query_pairs()
        .find(|(k, _)| k == "sni" || k == "peer")
        .map(|(_, v)| v.to_string());

    Some(Connection {
        name: fragment,
        server: host,
        port,
        key: if !user.is_empty() { user } else { pass },
        protocol,
        pbk,
        sni,
    })
}
