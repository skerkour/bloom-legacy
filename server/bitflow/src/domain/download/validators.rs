use kernel::KernelError;
use regex::Regex;
use std::net;
use url::Url;

pub fn url(download_url: &str) -> Result<(), KernelError> {
    // TODO: valdiate url...
    let parsed_url = Url::parse(&download_url)?;
    let scheme = parsed_url.scheme();
    if scheme != "http" && scheme != "https" && scheme != "magnet" {
        return Err(KernelError::Validation(
            "Url is not valid. It must be a Http(s) or magnet url.".to_string(),
        ));
    }

    match scheme {
        "magnet" => {
            let re = Regex::new(r"magnet:\?xt=urn:btih:[a-zA-Z0-9]*")
                .expect("error compiling magnet regex");
            if !re.is_match(&download_url) {
                return Err(KernelError::Validation(
                    "magnet Url is not valid".to_string(),
                ));
            }
        }
        _ => {
            match parsed_url.host() {
                Some(host) => {
                    let invalid_hosts = vec![
                        "127.0.0.1",
                        "localhost",
                        "0.0.0.0",
                        "::1:",
                        "bloom.sh",
                        "::1",
                        "[::1]",
                        "169.254.169.254",          // aws / DO metadata endpoint
                        "metadata.google.internal", // Google cloud platform metadata endpoint
                        "metadata.packet.net",      // packet.netmetadata endpoint
                    ];
                    if invalid_hosts.contains(&host.to_string().as_str()) {
                        return Err(KernelError::Validation("host is not valid".to_string()));
                    }
                    if let Ok(ip_address) = host.to_string().parse::<net::IpAddr>() {
                        if !ip_address.is_global() {
                            return Err(KernelError::Validation(
                                "private IP addresses are not valid".to_string(),
                            ));
                        }
                    }
                }
                None => return Err(KernelError::Validation("Url is not valid.".to_string())),
            }
        }
    }

    return Ok(());
}
