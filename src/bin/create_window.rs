use x11_client::*;

use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() {
    let mut socket = UnixStream::connect("/tmp/.X11-unix/X0").unwrap();

    let client_init: Vec<_> = ClientInit::new().into();
    socket.write_all(&client_init).unwrap();

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
    socket.write_all(&create_window.as_bytes()).unwrap();

    socket
        .write_all(&MapWindow::new(server_init.resource_id_base + 1).as_bytes())
        .unwrap();

    socket
        .write_all(
            &CreateGc::new(
                server_init.resource_id_base + 2,
                server_init.resource_id_base + 1,
                0x00_00_FF,
            )
            .as_bytes(),
        )
        .unwrap();

    socket
        .write_all(
            &ChangeWmName::new(
                server_init.resource_id_base + 1,
                "holy crap that worked".into(),
            )
            .as_bytes(),
        )
        .unwrap();

    loop {
        let mut buf = [0 as u8; 32];
        socket.read_exact(&mut buf).unwrap();

        let event = Event::from_bytes(&buf);
        println!("event: {:?}", event);

        if let Event::Expose { .. } = event {
            socket
                .write_all(
                    &PolyFillRectangle::new(
                        server_init.resource_id_base + 1,
                        server_init.resource_id_base + 2,
                        256,
                        256,
                        512,
                        512,
                    )
                    .as_bytes(),
                )
                .unwrap();
        }
    }
}
