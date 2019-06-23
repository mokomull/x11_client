use x11_client::*;

use std::io::prelude::*;
use std::os::unix::net::UnixStream;

fn main() {
    let mut socket = UnixStream::connect("/tmp/.X11-unix/X0").unwrap();

    let client_init: Vec<_> = ClientInit::new().into();
    socket.write_all(&client_init).unwrap();

    let server_response = ServerInit::from_stream(&mut socket).unwrap();

    println!("major: {}", server_response.major);
    println!("minor: {}", server_response.minor);
    println!("release_number: {}", server_response.release_number);
    println!("resource_id_base: {}", server_response.resource_id_base);
    println!("resource_id_mask: {}", server_response.resource_id_mask);
    println!("motion_buffer_size: {}", server_response.motion_buffer_size);
    println!(
        "maximum_request_length: {}",
        server_response.maximum_request_length
    );
    println!("image_byte_order: {}", server_response.image_byte_order);
    println!(
        "bitmap_format_bit_order: {}",
        server_response.bitmap_format_bit_order
    );
    println!(
        "bitmap_format_scanline_unit: {}",
        server_response.bitmap_format_scanline_unit
    );
    println!(
        "bitmap_format_scanline_pad: {}",
        server_response.bitmap_format_scanline_pad
    );
    println!("min_keycode: {}", server_response.min_keycode);
    println!("max_keycode: {}", server_response.max_keycode);
    println!("vendor: {}", server_response.vendor);

    println!("pixmap formats:");
    for fmt in server_response.pixmap_formats {
        println!("\tdepth: {}", fmt.depth);
        println!("\tbits_per_pixel: {}", fmt.bits_per_pixel);
        println!("\tscanline_pad: {}", fmt.scanline_pad);
    }

    println!("screens:");
    for screen in server_response.roots {
        println!("\troot: {}", screen.root);
        println!("\tdefault_colormap: {}", screen.default_colormap);
        println!("\twhite_pixel: {}", screen.white_pixel);
        println!("\tblack_pixel: {}", screen.black_pixel);
        println!("\tcurrent_input_masks: {}", screen.current_input_masks);
        println!("\twidth_pixels: {}", screen.width_pixels);
        println!("\theight_pixels: {}", screen.height_pixels);
        println!("\twidth_millimeters: {}", screen.width_millimeters);
        println!("\theight_millimeters: {}", screen.height_millimeters);
        println!("\tmin_installed_maps: {}", screen.min_installed_maps);
        println!("\tmax_installed_maps: {}", screen.max_installed_maps);
        println!("\troot_visual: {}", screen.root_visual);
        println!("\tbacking_stores: {}", screen.backing_stores);
        println!("\tsave_unders: {}", screen.save_unders);
        println!("\troot_depth: {}", screen.root_depth);

        println!("\tdepths:");
        for depth in screen.allowed_depths {
            println!("\t\tdepth: {}", depth.depth);
            println!("\t\tvisuals:");

            for visual in depth.visuals {
                println!("\t\t\tvisual_id: {}", visual.id);
                println!("\t\t\tclass: {}", visual.class);
                println!("\t\t\tbits_per_rgb_value: {}", visual.bits_per_rgb_value);
                println!("\t\t\tcolormap_entries: {}", visual.colormap_entries);
                println!("\t\t\tred_mask: {}", visual.red_mask);
                println!("\t\t\tgreen_mask: {}", visual.green_mask);
                println!("\t\t\tblue_mask: {}", visual.blue_mask);
                println!();
            }
        }
    }

    socket.set_nonblocking(true).unwrap();
    let mut buf = [0 as u8; 1];
    let result = socket.read(&mut buf);
    match result {
        Ok(_) => {
            panic!("Did not retrieve all data from server");
        }
        Err(e) => match e.kind() {
            std::io::ErrorKind::WouldBlock => {
                println!("Would block!  Ok!");
            }
            _ => {
                panic!("Some other kind of error: {:?}", e);
            }
        },
    }
}
