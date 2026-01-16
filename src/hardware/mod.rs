mod dll_wrapper;
mod types;

pub use dll_wrapper::{USMCDLL, USMCController};
pub use types::{
    USMC_Devices, USMC_Parameters, USMC_StartParameters, 
    USMC_Mode, USMC_State, USMC_EncoderState,
    USMC_SUCCESS, USMC_ERROR,
    is_success, cstr_to_string, create_cstring_buffer
};

pub type USMCResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub enum USMCError {
    LibraryLoad(String),
    FunctionCall(String),
    Execution(u32),
    Device(String),
    Initialization(String),
}

impl std::fmt::Display for USMCError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            USMCError::LibraryLoad(msg) => write!(f, "Library load error: {}", msg),
            USMCError::FunctionCall(msg) => write!(f, "Function call error: {}", msg),
            USMCError::Execution(code) => write!(f, "Execution error: code {}", code),
            USMCError::Device(msg) => write!(f, "Device error: {}", msg),
            USMCError::Initialization(msg) => write!(f, "Initialization error: {}", msg),
        }
    }
}

impl std::error::Error for USMCError {}

pub fn get_last_error_string(dll: &USMCDLL) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = vec![0u8; 256];
    dll.get_last_error(&mut buffer)?;
    
    let null_pos = buffer.iter().position(|&x| x == 0).unwrap_or(buffer.len());
    
    Ok(String::from_utf8_lossy(&buffer[..null_pos]).to_string())
}

pub fn check_dll_availability() -> bool {
    USMCDLL::new().is_ok()
}


#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: u32,
    pub serial: String,
    pub version: String,
    pub state: Option<USMC_State>,
}


pub struct USMCDeviceManager {
    controller: USMCController,
    devices: Vec<DeviceInfo>,
}

impl USMCDeviceManager {

    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let controller = USMCController::new()?;
        Ok(Self {
            controller,
            devices: Vec::new(),
        })
    }
    

    pub fn refresh_devices(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let device_list = self.controller.initialize()?;
        
        self.devices.clear();
        for (i, (serial, version)) in device_list.into_iter().enumerate() {
            let state = self.controller.get_device_state(i as u32).ok();
            
            self.devices.push(DeviceInfo {
                id: i as u32,
                serial,
                version,
                state,
            });
        }
        
        Ok(())
    }
    

    pub fn get_devices(&self) -> &[DeviceInfo] {
        &self.devices
    }
    

    pub fn get_device(&self, id: u32) -> Option<&DeviceInfo> {
        self.devices.iter().find(|d| d.id == id)
    }
    

    pub fn get_controller(&self) -> &USMCController {
        &self.controller
    }
}
