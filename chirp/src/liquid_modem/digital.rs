use crate::liquid_modem::{
    complex::Complex,
    error::{ModemError, ModemResult},
};
use liquid_dsp_sys::ffi;
use std::ptr::NonNull;

struct DigitalModem {
    modem: NonNull<ffi::modemcf_s>,
}

impl DigitalModem {
    pub fn new() -> Self {
        let modem = unsafe {
            let modem = ffi::modemcf_create(ffi::modulation_scheme_LIQUID_MODEM_ASK2);
            NonNull::new(modem).expect("Failed to create modem.")
        };
        DigitalModem { modem }
    }

    // symbol must be 0 or 1, representing bit values
    pub fn modulate(&self, symbol: u32, complex: &mut Complex) {
        unsafe {
            let output = complex.as_mut_ptr();
            ffi::modemcf_modulate(self.modem.as_ptr(), symbol, output);
        }
    }
}

impl Drop for DigitalModem {
    fn drop(&mut self) {
        unsafe { ffi::modemcf_destroy(self.modem.as_ptr()) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_digital_modem_new() {
        let modem = DigitalModem::new();
    }
}
