pub struct SVG {
    bytes: Vec<u8>,
    size: (usize, usize)
}

impl SVG {
    pub fn new(bytes: Vec<u8>, size: (usize, usize)) -> Self {
        return Self {
            bytes: bytes,
            size: size
        };
    }
}