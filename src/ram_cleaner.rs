use std::ffi::c_void;
use windows::Win32::Foundation::{CloseHandle, HANDLE, LUID};
use windows::Win32::Security::{
    AdjustTokenPrivileges, LookupPrivilegeValueW, LUID_AND_ATTRIBUTES, SE_PRIVILEGE_ENABLED,
    TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES, TOKEN_QUERY,
};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

#[link(name = "ntdll")]
extern "system" {
    fn NtSetSystemInformation(
        SystemInformationClass: u32,
        SystemInformation: *mut c_void,
        SystemInformationLength: u32,
    ) -> i32;
}

pub fn purge_standby_list() -> Result<String, String> {
    unsafe {
        let mut token: HANDLE = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_ADJUST_PRIVILEGES | TOKEN_QUERY, &mut token).is_err() {
            return Err("Failed to open process token. Are you running as Admin?".to_string());
        }

        let mut luid = LUID::default();
        let priv_name: Vec<u16> = "SeProfileSingleProcessPrivilege"
            .encode_utf16()
            .chain(std::iter::once(0))
            .collect();

        if LookupPrivilegeValueW(None, windows::core::PCWSTR(priv_name.as_ptr()), &mut luid).is_err() {
            let _ = CloseHandle(token);
            return Err("Failed to lookup SeProfileSingleProcessPrivilege.".to_string());
        }

        let mut tp = TOKEN_PRIVILEGES {
            PrivilegeCount: 1,
            Privileges: [LUID_AND_ATTRIBUTES {
                Luid: luid,
                Attributes: SE_PRIVILEGE_ENABLED,
            }],
        };

        if AdjustTokenPrivileges(token, false, Some(&mut tp), 0, None, None).is_err() {
            let _ = CloseHandle(token);
            return Err("Failed to adjust token privileges.".to_string());
        }

        // SystemMemoryListInformation == 80
        // MemoryPurgeStandbyList == 4
        let mut command: u32 = 4;
        let status = NtSetSystemInformation(
            80,
            &mut command as *mut _ as *mut c_void,
            std::mem::size_of::<u32>() as u32,
        );

        let _ = CloseHandle(token);

        if status >= 0 {
            Ok("Standby Memory List Purged successfully.".to_string())
        } else {
            Err(format!("NtSetSystemInformation failed (Status: {:#X})", status))
        }
    }
}
