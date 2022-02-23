use std::fmt;

/// Four character code (fourcc) identifying a pixel format.
///
/// fourcc codes are documented on <https://www.fourcc.org/>.
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Pixelformat(u32);

impl Pixelformat {
    /// `rrrrrrrr gggggggg bbbbbbbb aaaaaaaa`
    pub const RGBA32: Self = fmt(b"AB24");
    /// `aaaaaaaa rrrrrrrr gggggggg bbbbbbbb`
    pub const RGB32: Self = fmt(b"RGB4");

    /// Motion JPEG, a sequence of JPEG images with omitted huffman tables.
    ///
    /// The transmitted JPEG images lack the "DHT" frame (Define Huffman Table), and instead use a
    /// predefined one.
    pub const MJPG: Self = fmt(b"MJPG");

    /// Data is a sequence of regular JPEG still images.
    ///
    /// Images can be decoded with any off-the-shelf JPEG decoder, no preprocessing is needed.
    pub const JPEG: Self = fmt(b"JPEG");

    /// UVC payload header metadata.
    ///
    /// Data is a stream of [`UvcMetadata`][crate::uvc::UvcMetadata] structures.
    pub const UVC: Self = fmt(b"UVCH");

    /// `yyyyyyyy uuuuuuuu YYYYYYYY vvvvvvvv`
    ///
    /// Packed YUV/YCbCr data with 4:2:2 chroma subsampling in the horizontal direction.
    ///
    /// `uuuuuuuu` and `vvvvvvvv` are shared by 2 neighboring pixels, while `yyyyyyyy` is the left
    /// pixel's Y value, and `YYYYYYYY` is the right pixel's Y value.
    pub const YUYV: Self = fmt(b"YUYV");

    pub const fn from_fourcc(fourcc: &[u8; 4]) -> Self {
        Self(u32::from_le_bytes(*fourcc))
    }
}

// Just a shorthand for `Pixelformat::from_fourcc`.
const fn fmt(fourcc: &[u8; 4]) -> Pixelformat {
    Pixelformat::from_fourcc(fourcc)
}

impl fmt::Display for Pixelformat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b, c, d] = self.0.to_le_bytes();
        let [a, b, c, d] = [a as char, b as char, c as char, d as char];
        write!(f, "{}{}{}{}", a, b, c, d)
    }
}

impl fmt::Debug for Pixelformat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(Pixelformat::RGBA32.to_string(), "AB24");
    }
}
