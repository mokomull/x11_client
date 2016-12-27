extern crate x11_client;
use x11_client::*;

use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() {
    let mut socket = UnixStream::connect("/tmp/.X11-unix/X0").unwrap();

    let client_init: Vec<_> = ClientInit::new().into();
    socket.write(&client_init).unwrap();

    let server_init = ServerInit::from_stream(&mut socket).unwrap();

    let create_window = CreateWindow::new(
        24,
        server_init.resource_id_base + 1,
        server_init.roots[0].root,
        100,
        100,
        1024,
        1024,
        0,
        1, // InputOutput
        0, // CopyFromParent
    );
    socket.write(&create_window.as_bytes());

    socket.write(&MapWindow::new(
        server_init.resource_id_base + 1
    ).as_bytes());

    socket.write(&CreateGc::new(
        server_init.resource_id_base + 2,
        server_init.resource_id_base + 1,
        0x0000FF,
    ).as_bytes());

    std::thread::sleep(std::time::Duration::from_millis(100));

    socket.write(&PolyFillRectangle::new(
        server_init.resource_id_base + 1,
        server_init.resource_id_base + 2,
        256,
        256,
        512,
        512,
    ).as_bytes());

    std::thread::sleep(std::time::Duration::from_millis(100));

    socket.set_nonblocking(true);
    let mut buf = [0 as u8; 128];
    match socket.read(&mut buf) {
        Ok(count) => {
            println!("received: {:?}", &buf[..count]);
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            println!("Ok! No data yet");
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }

    let mut x = String::new();
    std::io::stdin().read_line(&mut x);
}
