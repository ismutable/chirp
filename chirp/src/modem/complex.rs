use liquid_dsp_sys::ffi;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex(ffi::liquid_float_complex);

impl Complex {
    pub fn new(re: f32, im: f32) -> Self {
        Self(ffi::liquid_float_complex { re, im })
    }
    pub fn re(&self) -> f32 {
        self.0.re
    }
    pub fn im(&self) -> f32 {
        self.0.im
    }

    pub fn as_ptr(&self) -> *const ffi::liquid_float_complex {
        &self.0
    }

    pub fn as_mut_ptr(&mut self) -> *mut ffi::liquid_float_complex {
        &mut self.0
    }
}

impl From<ffi::liquid_float_complex> for Complex {
    fn from(val: ffi::liquid_float_complex) -> Self {
        Self(val)
    }
}
