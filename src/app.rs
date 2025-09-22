use crate::vcs::VcsInfo;
use chrono::{DateTime, Local, Utc};
use sysinfo::{Disks, System, Networks};

pub struct AppState {
    pub local_time: DateTime<Local>,
    pub utc_time: DateTime<Utc>,
    pub system: System,
    pub cpu_history: Vec<u64>,
    pub memory_usage: f64,
    pub disks: Disks,
    pub vcs_status: Vec<VcsInfo>,
    pub network_up_history: Vec<u64>,
    pub network_down_history: Vec<u64>,
    pub networks: Networks,
    pub previous_network_up: u64,
    pub previous_network_down: u64,
}

impl AppState {
    pub fn new() -> AppState {
        let mut system = System::new_all();
        system.refresh_all();
        let disks = Disks::new_with_refreshed_list();
        let networks = Networks::new_with_refreshed_list();

        AppState {
            local_time: Local::now(),
            utc_time: Utc::now(),
            system,
            cpu_history: vec![0; 60],
            memory_usage: 0.0,
            disks,
            vcs_status: Vec::new(),
            network_up_history: vec![0; 60],
            network_down_history: vec![0; 60],
            networks,
            previous_network_up: 0,
            previous_network_down: 0,
        }
    }

    pub fn on_tick(&mut self) {
        self.local_time = Local::now();
        self.utc_time = Utc::now();

        self.system.refresh_cpu_all();
        self.system.refresh_memory();
        self.disks.refresh(true);
        self.networks.refresh(true);

        let cpu_usage = (self.system.global_cpu_usage() * 100.0) as u64;
        self.cpu_history.remove(0);
        self.cpu_history.push(cpu_usage);

        self.memory_usage = self.system.used_memory() as f64 / self.system.total_memory() as f64;

        let mut total_up = 0;
        let mut total_down = 0;
        for (_, data) in self.networks.iter() {
            total_up += data.transmitted();
            total_down += data.received();
        }

        let up_speed = if self.previous_network_up > 0 {
            (total_up - self.previous_network_up) * 4
        } else {
            0
        };
        let down_speed = if self.previous_network_down > 0 {
            (total_down - self.previous_network_down) * 4
        } else {
            0
        };

        self.network_up_history.remove(0);
        self.network_up_history.push(up_speed);
        self.network_down_history.remove(0);
        self.network_down_history.push(down_speed);

        self.previous_network_up = total_up;
        self.previous_network_down = total_down;
    }
}
