use sysinfo::{Pid, System};

pub fn process_name_for_pid(pid: u32) -> Option<String> {
    let mut sys = System::new_all();
    sys.refresh_processes();
    let proc = sys.process(Pid::from_u32(pid))?;
    Some(proc.name().to_string())
}
