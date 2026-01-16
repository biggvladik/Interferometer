use std::ffi::{c_void, c_char, CStr};
use std::mem;
use std::ptr;



/// Структура для устройств, подключенных к пк
#[repr(C)]
#[derive(Debug, Clone)]
pub struct USMC_Devices {
    pub NOD: u32,
    pub Serial: *mut *mut c_char,
    pub Version: *mut *mut c_char,
}

impl Default for USMC_Devices {
    fn default() -> Self {
        Self {
        NOD: 0,
        Serial: ptr::null_mut(),
        Version: ptr::null_mut(),
        }
    }
}

impl USMC_Devices {
    // Получаем серийный номер по индексу
    pub fn get_serial(&self, index: usize) -> Option<String> {
        if index >= self.NOD as usize || self.Serial.is_null() {
            return None;
        }
        unsafe {
            let serial_ptr = *self.Serial.add(index);
            if serial_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(serial_ptr).to_string_lossy().into_owned())

            }
        }
    }
    // Получаем версию по индексу
    pub fn get_version(&self, index: usize) -> Option<String> {
        if index >= self.NOD as usize || self.Version.is_null() {
            return None;
        }

        unsafe {
            let version_ptr = *self.Version.add(index);
            if version_ptr.is_null() {
                None
            } else {
                Some(CStr::from_ptr(version_ptr).to_string_lossy().into_owned())
            }
        }
    }






}