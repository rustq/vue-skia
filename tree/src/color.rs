// Note: SkColor _is_ a u32, and therefore its components are
// endian dependent, so we can't expose it as (transmuted) individual
// argb fields.
type SkColor = u32;
type U8CPU = ::std::os::raw::c_uint;
type u8cpu = U8CPU;

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
#[repr(transparent)]
pub struct Color(SkColor);

// native_transmutable!(SkColor, Color, color_layout);

impl From<u32> for Color {
    fn from(argb: u32) -> Self {
        Color::new(argb)
    }
}


impl Color {
    pub const fn new(argb: u32) -> Self {
        Self(argb)
    }

    // note: we don't use the u8cpu type in the arguments here, because we trust the Rust
    // compiler to optimize the storage type.
    pub const fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Color {
        Self(((a as u8cpu) << 24) | ((r as u8cpu) << 16) | ((g as u8cpu) << 8) | (b as u8cpu))
    }


    pub fn a(self) -> u8 {
        (self.into_native() >> 24) as _
    }

    pub fn r(self) -> u8 {
        (self.into_native() >> 16) as _
    }

    pub fn g(self) -> u8 {
        (self.into_native() >> 8) as _
    }

    pub fn b(self) -> u8 {
        self.into_native() as _
    }

    pub fn set_a(mut self, a: u8) {
        self.0 = ((a as u8cpu) << 24) | ((self.r() as u8cpu) << 16) | ((self.g() as u8cpu) << 8) | (self.b() as u8cpu);
    }

    fn into_native(self) -> u8cpu {
        let r = unsafe { std::mem::transmute_copy(&self) };
        // don't drop, the native type takes over.
        std::mem::forget(self);
        r
    }
}
