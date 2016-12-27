#[cfg(test)]
mod tests;

extern crate byteorder;

use std::io::{Read, Write, Result};

pub struct ClientInit<'a> {
    major: u16,
    minor: u16,
    authorization_protocol_name: Option<&'a [u8]>,
    authorization_protocol_data: Option<&'a [u8]>,
}

impl<'a> ClientInit<'a> {
    pub fn new() -> Self {
        ClientInit {
            major: 11,
            minor: 0,
            authorization_protocol_name: None,
            authorization_protocol_data: None,
        }
    }
}

impl<'a> Into<Vec<u8>> for ClientInit<'a> {
    fn into(self: Self) -> Vec<u8> {
        use std::io::Write;
        use byteorder::{BigEndian, WriteBytesExt};

        let mut bytes = [0 as u8; 2];
        let mut ret = Vec::new();

        ret.write(b"B\x00");
        ret.write_u16::<BigEndian>(self.major);
        ret.write_u16::<BigEndian>(self.minor);
        assert!(self.authorization_protocol_name.is_none());
        assert!(self.authorization_protocol_data.is_none());
        ret.write_u16::<BigEndian>(0);
        ret.write_u16::<BigEndian>(0);
        // the unused data needs to be sent, too.
        ret.write_u16::<BigEndian>(0);
        ret
    }
}

pub struct ServerInit {
    pub major: u16,
    pub minor: u16,
    pub release_number: u32,
    pub resource_id_base: u32,
    pub resource_id_mask: u32,
    pub motion_buffer_size: u32,
    pub maximum_request_length: u16,
    pub image_byte_order: u8,
    pub bitmap_format_bit_order: u8,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub vendor: String,
    pub pixmap_formats: Vec<PixmapFormat>,
    pub roots: Vec<Screen>,
}

impl ServerInit {
    pub fn from_stream<T: Read>(stream: &mut T) -> Result<ServerInit> {
        use byteorder::{BigEndian, ReadBytesExt};

        let success = stream.read_u8()?;
        stream.read_u8()?;
        let major = stream.read_u16::<BigEndian>()?;
        let minor = stream.read_u16::<BigEndian>()?;
        stream.read_u16::<BigEndian>()?;
        let release_number = stream.read_u32::<BigEndian>()?;
        let resource_id_base = stream.read_u32::<BigEndian>()?;
        let resource_id_mask = stream.read_u32::<BigEndian>()?;
        let motion_buffer_size = stream.read_u32::<BigEndian>()?;
        let vendor_len = stream.read_u16::<BigEndian>()?;
        let maximum_request_length = stream.read_u16::<BigEndian>()?;
        let screen_count = stream.read_u8()?;
        let pixmap_format_count = stream.read_u8()?;
        let image_byte_order = stream.read_u8()?;
        let bitmap_format_bit_order = stream.read_u8()?;
        let bitmap_format_scanline_unit = stream.read_u8()?;
        let bitmap_format_scanline_pad = stream.read_u8()?;
        let min_keycode = stream.read_u8()?;
        let max_keycode = stream.read_u8()?;
        stream.read_u32::<BigEndian>()?;

        let mut vendor_bytes = vec![0; vendor_len as usize];
        stream.read_exact(&mut vendor_bytes)?;
        let padding = (4 - (vendor_len % 4)) % 4;
        for _ in 0..padding {
            stream.read_u8()?;
        }

        let mut pixmap_formats = Vec::new();
        for _ in 0..pixmap_format_count {
            pixmap_formats.push(PixmapFormat::from_stream(stream)?);
        }

        let mut roots = Vec::new();
        for _ in 0..screen_count {
            roots.push(Screen::from_stream(stream)?);
        }

        Ok(ServerInit {
            major: major,
            minor: minor,
            release_number: release_number,
            resource_id_base: resource_id_base,
            resource_id_mask: resource_id_mask,
            motion_buffer_size: motion_buffer_size,
            maximum_request_length: maximum_request_length,
            image_byte_order: image_byte_order,
            bitmap_format_bit_order: bitmap_format_bit_order,
            bitmap_format_scanline_unit: bitmap_format_scanline_unit,
            bitmap_format_scanline_pad: bitmap_format_scanline_pad,
            min_keycode: min_keycode,
            max_keycode: max_keycode,

            pixmap_formats: pixmap_formats,
            roots: roots,
            vendor: String::from_utf8(vendor_bytes).unwrap(),
        })
    }
}

