pub struct picture {
    pub x: u32,
    pub y: u32,
    xcc: usize,
    ycc: usize,
    pub data: Vec<Vec<pixel>>,
}
pub struct values {
    pub x: u32,
    pub y: u32,
    xcc: usize,
    ycc: usize,
    pub data: Vec<Vec<pixel>>,
}
pub struct pixel {
    rgb: [u8; 3],
}

impl Iterator for picture {
    type Item = [u8; 3];
    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.data[self.xcc][self.ycc];
        self.xcc += 1;
        if xcc == 
        self.xcc = 0;<>
    }
}
