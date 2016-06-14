extern crate rust_win32error;

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::error::Error;
use std::os::windows::raw::HANDLE;
use rust_win32error::*;

#[repr(C)]
enum WINDIVERT_LAYER {
    WINDIVERT_LAYER_NETWORK = 0,
    WINDIVERT_LAYER_NETWORK_FORWARD,
}

#[repr(C)]
struct WINDIVERT_ADDRESS {
    IfIdx: u32,
    SubIfIdx: u32,
    Direction: u8,
}

#[link(name="windivert")]
extern "C" {
    fn WinDivertOpen(f: *const c_char, l: WINDIVERT_LAYER, p: i16, flag: i64) -> HANDLE;
    fn WinDivertRecv(h: HANDLE, p: *mut u8, l: u32, a: *mut WINDIVERT_ADDRESS, rl: &u32) -> bool;
}

fn windivert_open(f: &CStr) -> Win32Result<HANDLE> {
    unsafe {
        let h = WinDivertOpen(f.as_ptr(), WINDIVERT_LAYER::WINDIVERT_LAYER_NETWORK, 0, 0);
        match h as i32 {
            -1 => Err(Win32Error::new()),
            _ => Ok(h),
        }
    }
}

fn windivert_recv(h: HANDLE) -> Result<(Vec<u8>, WINDIVERT_ADDRESS, u32), i32> {
    let mut buf = Vec::with_capacity(5000);
    let mut addr = WINDIVERT_ADDRESS {
        Direction: 0,
        IfIdx: 0,
        SubIfIdx: 0,
    };
    let mut rl: u32 = 0;
    unsafe {

        let ok = WinDivertRecv(h,
                               buf.as_mut_ptr(),
                               buf.capacity() as u32,
                               &mut addr as *mut WINDIVERT_ADDRESS,
                               &rl);
    }
    Ok((buf, addr, rl))
}

fn main() {
    println!("Hello, world!");
    let h = windivert_open(&CString::new("tcp").unwrap()).unwrap();
    let (buf, addr, rl) = windivert_recv(h).unwrap();
    println!("{}", rl);
}
