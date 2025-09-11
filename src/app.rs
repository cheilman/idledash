use crate::vcs::VcsInfo;
use chrono::{DateTime, Local, Utc};
use sysinfo::{Disks, System};

pub struct AppState {
    pub local_time: DateTime<Local>,
    pub utc_time: DateTime<Utc>,
    pub system: System,
    pub cpu_history: Vec<u64>,
    pub memory_usage: f64,
    pub disks: Disks,
    pub vcs_status: Vec<VcsInfo>,
}

impl AppState {
    pub fn new() -> AppState {
        let mut system = System::new_all();
        system.refresh_all();
        let disks = Disks::new_with_refreshed_list();

        AppState {
            local_time: Local::now(),
            utc_time: Utc::now(),
            system,
            cpu_history: vec![0; 60],
            memory_usage: 0.0,
            disks,
            vcs_status: Vec::new(),
        }
    }

    pub fn on_tick(&mut self) {
        self.local_time = Local::now();
        self.utc_time = Utc::now();

        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.disks.refresh(true);

        let cpu_usage = (self.system.global_cpu_usage() * 100.0) as u64;
        self.cpu_history.remove(0);
        self.cpu_history.push(cpu_usage);

        self.memory_usage = self.system.used_memory() as f64 / self.system.total_memory() as f64;
    }
}
