use crate::{DesktopEnv, Platform};

use std::mem;

#[allow(unused)]
#[repr(C)]
enum ExtendedNameFormat {
    NameUnknown,          // Nothing
    NameFullyQualifiedDN, // Nothing
    NameSamCompatible,    // Hostname Followed By Username
    NameDisplay,          // Full Name
    NameUniqueId,         // Nothing
    NameCanonical,        // Nothing
    NameUserPrincipal,    // Nothing
    NameCanonicalEx,      // Nothing
    NameServicePrincipal, // Nothing
    NameDnsDomain,        // Nothing
    NameGivenName,        // Nothing
    NameSurname,          // Nothing
}

#[allow(unused)]
#[repr(C)]
enum ComputerNameFormat {
    ComputerNameNetBIOS,             // Same as GetComputerNameW
    ComputerNameDnsHostname,         // Fancy Name
    ComputerNameDnsDomain,           // Nothing
    ComputerNameDnsFullyQualified,   // Fancy Name with, for example, .com
    ComputerNamePhysicalNetBIOS,     // Same as GetComputerNameW
    ComputerNamePhysicalDnsHostname, // Same as GetComputerNameW
    ComputerNamePhysicalDnsDomain,   // Nothing
    ComputerNamePhysicalDnsFullyQualified, // Fancy Name with, for example, .com
    ComputerNameMax,
}

#[link(name = "Secur32")]
extern "system" {
    fn GetUserNameExW(a: ExtendedNameFormat, b: *mut u16, c: *mut usize) -> u8;
    fn GetUserNameW(a: *mut u16, b: *mut usize) -> i32;
    fn GetComputerNameW(a: *mut u16, b: *mut usize) -> i32;
    fn GetComputerNameExW(
        a: ComputerNameFormat,
        b: *mut u16,
        c: *mut usize,
    ) -> i32;
}

pub fn username() -> String {
    let mut name = mem::MaybeUninit::<[u16; 256]>::uninit();
    let mut size = [256];

    let name = unsafe {
        GetUserNameW(name.as_mut_ptr() as *mut _, size.as_mut_ptr());
        name.assume_init()
    };

    String::from_utf16_lossy(if size[0] == 0 {
        &[]
    } else {
        &name[..size[0] - 1]
    })
}

#[inline(always)]
pub fn realname() -> String {
    let mut name = mem::MaybeUninit::<[u16; 256]>::uninit();
    let mut size = [256];

    let name = unsafe {
        GetUserNameExW(
            ExtendedNameFormat::NameDisplay,
            name.as_mut_ptr() as *mut _,
            size.as_mut_ptr(),
        );
        name.assume_init()
    };

    if size[0] == 0 {
        username()
    } else {
        String::from_utf16_lossy(&name[..size[0]])
    }
}

#[inline(always)]
pub fn computer() -> String {
    let mut name = mem::MaybeUninit::<[u16; 256]>::uninit();
    let mut size = [256];

    let name = unsafe {
        GetComputerNameExW(
            ComputerNameFormat::ComputerNameDnsFullyQualified,
            name.as_mut_ptr() as *mut _,
            size.as_mut_ptr(),
        );
        name.assume_init()
    };

    String::from_utf16_lossy(&name[..size[0]])
}

pub fn hostname() -> String {
    let mut name = mem::MaybeUninit::<[u16; 256]>::uninit();
    let mut size = [256];

    let name = unsafe {
        GetComputerNameW(name.as_mut_ptr() as *mut _, size.as_mut_ptr());
        name.assume_init()
    };

    String::from_utf16_lossy(&name[..size[0]])
}

pub fn os() -> String {
    extern "system" {
        fn GetVersion() -> usize;
    }

    let bits = unsafe { GetVersion() } as u32;

    let mut out = "Windows ".to_string();

    let major: u8 = ((bits & 0b00000000_00000000_00000000_11111111) >> 0) as u8;
    let minor: u8 = ((bits & 0b00000000_00000000_11111111_00000000) >> 8) as u8;
    let build: u16 =
        ((bits & 0b11111111_11111111_00000000_00000000) >> 16) as u16;

    match major {
        5 => out.push_str("XP"),
        6 => match minor {
            0 => out.push_str("Vista"),
            1 => out.push_str("7"),
            2 => match build {
                9200 => out.push_str("10"),
                _ => out.push_str("8"),
            },
            _ => out.push_str("8"),
        },
        _ => out.push_str("Unknown"),
    }

    out
}

#[inline(always)]
pub const fn env() -> DesktopEnv {
    DesktopEnv::Windows
}

#[inline(always)]
pub const fn platform() -> Platform {
    Platform::Windows
}
