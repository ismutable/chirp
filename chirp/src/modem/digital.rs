use liquid_dsp_sys::ffi;
use crate::modem::error::{ModemResult, ModemError};
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
}

impl Drop for DigitalModem {
    fn drop(&mut self) {
        unsafe { ffi::modemcf_destroy(self.modem) };
    }
}