use std::io::Read;

#[derive(Debug, Clone, Copy)]
pub enum ColorMapType {
    NoColorMap,
    Present,
    Reserved,
    DeveloperUse,
}

#[derive(Debug, Clone, Copy)]
pub enum ImageType {
    NoImageData = 0b0000,
    UncompressedColorMapped = 0b0001,
    UncompressedTrueColor = 0b0010,
    UncompressedBlackAndWhite = 0b0011,
    RunLengthColorMapped = 0b1001,
    RunLengthTrueColor = 0b1010,
    RunLengthBlackAndWhite = 0b1011,
}

#[derive(Debug, Clone, Copy)]
pub struct ImgSpec {
    x_origin: u16,
    y_origin: u16,
    width: u16,
    height: u16,
    pixel_depth: u8,
    alpha_depth: u8,
    pixels_right_to_left: bool,
    pixels_top_to_bottom: bool,
}

#[derive(Debug, Clone)]
pub struct TgaImage {
    color_map_type: ColorMapType,
    image_type: ImageType,
    img_spec: ImgSpec,

    image_data: Vec<u8>,
    color_map: Vec<u8>,
    image_id: Vec<u8>,
}

pub fn read(r: &mut impl Read) -> std::io::Result<TgaImage> {
    let mut id_len = [0; 1];
    r.read_exact(&mut id_len)?;
    let id_len = u8::from_le_bytes(id_len);

    let mut color_map_type = [0; 1];
    r.read_exact(&mut color_map_type)?;
    let color_map_type = u8::from_le_bytes(color_map_type);

    let mut image_type = [0; 1];
    r.read_exact(&mut image_type)?;
    let image_type = u8::from_le_bytes(image_type);

    let mut first_entry_index = [0; 2];
    r.read_exact(&mut first_entry_index)?;
    let first_entry_index = u16::from_le_bytes(first_entry_index);

    let mut color_map_length = [0; 2];
    r.read_exact(&mut color_map_length)?;
    let color_map_length = u16::from_le_bytes(color_map_length);

    let mut color_map_entry_size = [0; 1];
    r.read_exact(&mut color_map_entry_size)?;
    let color_map_entry_size = u8::from_le_bytes(color_map_entry_size);

    let mut x_origin = [0; 2];
    r.read_exact(&mut x_origin)?;
    let x_origin = u16::from_le_bytes(x_origin);

    let mut y_origin = [0; 2];
    r.read_exact(&mut y_origin)?;
    let y_origin = u16::from_le_bytes(y_origin);

    let mut width = [0; 2];
    r.read_exact(&mut width)?;
    let width = u16::from_le_bytes(width);

    let mut height = [0; 2];
    r.read_exact(&mut height)?;
    let height = u16::from_le_bytes(height);

    let mut pixel_depth = [0; 1];
    r.read_exact(&mut pixel_depth)?;
    let pixel_depth = u8::from_le_bytes(pixel_depth);

    let mut image_descriptor = [0; 1];
    r.read_exact(&mut image_descriptor)?;
    let image_descriptor = u8::from_le_bytes(image_descriptor);

    let mut image_id = vec![0; id_len as usize];
    r.read_exact(&mut image_id)?;

    let mut color_map = vec![0; color_map_length as usize * color_map_entry_size as usize];
    r.read_exact(&mut color_map)?;

    let bytes_per_pixel = pixel_depth / 8;
    let mut image_data = vec![0; width as usize * height as usize * bytes_per_pixel as usize];
    r.read_exact(&mut image_data)?;

    Ok(TgaImage {
        color_map_type: match color_map_type {
            0 => ColorMapType::NoColorMap,
            1 => ColorMapType::Present,
            2..=127 => ColorMapType::Reserved,
            128..=255 => ColorMapType::DeveloperUse,
        },

        image_type: match image_type {
            0 => ImageType::NoImageData,
            1 => ImageType::UncompressedColorMapped,
            2 => ImageType::UncompressedTrueColor,
            3 => ImageType::UncompressedBlackAndWhite,
            9 => ImageType::RunLengthColorMapped,
            10 => ImageType::RunLengthTrueColor,
            11 => ImageType::RunLengthBlackAndWhite,

            _ => {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Invalid image type",
                ))
            }
        },

        img_spec: ImgSpec {
            pixels_right_to_left: image_descriptor & 0x10 != 0,
            pixels_top_to_bottom: image_descriptor & 0x20 != 0,
            alpha_depth: image_descriptor & 0x0f,
            pixel_depth,
            x_origin,
            y_origin,
            width,
            height,
        },

        image_data,
        color_map,
        image_id,
    })
}
