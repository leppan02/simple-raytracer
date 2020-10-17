extern crate image;
use super::structs;
pub fn save(obj: structs::picture) {
    let mut imgbuf = image::ImageBuffer::new(obj.x, obj.y);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (x / 12) as u8;
        let b = (y / 12) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }

    imgbuf.save("fractal.png").unwrap();
}
