//! # Rust data conversion demo
//!
//! This is a demo to show how to use rust convention for converting different data type
//!
//! For running the live testing, plz run: `cargo watch -c --exec 'test -- --nocapture'`
//!
//! - `std::str::FromStr`: Any type `T` implemented this trait will be able to call `T::from_str(&str)`
//!
//! - `std::convert::{From, Into}`: Any type `T` implemented the `From<U>` trait will be able to call:
//!     - `T::from(U)`
//!     - `U.into()`
//!
//! # Examples
//! ```rust
//! // Get back the `HttpServerConfig` from `&str`, as it implemented `FromStr` trait.
//! let test_connection_str = "https://www.rust-lang.org";
//! let temp_result = HttpServerConfig::from_str(test_connection_str);
//!
//! // Get back `UdpServerConfig` from `HttpServerConfig`, as it implemented `From<HttpServerConfig>` trait.
//! let config: UdpServerConfig = temp_result.unwrap().into();
//! println!(
//!     "test_connection_str (into UdpServerConfig): {:?}",
//!     test_connection_str
//! );
//! 
//! println!("config: {:?}", &config)
//! assert_eq!(type_of(&config), "from_into_train_demo::UdpServerConfig");
//! assert_eq!(config.host, "www.rust-lang.org");
//! assert_eq!(config.port, 443);
//! ```
use std::convert::From;
use std::fmt;
use std::str::FromStr;

/// Own conversion result type
type ConversionResult<T> = Result<T, ServerConfigConversionError>;

#[derive(Debug, Clone, Copy)]
pub enum ServerProtocolType {
    Http,
    SecureHttp,
    WebSocket,
    SecureWebSocket,
    Tcp,
    Udp,
    MongoDB,
}

// ------------------------------------ ServerConfigConversionError -------------------------------

pub struct ServerConfigConversionError {
    error_message: String,
}

impl fmt::Display for ServerConfigConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

impl fmt::Debug for ServerConfigConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}

// ------------------------------------ Common util functions -------------------------------------
fn parse_config_from_str(
    value: &str,
) -> (Option<String>, Option<String>, Option<u16>, Option<String>) {
    // http://www.google.com:8080
    // https://www.google.com
    // ws://www.google.com:8080/path
    // wss://www.google.com:8080/path
    // tcp://www.google.com:4000
    // udp://www.google.com:5000
    let temp_vec = value.split(':').collect::<Vec<&str>>();
    // println!("temp_vec: {:?}", temp_vec);
    if temp_vec.len() < 2 {
        return (None, None, None, None);
    }

    // Handle protocol
    let protocol = temp_vec[0].trim();
    if protocol.len() < 1 {
        return (None, None, None, None);
    }

    // Handle host
    let host_and_path = temp_vec[1].replace("//", "");
    let host_path_vec = host_and_path.split("/").collect::<Vec<&str>>();
    // println!("host_path_vec len: {:?}", host_path_vec.len());
    // println!("host_path_vec: {:#?}", host_path_vec);
    let host = host_path_vec[0].trim();
    // println!("host: {}", host);
    // println!("host len: {}", host.len());

    if host.len() < 1 || host.find('.').is_none() {
        return (Some(protocol.to_owned()), None, None, None);
    }

    let mut result = (Some(protocol.to_owned()), Some(host.to_owned()), None, None);

    // Handle path followed by host
    if host_path_vec.len() == 2 {
        result.3 = Some(host_path_vec[1].to_owned());
    }

    // Handle port (or maybe with path)
    if temp_vec.len() == 3 {
        let port_and_path = temp_vec[2].trim().split("/").collect::<Vec<&str>>();
        // println!("port_and_path: {:#?}", port_and_path);

        let port_result = port_and_path[0].trim().parse::<u16>();
        // println!("port: {:?}", port_result);

        if port_result.is_ok() {
            result.2 = Some(port_result.unwrap());
        }

        if port_and_path.len() == 2 {
            result.3 = Some(port_and_path[1].to_owned());
        }
    }

    result
}

// ------------------------------------ Http Server Config ----------------------------------------

#[derive(Debug, Clone)]
pub struct HttpServerConfig {
    pub protocol_type: ServerProtocolType,
    pub host: String,
    pub port: u16,
}

impl FromStr for HttpServerConfig {
    type Err = ServerConfigConversionError;

    fn from_str(value: &str) -> ConversionResult<Self> {
        let error_message =
            "Invalid input, valid http config string would look like this: 'http[s]://host_name[:port]'".to_string();

        let (protocol, host, port, _) = parse_config_from_str(value);
        if protocol.is_none()
            || host.is_none()
            || (protocol.as_ref().unwrap() != "http" && protocol.as_ref().unwrap() != "https")
        {
            return Err(ServerConfigConversionError { error_message });
        }

        let protocol_type = protocol.unwrap();

        Ok(HttpServerConfig {
            protocol_type: if protocol_type == "https" {
                ServerProtocolType::SecureHttp
            } else {
                ServerProtocolType::Http
            },
            host: host.unwrap(),
            port: match port {
                Some(inner_port) => inner_port,
                None => {
                    if protocol_type == "https" {
                        443
                    } else {
                        80
                    }
                }
            },
        })
    }
}