pub struct PixmapFormat {
    pub depth: u8,
    pub bits_per_pixel: u8,
    pub scanline_pad: u8,
}

impl PixmapFormat {
    pub fn from_stream<T: Read>(stream: &mut T) -> Result<Self> {
        use byteorder::ReadBytesExt;
        let depth = stream.read_u8()?;
        let bits_per_pixel = stream.read_u8()?;
        let scanline_pad = stream.read_u8()?;
        stream.read_exact(&mut [0; 5])?;

        Ok(PixmapFormat{
            depth: depth,
            bits_per_pixel: bits_per_pixel,
            scanline_pad: scanline_pad,
        })
    }
}

pub struct Screen {
    pub root: u32,
    pub default_colormap: u32,
    pub white_pixel: u32,
    pub black_pixel: u32,
    pub current_input_masks: u32,
    pub width_pixels: u16,
    pub height_pixels: u16,
    pub width_millimeters: u16,
    pub height_millimeters: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: u32,
    pub backing_stores: u8, // TODO: enum
    pub save_unders: bool,
    pub root_depth: u8,
    pub allowed_depths: Vec<Depth>,
}

impl Screen {
    pub fn from_stream<T: Read>(stream: &mut T) -> Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        let root = stream.read_u32::<BigEndian>()?;
        let default_colormap = stream.read_u32::<BigEndian>()?;
        let white_pixel = stream.read_u32::<BigEndian>()?;
        let black_pixel = stream.read_u32::<BigEndian>()?;
        let current_input_masks = stream.read_u32::<BigEndian>()?;
        let width_pixels = stream.read_u16::<BigEndian>()?;
        let height_pixels = stream.read_u16::<BigEndian>()?;
        let width_millimeters = stream.read_u16::<BigEndian>()?;
        let height_millimeters = stream.read_u16::<BigEndian>()?;
        let min_installed_maps = stream.read_u16::<BigEndian>()?;
        let max_installed_maps = stream.read_u16::<BigEndian>()?;
        let root_visual = stream.read_u32::<BigEndian>()?;
        let backing_stores = stream.read_u8()?;
        let save_unders = stream.read_u8()? != 0;
        let root_depth = stream.read_u8()?;
        let depth_count = stream.read_u8()?;

        let mut allowed_depths = Vec::new();
        for _ in 0..depth_count {
            allowed_depths.push(Depth::from_stream(stream)?);
        }

        Ok(Screen {
            root: root,
            default_colormap: default_colormap,
            white_pixel: white_pixel,
            black_pixel: black_pixel,
            current_input_masks: current_input_masks,
            width_pixels: width_pixels,
            height_pixels: height_pixels,
            width_millimeters: width_millimeters,
            height_millimeters: height_millimeters,
            min_installed_maps: min_installed_maps,
            max_installed_maps: max_installed_maps,
            root_visual: root_visual,
            backing_stores: backing_stores,
            save_unders: save_unders,
            root_depth: root_depth,
            allowed_depths: allowed_depths,
        })
    }
}

pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<Visual>,
}

impl Depth {
    pub fn from_stream<T: Read>(stream: &mut T) -> Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        let depth = stream.read_u8()?;
        stream.read_u8()?;
        let visual_count = stream.read_u16::<BigEndian>()?;
        stream.read_u32::<BigEndian>()?;

        let mut visuals = Vec::new();
        for _ in 0..visual_count {
            visuals.push(Visual::from_stream(stream)?);
        }

        Ok(Depth {
            depth: depth,
            visuals: visuals,
        })
    }
}

pub struct Visual {
    pub id: u32,
    pub class: u8, // TODO: enum
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
}

