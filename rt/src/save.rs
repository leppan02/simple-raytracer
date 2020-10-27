extern crate image;
use super::structs::Picture;
pub fn save(obj: &Picture) {
    let mut imgbuf = image::ImageBuffer::new(obj.size.0, obj.size.1);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (obj.data[y as usize][x as usize].val().0) as u8;
        let g = (obj.data[y as usize][x as usize].val().1) as u8;
        let b = (obj.data[y as usize][x as usize].val().2) as u8;
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("image.png").unwrap();
}
