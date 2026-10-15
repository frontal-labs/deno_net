use std::net::SocketAddr;

use deno_net::DefaultTlsOptions;
use deno_net::raw::NetworkStreamAddress;
use deno_net::resolve_addr::resolve_addr_sync;

#[test]
fn resolves_literal_addresses_without_dns() {
  let addresses: Vec<_> =
    resolve_addr_sync("127.0.0.1", 8080).unwrap().collect();
  assert_eq!(addresses, vec![SocketAddr::from(([127, 0, 0, 1], 8080))]);
}

#[test]
fn accepts_bracketed_ipv6_literals() {
  let addresses: Vec<_> = resolve_addr_sync("[::1]", 443).unwrap().collect();
  assert_eq!(
    addresses,
    vec![SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], 443))]
  );
}

#[test]
fn wraps_socket_addresses_and_keeps_default_tls_options_empty() {
  let address =
    NetworkStreamAddress::from(SocketAddr::from(([127, 0, 0, 1], 80)));
  assert!(
    matches!(address, NetworkStreamAddress::Ip(addr) if addr.port() == 80)
  );
  let options = DefaultTlsOptions {
    root_cert_store_provider: None,
  };
  assert!(options.root_cert_store().unwrap().is_none());
}