impl Visual {
    pub fn from_stream<T: Read>(stream: &mut T) -> Result<Self> {
        use byteorder::{BigEndian, ReadBytesExt};
        let id = stream.read_u32::<BigEndian>()?;
        let class = stream.read_u8()?;
        let bits_per_rgb_value = stream.read_u8()?;
        let colormap_entries = stream.read_u16::<BigEndian>()?;
        let red_mask = stream.read_u32::<BigEndian>()?;
        let green_mask = stream.read_u32::<BigEndian>()?;
        let blue_mask = stream.read_u32::<BigEndian>()?;
        stream.read_u32::<BigEndian>()?;

        Ok(Visual {
            id: id,
            class: class,
            bits_per_rgb_value: bits_per_rgb_value,
            colormap_entries: colormap_entries,
            red_mask: red_mask,
            green_mask: green_mask,
            blue_mask: blue_mask,
        })
    }
}

pub struct CreateWindow {
    wid: u32,
    parent: u32,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    border_width: u16,
    class: u16,
    visual: u32,
    depth: u8,
    // TODO: Option soup for values
}

impl CreateWindow {
    pub fn new(
            depth: u8, wid: u32, parent: u32, x: u16, y: u16,
            width: u16, height: u16, border_width: u16, class: u16,
            visual: u32) -> Self {
        CreateWindow {
            depth: depth,
            wid: wid,
            parent: parent,
            x: x,
            y: y,
            width: width,
            height: height,
            border_width: border_width,
            class: class,
            visual: visual,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        use byteorder::{BigEndian, WriteBytesExt};
        let mut ret = Vec::new();

        ret.write_u8(1);
        ret.write_u8(self.depth);
        ret.write_u16::<BigEndian>(8); // TODO: length
        ret.write_u32::<BigEndian>(self.wid);
        ret.write_u32::<BigEndian>(self.parent);
        ret.write_u16::<BigEndian>(self.x);
        ret.write_u16::<BigEndian>(self.y);
        ret.write_u16::<BigEndian>(self.width);
        ret.write_u16::<BigEndian>(self.height);
        ret.write_u16::<BigEndian>(self.border_width);
        ret.write_u16::<BigEndian>(self.class);
        ret.write_u32::<BigEndian>(self.visual);
        ret.write_u32::<BigEndian>(0); // TODO: value-mask and value-list

        ret
    }
}

pub struct MapWindow {
    window: u32,
}

impl MapWindow {
    pub fn new(window: u32) -> Self {
        MapWindow { window: window }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        use byteorder::{BigEndian, WriteBytesExt};
        let mut ret = Vec::new();

        ret.write_u8(8);
        ret.write_u8(0);
        ret.write_u16::<BigEndian>(2);
        ret.write_u32::<BigEndian>(self.window);

        ret
    }
}

pub struct CreateGc {
    cid: u32,
    drawable: u32,
    foreground: u32,
}

impl CreateGc {
    pub fn new(cid: u32, drawable: u32, foreground: u32) -> Self {
        CreateGc {
            cid: cid,
            drawable: drawable,
            foreground: foreground,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        use byteorder::{BigEndian, WriteBytesExt};
        let mut ret = Vec::new();

        ret.write_u8(55);
        ret.write_u8(0);
        ret.write_u16::<BigEndian>(5);
        ret.write_u32::<BigEndian>(self.cid);
        ret.write_u32::<BigEndian>(self.drawable);
        ret.write_u32::<BigEndian>(0x04);
        ret.write_u32::<BigEndian>(self.foreground);

        ret
    }
}

pub struct PolyFillRectangle {
    drawable: u32,
    gc: u32,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
}

impl PolyFillRectangle {
    pub fn new(drawable: u32, gc: u32, x: i16, y: i16,
            width: u16, height: u16) -> Self {
        PolyFillRectangle {
            drawable: drawable,
            gc: gc,
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        use byteorder::{BigEndian, WriteBytesExt};
        let mut ret = Vec::new();

        ret.write_u8(70);
        ret.write_u8(0);
        ret.write_u16::<BigEndian>(5);
        ret.write_u32::<BigEndian>(self.drawable);
        ret.write_u32::<BigEndian>(self.gc);
        ret.write_i16::<BigEndian>(self.x);
        ret.write_i16::<BigEndian>(self.y);
        ret.write_u16::<BigEndian>(self.width);
        ret.write_u16::<BigEndian>(self.height);

        ret
    }
}
