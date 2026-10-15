// Copyright 2018-2026 the Deno authors. MIT license.
//
// On Apple platforms, hickory-resolver 0.26 reads DNS settings from
// SystemConfiguration. That path rejects zone-scoped IPv6 servers such as
// `fe80::1%en0`, which routers commonly advertise, so every lookup fails.
// hickory-resolver 0.25 read `/etc/resolv.conf` on Apple platforms, which
// accepts them. This module restores that behavior.
//
// The conversion below is adapted from hickory-resolver
// (`src/system_conf/unix.rs`, Apache-2.0 OR MIT). See NOTICE.md.

pub(crate) use imp::read_system_conf;

#[cfg(target_vendor = "apple")]
mod imp {
  use std::io;
  use std::str::FromStr;
  use std::time::Duration;

  use hickory_resolver::config::NameServerConfig;
  use hickory_resolver::config::ResolverConfig;
  use hickory_resolver::config::ResolverOpts;
  use hickory_resolver::net::NetError;
  use hickory_resolver::proto::rr::Name;

  pub(crate) fn read_system_conf()
  -> Result<(ResolverConfig, ResolverOpts), NetError> {
    let data = std::fs::read("/etc/resolv.conf")?;
    parse_resolv_conf(&data)
  }

  pub(crate) fn parse_resolv_conf(
    data: &[u8],
  ) -> Result<(ResolverConfig, ResolverOpts), NetError> {
    let parsed = resolv_conf::Config::parse(data).map_err(|e| {
      io::Error::other(format!("Error parsing resolv.conf: {e}"))
    })?;

    let domain = parsed
      .get_system_domain()
      .and_then(|domain| Name::from_str(domain.as_str()).ok());

    let nameservers: Vec<NameServerConfig> = parsed
      .nameservers
      .iter()
      .map(|ip| NameServerConfig::udp_and_tcp(ip.into()))
      .collect();
    if nameservers.is_empty() {
      return Err(io::Error::other("no nameservers found in config").into());
    }

    let mut search = vec![];
    for search_domain in parsed.get_last_search_or_domain() {
      // Ignore invalid search domains, as hickory does.
      if search_domain == "--" {
        continue;
      }
      search.push(Name::from_str_relaxed(search_domain).map_err(|e| {
        io::Error::other(format!("Error parsing resolv.conf: {e}"))
      })?);
    }

    let config = ResolverConfig::from_parts(domain, search, nameservers);
    // `ResolverOpts` is non-exhaustive, so it can't be built with a literal.
    let mut options = ResolverOpts::default();
    options.ndots = parsed.ndots as usize;
    options.timeout = Duration::from_secs(u64::from(parsed.timeout));
    options.attempts = parsed.attempts as usize;
    options.edns0 = parsed.edns0;
    Ok((config, options))
  }

  #[cfg(test)]
  mod tests {
    use std::net::IpAddr;

    use super::*;

    #[test]
    fn accepts_zone_scoped_ipv6_nameserver() {
      let (config, _) = parse_resolv_conf(
        b"search lan\nnameserver fe80::1%en0\nnameserver 192.168.1.1\n",
      )
      .unwrap();
      let ips: Vec<IpAddr> =
        config.name_servers().iter().map(|ns| ns.ip).collect();
      assert_eq!(
        ips,
        vec![
          "fe80::1".parse::<IpAddr>().unwrap(),
          "192.168.1.1".parse::<IpAddr>().unwrap(),
        ]
      );
    }

    #[test]
    fn rejects_config_without_nameservers() {
      assert!(parse_resolv_conf(b"search lan\n").is_err());
    }
  }
}

#[cfg(not(target_vendor = "apple"))]
mod imp {
  pub(crate) use hickory_resolver::system_conf::read_system_conf;
}