// ------------------------------------ Web Socket Server Config ----------------------------------

#[derive(Debug, Clone)]
pub struct WebSocketServerConfig {
    pub protocol_type: ServerProtocolType,
    pub host: String,
    pub port: u16,
    pub path: String,
}

impl FromStr for WebSocketServerConfig {
    type Err = ServerConfigConversionError;

    fn from_str(value: &str) -> ConversionResult<Self> {
        let error_message =
            "Invalid input, valid web socket config string would look like this: 'ws[s]://host_name[:port][/path]'".to_string();

        let (protocol, host, port, path) = parse_config_from_str(value);
        if protocol.is_none()
            || host.is_none()
            || (protocol.as_ref().unwrap() != "ws" && protocol.as_ref().unwrap() != "wss")
        {
            return Err(ServerConfigConversionError { error_message });
        }

        let protocol_type = protocol.unwrap();

        Ok(WebSocketServerConfig {
            protocol_type: if protocol_type == "wss" {
                ServerProtocolType::SecureWebSocket
            } else {
                ServerProtocolType::WebSocket
            },
            host: host.unwrap(),
            port: match port {
                Some(inner_port) => inner_port,
                None => {
                    if protocol_type == "wss" {
                        443
                    } else {
                        80
                    }
                }
            },
            path: if path.is_some() {
                path.unwrap()
            } else {
                "".to_string()
            },
        })
    }
}

// ------------------------------------ Tcp Server Config -----------------------------------------

#[derive(Debug, Clone)]
pub struct TcpServerConfig {
    pub protocol_type: ServerProtocolType,
    pub host: String,
    pub port: u16,
}

impl FromStr for TcpServerConfig {
    type Err = ServerConfigConversionError;

    fn from_str(value: &str) -> ConversionResult<Self> {
        let error_message =
            "Invalid input, valid tcp config string would look like this: 'tcp://host_name:port'"
                .to_string();

        let (protocol, host, port, _) = parse_config_from_str(value);
        if protocol.is_none()
            || host.is_none()
            || protocol.as_ref().unwrap() != "tcp"
            || port.is_none()
        {
            return Err(ServerConfigConversionError { error_message });
        }

        Ok(TcpServerConfig {
            protocol_type: ServerProtocolType::Tcp,
            host: host.unwrap(),
            port: port.unwrap(),
        })
    }
}

// ------------------------------------ Udp Server Config -----------------------------------------

#[derive(Debug, Clone)]
pub struct UdpServerConfig {
    pub protocol_type: ServerProtocolType,
    pub host: String,
    pub port: u16,
}

impl FromStr for UdpServerConfig {
    type Err = ServerConfigConversionError;

    fn from_str(value: &str) -> ConversionResult<Self> {
        let error_message =
            "Invalid input, valid udp config string would look like this: 'udp://host_name:port'"
                .to_string();

        let (protocol, host, port, _) = parse_config_from_str(value);
        if protocol.is_none()
            || host.is_none()
            || protocol.as_ref().unwrap() != "udp"
            || port.is_none()
        {
            return Err(ServerConfigConversionError { error_message });
        }

        Ok(UdpServerConfig {
            protocol_type: ServerProtocolType::Udp,
            host: host.unwrap(),
            port: port.unwrap(),
        })
    }
}

impl From<HttpServerConfig> for UdpServerConfig {
    fn from(http_server_config: HttpServerConfig) -> Self {
        UdpServerConfig {
            protocol_type: ServerProtocolType::Udp,
            host: http_server_config.host,
            port: http_server_config.port,
        }
    }
}

impl From<TcpServerConfig> for UdpServerConfig {
    fn from(tcp_server_config: TcpServerConfig) -> Self {
        UdpServerConfig {
            protocol_type: ServerProtocolType::Udp,
            host: tcp_server_config.host,
            port: tcp_server_config.port,
        }
    }
}

// ------------------------------------ MongoDB Server Config--------------------------------------

