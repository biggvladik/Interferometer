#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::c_char;
use std::sync::Arc;
use std::sync::Mutex;
use libloading::{Library, Symbol};

use super::types::*;


pub struct USMCDLL {
    library: Arc<Library>,
    initialized: bool,
}

type USMC_InitFn = unsafe extern "C" fn(devices: *mut USMC_Devices) -> u32;
type USMC_DevicesFn = unsafe extern "C" fn(devices: *mut USMC_Devices) -> u32;
type USMC_GetStateFn = unsafe extern "C" fn(device: u32, state: *mut USMC_State) -> u32;
/// type USMC_StateFn = unsafe extern "C" fn(device: u32, state: *mut USMC_State) -> u32;
type USMC_SaveParametersToFlashFn = unsafe extern "C" fn(device: u32) -> u32;
type USMC_SetCurrentPositionFn = unsafe extern "C" fn(device: u32, position: i32) -> u32;
type USMC_GetModeFn = unsafe extern "C" fn(device: u32, mode: *mut USMC_Mode) -> u32;
type USMC_SetModeFn = unsafe extern "C" fn(device: u32, mode: *mut USMC_Mode) -> u32;
type USMC_GetParametersFn = unsafe extern "C" fn(device: u32, params: *mut USMC_Parameters) -> u32;
type USMC_SetParametersFn = unsafe extern "C" fn(device: u32, params: *mut USMC_Parameters) -> u32;
type USMC_GetStartParametersFn = unsafe extern "C" fn(device: u32, params: *mut USMC_StartParameters) -> u32;
type USMC_StartFn = unsafe extern "C" fn(device: u32, dest_pos: i32, speed: *mut f32, params: *mut USMC_StartParameters) -> u32;
type USMC_StopFn = unsafe extern "C" fn(device: u32) -> u32;
type USMC_GetEncoderStateFn = unsafe extern "C" fn(device: u32, encoder_state: *mut USMC_EncoderState) -> u32;
type USMC_CloseFn = unsafe extern "C" fn() -> u32;
type USMC_GetLastErrFn = unsafe extern "C" fn(buffer: *mut c_char, length: usize);

impl USMCDLL {

    pub fn new() -> Result<Self,Box<dyn std::error::Error>> {

        let library = unsafe {
            Library::new("USMCDLL.dll").map_err(|e| format!("Failed to load USMCDLL.dll: {}", e))?
        };
        
        Ok(Self {
            library: Arc::new(library),
            initialized: false,
        })
    }
    
    pub fn init(&mut self, devices: &mut USMC_Devices) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_InitFn> = unsafe {
            self.library.get(b"USMC_Init")?
        };
        
        let result = unsafe { func(devices) };
        if result == USMC_SUCCESS {
            self.initialized = true;
        }
        
