use liquid_dsp_sys::ffi;
use crate::modem::error::{ModemResult, ModemError};


pub type ComplexScalar = ffi::liquid_float_complex;

struct ComplexBuffer<N: const> {
    buf: [ComplexScalar; N]
}

/// Complex Buffer
/// 
/// This buffer abstracts the handling of liquid dsp's
/// interal complex values type. API is minimal allowing
/// only creation and slice references. Size is set
/// at compile time.
impl <N: const>ComplexBuffer<N> {
    pub fn new() -> Self {
        let zero = ComplexScalar {
            re: 0.0,
            im: 0.0
        };
        Self { 
            buf: [zero; N]
        }
    }

    pub fn as_slice(&self) -> &[ComplexScalar] {
        self.buf.as_slice()
    }

    pub fn as_mut_slice(&mut self) -> &mut[ComplexScalar] {
        self.buf.as_mut_slice()
    }
}

struct DigitalModem {
    modem: ffi::modemcf,
}

impl DigitalModem {
    pub fn try_new() -> ModemResult<Self> {
        let modem = unsafe {
            let modem = ffi::modemcf_create(ffi::modulation_scheme_LIQUID_MODEM_ASK2);
            if modem.is_null() {
                return Err(ModemError::CreationError);
            }
            modem
        };
        Ok(DigitalModem { modem })
    }

    pub fn modulate(&self, output: &mut[ComplexScalar], data: &[u8]) -> ModemResult<()> {
        unsafe {
            let a = ffi::liquid_float_complex;
        }
        
        let mut output = vec![0.0; data.len() * 2]; // Assuming 2 samples per byte
        let mut output_len = output.len();
        let result = unsafe {
            ffi::modemcf_modulate(self.modem, data.as_ptr(), data.len(), output.as_mut_ptr(), &mut output_len)
        };
        if result != 0 {
            return Err(ModemError::ModulationError);
        }
        output.truncate(output_len);
        Ok(output)
    }
}

impl Drop for DigitalModem {
    fn drop(&mut self) {
        unsafe { ffi::modemcf_destroy(self.modem) };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_digital_modem_try_new() {
        let modem = DigitalModem::try_new();
        assert!(modem.is_ok(), "Failed to create DigitalModem: {:?}", modem.err());
    }
}