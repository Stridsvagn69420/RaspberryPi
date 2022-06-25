use regex::Regex;

pub enum BindType {
    /// IPv4
    /// 
    /// Bind Address is IPv4
    IPv4,
    
    #[cfg(unix)]
    /// Unix (Unix only)
    /// 
    /// Bind Address is a Unix Socket
    Unix,

    /// Not Valid
    /// 
    /// Bind Address isn't valid
    NotValid
}

/// IPv4 Regex
/// 
/// Regex for IPv4 addresses
const IPV4_REGEX: &str = r"^(?:(?:2(?:[0-4][0-9]|5[0-5])|[0-1]?[0-9]?[0-9])\.){3}(?:(?:2([0-4][0-9]|5[0-5])|[0-1]?[0-9]?[0-9]))\b:(0|[1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5])$";

#[cfg(unix)]
/// Unix Path RegEx
/// 
/// Regex for Linux, OpenBSD, NetBSD, FreeBSD and Unix paths
const UNIX_REGEX: &str = r"^(/[^/ ]*)+/?$";

pub fn validate_bind(addr: &str) -> BindType {
    if Regex::new(IPV4_REGEX).unwrap().is_match(addr) {
        return BindType::IPv4
    }

    #[cfg(unix)]
    if Regex::new(UNIX_REGEX).unwrap().is_match(addr) {
        return BindType::Unix
    }

    BindType::NotValid
}