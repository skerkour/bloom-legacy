use kernel::KernelError;
use std::net;


pub fn scan_name(name: &str) -> Result<(), KernelError> {
    if name.len() > 64 {
        return Err(KernelError::Validation("Name's length must be lower than 64 characters".to_string()));
    }

    return Ok(());
}


pub fn target(target: &str) -> Result<(), KernelError> {
    if target.to_lowercase() != target {
        return Err(KernelError::Validation("target must be lowercase".to_string()));
    }


    let invalid_targets = vec![
        "127.0.0.1",
        "localhost",
        "0.0.0.0",
        "::1:",
        "bloom.sh",
        "::1",
        "[::1]",
        "169.254.169.254", // aws / DO metadata endpoint
        "metadata.google.internal", // Google cloud platform metadata endpoint
        "metadata.packet.net", // packet.netmetadata endpoint
    ];

    if invalid_targets.contains(&target) {
        return Err(KernelError::Validation("target is not valid".to_string()));
    }

    if let Ok(ip_address) = target.parse::<net::IpAddr>() {
        if !ip_address.is_global() {
            return Err(KernelError::Validation("private IP addresses are not valid".to_string()));
        }
    }

    return Ok(());
}
