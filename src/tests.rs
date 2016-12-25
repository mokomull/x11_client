use super::*;

#[test]
fn client_init() {
    let x: Vec<u8> = ClientInit::new().into();
    assert_eq!(x, b"B\x00\x00\x0B\x00\x00\x00\x00\x00\x00");
}
