#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi::{c_char, CStr};
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

// Параметры контроллера 
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct USMC_Parameters {
    pub AccelT: f32,  
    pub DecelT: f32,        
    pub PTimeout: f32,      
    pub BTimeout1: f32,     
    pub BTimeout2: f32,     
    pub BTimeout3: f32,     
    pub BTimeout4: f32,     
    pub BTimeoutR: f32,     
    pub BTimeoutD: f32,     
    pub MinP: f32,          
    pub BTO1P: f32,         
    pub BTO2P: f32,         
    pub BTO3P: f32,         
    pub BTO4P: f32,         
    pub MaxLoft: u16,       
    pub StartPos: u32,      
    pub RTDelta: u16,       
    pub RTMinError: u16,    
    pub MaxTemp: f32,       
    pub SynOUTP: f32,       
    pub LoftPeriod: f32,    
    pub EncMult: f32,       
}

impl Default for USMC_Parameters {
    fn default() -> Self {
        Self {
            AccelT: 0.0,
            DecelT: 0.0,
            PTimeout: 0.0,
            BTimeout1: 0.0,
            BTimeout2: 0.0,
            BTimeout3: 0.0,
            BTimeout4: 0.0,
            BTimeoutR: 0.0,
            BTimeoutD: 0.0,
            MinP: 0.0,
            BTO1P: 0.0,
            BTO2P: 0.0,
            BTO3P: 0.0,
            BTO4P: 0.0,
            MaxLoft: 0,
            StartPos: 0,
            RTDelta: 0,
            RTMinError: 0,
            MaxTemp: 0.0,
            SynOUTP: 0.0,
            LoftPeriod: 0.0,
            EncMult: 0.0,
        }
    }
}

// Параметры запуска
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct USMC_StartParameters {
    pub SDivisor: u8,       
    pub DefDir: bool,       
    pub LoftEn: bool,       
    pub SlStart: bool,      
    pub WSyncIN: bool,      
    pub SyncOUTR: bool,     
    pub ForceLoft: bool,    
}

impl Default for USMC_StartParameters {
    fn default() -> Self {
        Self {
            SDivisor: 1,
            DefDir: false,
            LoftEn: false,
            SlStart: false,
            WSyncIN: false,
            SyncOUTR: false,
            ForceLoft: false,
        }
    }
}

/// Режим контроллера
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct USMC_Mode {
    pub PMode: bool,        
    pub PReg: bool,         
    pub ResetD: bool,       
    pub EMReset: bool,      
    pub Tr1T: bool,         
    pub Tr2T: bool,         
    pub RotTrT: bool,       
    pub TrSwap: bool,       
    pub Tr1En: bool,        
    pub Tr2En: bool,        
    pub RotTeEn: bool,      
    pub RotTrOp: bool,      
    pub Butt1T: bool,       
    pub Butt2T: bool,       
    pub ResetRT: bool,      
    pub SyncOUTEn: bool,    
    pub SyncOUTR: bool,     
    pub SyncINOp: bool,     
    pub SyncCount: u32,     
    pub SyncInvert: bool,   
    pub EncoderEn: bool,    
    pub EncoderInv: bool,   
    pub ResBEnc: bool,      
    pub ResEnc: bool,       
}

impl Default for USMC_Mode {
    fn default() -> Self {
        Self {
            PMode: false,
            PReg: false,
            ResetD: false,
            EMReset: false,
            Tr1T: false,
            Tr2T: false,
            RotTrT: false,
            TrSwap: false,
            Tr1En: false,
            Tr2En: false,
            RotTeEn: false,
            RotTrOp: false,
            Butt1T: false,
            Butt2T: false,
            ResetRT: false,
            SyncOUTEn: false,
            SyncOUTR: false,
            SyncINOp: false,
            SyncCount: 0,
            SyncInvert: false,
            EncoderEn: false,
            EncoderInv: false,
            ResBEnc: false,
            ResEnc: false,
        }
    }
}

/// Состояние контроллера
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct USMC_State {
    pub CurPos: i32,        
    pub Temp: f32,          
    pub SDivisor: u8,       
    pub Loft: bool,         
    pub FullPower: bool,    
    pub CW_CCW: bool,       
    pub Power: bool,        
    pub FullSpeed: bool,    
    pub AReset: bool,       
    pub RUN: bool,          
    pub SyncIN: bool,       
    pub SyncOUT: bool,      
    pub RotTr: bool,        
    pub RotTrErr: bool,     
    pub EmReset: bool,      
    pub Trailer1: bool,     
    pub Trailer2: bool,     
    pub Voltage: f32,       
}

impl Default for USMC_State {
    fn default() -> Self {
        Self {
            CurPos: 0,
            Temp: 0.0,
            SDivisor: 1,
            Loft: false,
            FullPower: false,
            CW_CCW: false,
            Power: false,
            FullSpeed: false,
            AReset: false,
            RUN: false,
            SyncIN: false,
            SyncOUT: false,
            RotTr: false,
            RotTrErr: false,
            EmReset: false,
            Trailer1: false,
            Trailer2: false,
            Voltage: 0.0,
        }
    }
}

/// Структура состояния энкодера
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct USMC_EncoderState {
    pub EncoderPos: i32,  
    pub ECurPos: i32,
}

impl Default for USMC_EncoderState {
    fn default() -> Self {
        Self {
            EncoderPos: 0,
            ECurPos: 0,
        }
    }
}

pub const USMC_SUCCESS: u32 = 0;
pub const USMC_ERROR: u32 = 1;

pub fn cstr_to_string(ptr: *const c_char) -> String {
    if ptr.is_null() {
        return String::new();
    }
    
    unsafe {
        CStr::from_ptr(ptr)
            .to_string_lossy()
            .into_owned()
    }
}

pub fn create_cstring_buffer(len: usize) -> Vec<u8> {
    vec![0u8; len]
}

pub fn is_success(error_code: u32) -> bool {
    error_code == USMC_SUCCESS
}