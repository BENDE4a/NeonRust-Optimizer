use sysinfo::{System, ProcessRefreshKind, RefreshKind};
use std::process::Command;

pub fn kill_bloatware() -> (u32, String) {
    let sys = System::new_with_specifics(RefreshKind::new().with_processes(ProcessRefreshKind::everything()));
    let target_processes = [
        "epicgameslauncher", "discord", "spotify", "onedrive", 
        "skype", "cortana", "msedge",
    ];

    let mut killed = 0;
    for (_pid, process) in sys.processes() {
        let name = process.name().to_lowercase();
        if target_processes.iter().any(|&target| name.contains(target)) {
            if process.kill() {
                killed += 1;
            }
        }
    }

    // Stop common heavy Windows services
    let services = ["SysMain", "DiagTrack", "WSearch"];
    let mut stopped_services = 0;
    for svc in services {
        if let Ok(status) = Command::new("net").args(["stop", svc, "/y"]).output() {
            if status.status.success() {
                stopped_services += 1;
            }
        }
    }

    (killed, format!("Killed {} processes, stopped {} telemetry/heavy services.", killed, stopped_services))
}
