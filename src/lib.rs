pub mod functions;
use rstest::*;

#[rstest]
#[case("1.2.3.4:8080", 8080)]
#[case("127.0.0.1:9000", 9000)]

fn check_port(#[case] addr: std::net::SocketAddr, #[case] expected: u16) {
    assert_eq!(addr.port(), expected);
}
