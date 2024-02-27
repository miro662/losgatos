/// Rust wrapper over SBI v2 interface.
///
/// Safe wrappers over SBI functions used in LosGatOS.
/// Based on https://drive.google.com/file/d/1U2kwjqxXgDONXk_-ZDTYzvsV-F_8ylEH/view
use core::arch::asm;
use core::mem;

/// Represents access to base extension
pub struct Base;

impl Base {
    /// Checks if extension with given name exists
    pub fn probe_extension(&self, extension: ExtensionName) -> bool {
        let call = SbiCall {
            extension_name: ExtensionName::Base,
            fid: 2,
            params: [extension as u32, 0, 0, 0, 0, 0],
        };
        let result = unsafe { call.make() };
        result.map(|r| r > 0).unwrap_or_default()
    }
}

/// Represents access to console extension
pub struct DebugConsole {
    _construction_guard: (),
}

impl DebugConsole {
    /// Returns debug console handle if it is supported
    pub fn get_if_available() -> Option<DebugConsole> {
        if true {
            Some(DebugConsole {
                _construction_guard: (),
            })
        } else {
            None
        }
    }

    /// Writes a single byte to debug console
    pub fn write_byte(&self, byte: u8) {
        let call = SbiCall {
            extension_name: ExtensionName::DebugConsole,
            fid: 2,
            params: [byte.into(), 0, 0, 0, 0, 0],
        };
        unsafe { call.make().unwrap() };
    }
}

/// Names of SBI extensions
#[derive(Debug, Clone, Copy)]
pub enum ExtensionName {
    Base = 0x10,
    DebugConsole = 0x4442434E,
}

#[derive(Debug, Clone, Copy)]
struct SbiCall {
    extension_name: ExtensionName,
    fid: i32,
    params: [u32; 6],
}

type SbiResult = Result<i32, i32>;

impl SbiCall {
    fn split_pointer<T>(pointer: *const T) -> (u32, u32) {
        let contents: [u32; 2] = unsafe { mem::transmute(pointer) };
        (contents[0], contents[1])
    }

    unsafe fn make(&self) -> SbiResult {
        let mut error: i32;
        let mut value: i32;

        asm! {
            "mv a7, {extension_id}",
            "mv a6, {fid}",
            "mv a0, {param0}",
            "mv a1, {param1}",
            "mv a2, {param2}",
            "mv a3, {param3}",
            "mv a4, {param4}",
            "mv a5, {param5}",
            "ecall",
            "mv {error}, a0",
            "mv {value}, a1",
            extension_id = in(reg) self.extension_name as i32,
            fid = in(reg) self.fid,
            param0 = in(reg) self.params[0],
            param1 = in(reg) self.params[1],
            param2 = in(reg) self.params[2],
            param3 = in(reg) self.params[3],
            param4 = in(reg) self.params[4],
            param5 = in(reg) self.params[5],
            error = out(reg) error,
            value = out(reg) value,
        }

        if error < 0 {
            Err(error)
        } else {
            Ok(value)
        }
    }
}