        Ok(result)
    }

    pub fn get_devices(&self, devices: &mut USMC_Devices) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_DevicesFn> = unsafe {
            self.library.get(b"USMC_Devices")?
        };
        
        Ok(unsafe { func(devices) })
    }

    pub fn get_state(&self, device: u32, state: &mut USMC_State) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_GetStateFn> = unsafe {
            self.library.get(b"USMC_GetState")?
        };
        
        Ok(unsafe { func(device, state) })
    }


    pub fn save_parameters_to_flash(&self, device: u32) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_SaveParametersToFlashFn> = unsafe {
            self.library.get(b"USMC_SaveParametersToFlash")?
        };
        
        Ok(unsafe { func(device) })
    }

    pub fn set_current_position(&self, device: u32, position: i32) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_SetCurrentPositionFn> = unsafe {
            self.library.get(b"USMC_SetCurrentPosition")?
        };
        
        Ok(unsafe { func(device, position) })
    }

    pub fn get_mode(&self, device: u32, mode: &mut USMC_Mode) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_GetModeFn> = unsafe {
            self.library.get(b"USMC_GetMode")?
        };
        
        Ok(unsafe { func(device, mode) })
    }

    pub fn set_mode(&self, device: u32, mode: &mut USMC_Mode) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_SetModeFn> = unsafe {
            self.library.get(b"USMC_SetMode")?
        };
        
        Ok(unsafe { func(device, mode) })
    }

    pub fn get_parameters(&self, device: u32, params: &mut USMC_Parameters) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_GetParametersFn> = unsafe {
            self.library.get(b"USMC_GetParameters")?
        };
        
        Ok(unsafe { func(device, params) })
    }

    pub fn set_parameters(&self, device: u32, params: &mut USMC_Parameters) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_SetParametersFn> = unsafe {
            self.library.get(b"USMC_SetParameters")?
        };
        
        Ok(unsafe { func(device, params) })
    }

    pub fn get_start_parameters(&self, device: u32, params: &mut USMC_StartParameters) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_GetStartParametersFn> = unsafe {
            self.library.get(b"USMC_GetStartParameters")?
        };
        
        Ok(unsafe { func(device, params) })
    }

    pub fn start(&self, device: u32, dest_pos: i32, speed: &mut f32, params: &mut USMC_StartParameters) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_StartFn> = unsafe {
            self.library.get(b"USMC_Start")?
        };
        
        Ok(unsafe { func(device, dest_pos, speed, params) })
    }

    pub fn stop(&self, device: u32) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_StopFn> = unsafe {
            self.library.get(b"USMC_Stop")?
        };
        
        Ok(unsafe { func(device) })
    }

    pub fn get_encoder_state(&self, device: u32, encoder_state: &mut USMC_EncoderState) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_GetEncoderStateFn> = unsafe {
            self.library.get(b"USMC_GetEncoderState")?
        };
        
        Ok(unsafe { func(device, encoder_state) })
    }

    pub fn close(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let func: Symbol<USMC_CloseFn> = unsafe {
            self.library.get(b"USMC_Close")?
        };
        
        Ok(unsafe { func() })
    }

    pub fn get_last_error(&self, buffer: &mut [u8]) -> Result<(), Box<dyn std::error::Error>> {
        let func: Symbol<USMC_GetLastErrFn> = unsafe {
            self.library.get(b"USMC_GetLastErr")?
        };
        
        unsafe {
            func(buffer.as_mut_ptr() as *mut c_char, buffer.len());
        }
        
        Ok(())
    }
    
    /// Проверяет, инициализирована ли библиотека
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }
}

impl Drop for USMCDLL {
    fn drop(&mut self) {
        // При уничтожении объекта закрываем библиотеку
        if self.initialized {
            let _ = self.close();
        }
    }
}



pub struct USMCController {
    dll: Mutex<USMCDLL>,
    device_count: usize,
}

impl USMCController {

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let dll = USMCDLL::new()?;
        
        Ok(Self {
            dll: Mutex::new(dll),
            device_count: 0,
        })
    }

    pub fn initialize(&mut self) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
        let mut dll = self.dll.lock().unwrap();
        let mut devices = USMC_Devices::default();
        
        let result = dll.init(&mut devices)?;
        
        if !is_success(result) {
            return Err(format!("USMC_Init failed with error code: {}", result).into());
        }
        
        self.device_count = devices.NOD as usize;
        
        // Собираем информацию об устройствах
        let mut device_info = Vec::new();
        for i in 0..self.device_count {
            let serial = devices.get_serial(i).unwrap_or_default();
            let version = devices.get_version(i).unwrap_or_default();
            device_info.push((serial, version));
        }
        
        Ok(device_info)
    }

    pub fn get_device_state(&self, device_id: u32) -> Result<USMC_State, Box<dyn std::error::Error>> {
        let dll = self.dll.lock().unwrap();
        let mut state = USMC_State::default();
        
        let result = dll.get_state(device_id, &mut state)?;
        
        if !is_success(result) {
            return Err(format!("USMC_GetState failed with error code: {}", result).into());
        }
        
        Ok(state)
    }

    pub fn start_motor(&self, device_id: u32, dest_pos: i32, speed: f32) -> Result<f32, Box<dyn std::error::Error>> {
        let dll = self.dll.lock().unwrap();
        let mut speed_mut = speed;
        let mut params = USMC_StartParameters::default();
        
        let result = dll.start(device_id, dest_pos, &mut speed_mut, &mut params)?;
        
        if !is_success(result) {
            return Err(format!("USMC_Start failed with error code: {}", result).into());
        }
        
        Ok(speed_mut)
    }

    pub fn stop_motor(&self, device_id: u32) -> Result<(), Box<dyn std::error::Error>> {
        let dll = self.dll.lock().unwrap();
        
        let result = dll.stop(device_id)?;
        
        if !is_success(result) {
            return Err(format!("USMC_Stop failed with error code: {}", result).into());
        }
        
        Ok(())
    }


    pub fn device_count(&self) -> usize {
        self.device_count
    }


    pub fn has_devices(&self) -> bool {
        self.device_count > 0
    }


}
