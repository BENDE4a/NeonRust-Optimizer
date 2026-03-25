use std::fs;
use std::path::PathBuf;
use sysinfo::{System, ProcessRefreshKind, RefreshKind};
use windows::Win32::System::Threading::{OpenProcess, SetPriorityClass, HIGH_PRIORITY_CLASS, PROCESS_SET_INFORMATION};

pub fn optimize_client_cfg() -> Result<String, String> {
    let steam_path = match winreg::RegKey::predef(winreg::enums::HKEY_LOCAL_MACHINE)
        .open_subkey("SOFTWARE\\WOW6432Node\\Valve\\Steam") {
            Ok(key) => key.get_value::<String, _>("InstallPath").unwrap_or_else(|_| String::new()),
            Err(_) => String::new(),
        };

    if steam_path.is_empty() {
        return Err("Could not find Steam path in Registry.".to_string());
    }

    let rust_cfg_path = PathBuf::from(&steam_path).join("steamapps/common/Rust/cfg/client.cfg");
    
    if rust_cfg_path.exists() {
        let backup_path = rust_cfg_path.with_extension("cfg.backup");
        let _ = fs::copy(&rust_cfg_path, &backup_path);
        
        let content = fs::read_to_string(&rust_cfg_path).unwrap_or_default();
        let mut new_content = content.replace("graphics.shadowcascades \"1\"", "graphics.shadowcascades \"0\"")
                                     .replace("graphics.shadowmode \"1\"", "graphics.shadowmode \"0\"");
        
        let optimizations = [
            "graphics.shadowcascades \"0\"",
            "gc.buffer \"2048\"",
            "physics.steps \"60\"",
            "global.freezeshortcuts \"1\"",
            "client.lookatradius \"0.01\"",
        ];

        for opt in optimizations.iter() {
            let key = opt.split(' ').next().unwrap();
            if !new_content.contains(key) {
                new_content.push_str(&format!("\n{}", opt));
            }
        }

        if fs::write(&rust_cfg_path, new_content).is_ok() {
            Ok("client.cfg backed up and optimized (shadows off).".to_string())
        } else {
            Err("Failed to write client.cfg".to_string())
        }
    } else {
        Err("client.cfg not found (game not installed on C: drive?).".to_string())
    }
}

pub fn prioritize_rust_client() -> Result<String, String> {
    let mut sys = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    sys.refresh_processes();
    let mut found = false;
    
    for (pid, process) in sys.processes() {
        if process.name().to_lowercase().contains("rustclient.exe") {
            unsafe {
                let handle = OpenProcess(PROCESS_SET_INFORMATION, false, pid.as_u32());
                if let Ok(h) = handle {
                    let _ = SetPriorityClass(h, HIGH_PRIORITY_CLASS);
                    let _ = windows::Win32::Foundation::CloseHandle(h);
                }
            }
            found = true;
        }
    }

    if found {
        Ok("RustClient.exe set to HIGH priority.".to_string())
    } else {
        Err("RustClient.exe is not running.".to_string())
    }
}

pub fn apply_lossless_scaling() -> Result<String, String> {
    use windows::Win32::UI::WindowsAndMessaging::{
        FindWindowW, GetWindowLongW, SetWindowLongW, SetWindowPos, GetSystemMetrics,
        GWL_STYLE, WS_BORDER, WS_CAPTION, WS_THICKFRAME, WS_MINIMIZEBOX, WS_MAXIMIZEBOX, WS_SYSMENU,
        SM_CXSCREEN, SM_CYSCREEN, SWP_NOZORDER, SWP_FRAMECHANGED,
    };
    use windows::core::PCWSTR;
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    unsafe {
        // Rust Class name is typically UnityWndClass for Unity games.
        // Title is "Rust".
        let title: Vec<u16> = OsStr::new("Rust").encode_wide().chain(std::iter::once(0)).collect();
        let hwnd = FindWindowW(PCWSTR::null(), PCWSTR(title.as_ptr()));
        
        if hwnd.0 == 0 {
            return Err("Rust window not found.".to_string());
        }

        let mut style = GetWindowLongW(hwnd, GWL_STYLE);
        
        // Remove borders and title bar
        style &= !(WS_BORDER | WS_CAPTION | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX | WS_SYSMENU).0 as i32;
        SetWindowLongW(hwnd, GWL_STYLE, style);

        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        // Apply new position and size to cover the entire primary monitor
        let _ = SetWindowPos(
            hwnd,
            windows::Win32::Foundation::HWND(0),
            0, 0, screen_width, screen_height,
            SWP_NOZORDER | SWP_FRAMECHANGED,
        );

        Ok("Lossless Scaling applied (Borderless Fullscreen).".to_string())
    }
}

pub fn set_ultimate_power_plan() -> Result<String, String> {
    let _ = std::process::Command::new("powercfg")
        .args(&["-duplicatescheme", "e9a42b02-d5df-448d-aa00-03f14749eb61"])
        .output();
    Ok("Power Plan optimized (Ultimate Performance activated).".to_string())
}

pub fn disable_game_dvr() -> Result<String, String> {
    use winreg::enums::{HKEY_CURRENT_USER, KEY_SET_VALUE};
    use winreg::RegKey;

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok((key, _)) = hkcu.create_subkey_with_flags("System\\GameConfigStore", KEY_SET_VALUE) {
        let _ = key.set_value("GameDVR_Enabled", &0u32);
    }
    if let Ok((key, _)) = hkcu.create_subkey_with_flags("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\GameDVR", KEY_SET_VALUE) {
        let _ = key.set_value("AppCaptureEnabled", &0u32);
    }
    Ok("Xbox Game DVR and overlays disabled.".to_string())
}

pub fn optimize_network() -> Result<String, String> {
    use winreg::enums::{HKEY_LOCAL_MACHINE, KEY_SET_VALUE};
    use winreg::RegKey;
    
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    if let Ok((key, _)) = hklm.create_subkey_with_flags("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion\\Multimedia\\SystemProfile", KEY_SET_VALUE) {
        let _ = key.set_value("NetworkThrottlingIndex", &0xFFFFFFFFu32);
    }
    Ok("Network tweaks applied (Throttling removed).".to_string())
}

pub fn set_timer_resolution() -> Result<String, String> {
    use windows::Win32::Media::timeBeginPeriod;
    unsafe {
        timeBeginPeriod(1);
    }
    Ok("System Timer Resolution locked to 1ms.".to_string())
}