#[derive(Debug, Clone)]
pub struct MongoDbServerConfig {
    pub protocol_type: ServerProtocolType,
    pub host: String,
    pub port: u16,
    pub user_name: String,
    pub password: String,
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn type_of<T>(_: &T) -> &'static str {
        std::any::type_name::<T>()
    }

    #[test]
    fn parse_http_from_string() {
        let test_connection_str = "http://  www.google.com : 8080";
        let temp_result = HttpServerConfig::from_str(test_connection_str);
        println!("test_connection_str: {:?}", test_connection_str);
        println!("temp_result: {:?}\n", temp_result);
        assert_eq!(temp_result.is_ok(), true);

        let config = temp_result.as_ref().unwrap();
        assert_eq!(type_of(config), "from_into_train_demo::HttpServerConfig");
        assert_eq!(config.host, "www.google.com");
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn parse_secure_http_from_string() {
        let test_connection_str = "https://www.google.com";
        let temp_result = HttpServerConfig::from_str(test_connection_str);
        println!("test_connection_str: {:?}", test_connection_str);
        println!("temp_result: {:?}\n", temp_result);

        let config = temp_result.as_ref().unwrap();
        assert_eq!(type_of(config), "from_into_train_demo::HttpServerConfig");
        assert_eq!(config.host, "www.google.com");
        assert_eq!(config.port, 443);
    }

    #[test]
    fn parse_web_socket_from_string() {
        // let temp_result = WebSocketServerConfig::from_str("ws://www.google.com");
        let test_connection_str = "ws://www.google.com/path-to-connect";
        let temp_result = WebSocketServerConfig::from_str(test_connection_str);
        println!("test_connection_str: {:?}", test_connection_str);
        println!("temp_result: {:?}\n", temp_result);

        let config = temp_result.as_ref().unwrap();
        assert_eq!(
            type_of(config),
            "from_into_train_demo::WebSocketServerConfig"
        );
        assert_eq!(config.host, "www.google.com");
        assert_eq!(config.port, 80);
        assert_eq!(config.path, "path-to-connect");
    }

    #[test]
    fn parse_secure_web_socket_from_string() {
        let test_connection_str = "wss://www.google.com/path-to-connect";
        let temp_result = WebSocketServerConfig::from_str(test_connection_str);
        println!("test_connection_str: {:?}", test_connection_str);
        println!("temp_result: {:?}\n", temp_result);

        let config = temp_result.as_ref().unwrap();
        assert_eq!(
            type_of(config),
            "from_into_train_demo::WebSocketServerConfig"
        );
        assert_eq!(config.host, "www.google.com");
        assert_eq!(config.port, 443);
        assert_eq!(config.path, "path-to-connect");
    }

    #[test]
    fn parse_secure_web_socket_with_port_and_path_from_string() {
        let test_connection_str = "wss://www.google.com:8888/path-to-connect";
        let temp_result = WebSocketServerConfig::from_str(test_connection_str);
        println!("test_connection_str: {:?}", test_connection_str);
        println!("temp_result: {:?}\n", temp_result);

        let config = temp_result.as_ref().unwrap();
        assert_eq!(
            type_of(config),
            "from_into_train_demo::WebSocketServerConfig"
        );
        assert_eq!(config.host, "www.google.com");
        assert_eq!(config.port, 8888);
        assert_eq!(config.path, "path-to-connect");
    }

    #[test]
    fn parse_tcp_from_string() {
        let test_connection_str = "tcp://www.google.com:9999";
        let temp_result = TcpServerConfig::from_str(test_connection_str);
        println!("test_connection_str: {:?}", test_connection_str);
        println!("temp_result: {:?}\n", temp_result);

        let config = temp_result.as_ref().unwrap();
        assert_eq!(type_of(config), "from_into_train_demo::TcpServerConfig");
        assert_eq!(config.host, "www.google.com");
        assert_eq!(config.port, 9999);
    }

    #[test]
    fn parse_udp_from_string() {
        let test_connection_str = "udp://www.google.com:7777";
        let temp_result = UdpServerConfig::from_str(test_connection_str);
        println!("test_connection_str: {:?}", test_connection_str);
        println!("temp_result: {:?}\n", temp_result);

        let config = temp_result.as_ref().unwrap();
        assert_eq!(type_of(config), "from_into_train_demo::UdpServerConfig");
        assert_eq!(config.host, "www.google.com");
        assert_eq!(config.port, 7777);
    }

    // #[test]
    // fn parse_mongodb_from_string() {}
    //
    #[test]
    fn parse_udp_from_tcp_config() {
        let test_connection_str = "tcp://test.com:7890";
        let temp_result = TcpServerConfig::from_str(test_connection_str);
        let config: UdpServerConfig = temp_result.unwrap().into();
        println!(
            "test_connection_str (into UdpServerConfig): {:?}",
            test_connection_str
        );
        println!("config: {:?}", &config);

        assert_eq!(type_of(&config), "from_into_train_demo::UdpServerConfig");
        assert_eq!(config.host, "test.com");
        assert_eq!(config.port, 7890);
    }

    #[test]
    fn parse_udp_from_http_config() {
        let test_connection_str = "https://www.rust-lang.org";
        let temp_result = HttpServerConfig::from_str(test_connection_str);
        let config: UdpServerConfig = temp_result.unwrap().into();
        println!(
            "test_connection_str (into UdpServerConfig): {:?}",
            test_connection_str
        );
        println!("config: {:?}", &config);

        assert_eq!(type_of(&config), "from_into_train_demo::UdpServerConfig");
        assert_eq!(config.host, "www.rust-lang.org");
        assert_eq!(config.port, 443);
    }
    //
    // #[test]
    // fn parse_udp_from_secure_http() {}
    //
    // #[test]
    // fn parse_secure_http_from_web_socket() {}
    //
    // #[test]
    // fn parse_secure_http_from_mongodb() {}
}
