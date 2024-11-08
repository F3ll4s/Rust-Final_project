use sysinfo::NetworksExt;
use iced::{Application, Command, Element, Settings, executor, Subscription};
use iced::time::{self, Duration};
use iced::widget::{Column, Text};
use sysinfo::{System, SystemExt, ProcessExt, DiskExt};
use chrono::Utc;
use csv::Writer;
use serde::Serialize;
use std::fs::File;
use sysinfo::CpuExt;
use sysinfo::NetworkExt;


#[derive(Debug, Clone)]
enum Message {
    Tick,
}

#[derive(Serialize)]
struct SystemData {
    host_name: String,
    system_name: String,
    system_version: String,
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    disk_used: u64,
    disk_total: u64,
    network_in: u64,
    network_out: u64,
    timestamp: String,
}

struct SystemMonitorApp {
    host_name: String,
    system_name: String,
    system_version: String,
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    disk_used: u64,
    disk_total: u64,
    network_in: u64,
    network_out: u64,
    system: System,
    writer: Writer<File>,
}

impl SystemMonitorApp {
    fn get_system_metrics(&mut self) {
        self.host_name = self.system.host_name().unwrap_or_default();
        self.system_name = self.system.name().unwrap_or_default();
        self.system_version = self.system.os_version().unwrap_or_default();
        self.system.refresh_all();
        self.cpu_usage = self.system.global_cpu_info().cpu_usage();
        self.memory_used = self.system.used_memory();
        self.memory_total = self.system.total_memory();

        if let Some(disk) = self.system.disks().get(0) {
            self.disk_used = disk.total_space() - disk.available_space();
            self.disk_total = disk.total_space();
        }

        self.network_in = self.system.networks().iter().map(|(_, net)| net.received()).sum();
        self.network_out = self.system.networks().iter().map(|(_, net)| net.transmitted()).sum();
    }

    fn log_to_csv(&mut self) {
        let data = SystemData {
            host_name: self.host_name.clone(),
            system_name: self.system_name.clone(),
            system_version: self.system_version.clone(),
            cpu_usage: self.cpu_usage,
            memory_used: self.memory_used,
            memory_total: self.memory_total,
            disk_used: self.disk_used,
            disk_total: self.disk_total,
            network_in: self.network_in,
            network_out: self.network_out,
            timestamp: Utc::now().to_rfc3339(),
        };

        if let Err(e) = self.writer.serialize(&data) {
            eprintln!("Failed to write to CSV: {:?}", e);
        }
    }
}

impl Application for SystemMonitorApp {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::theme::Theme; 
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let mut system = System::new_all();
        system.refresh_all();

        let cpu_usage = system.global_cpu_info().cpu_usage();
        let memory_used = system.used_memory();
        let memory_total = system.total_memory();

        let file = File::options()
            .append(true)
            .create(true)
            .open("system_data.csv")
            .expect("Unable to open CSV file");
        
        let writer = Writer::from_writer(file);

        let mut app = SystemMonitorApp {
            host_name: String::new(),
            system_name: String::new(),
            system_version: String::new(),
            cpu_usage,
            memory_used,
            memory_total,
            disk_used: 0,
            disk_total: 0,
            network_in: 0,
            network_out: 0,
            system,
            writer,
        };
        
        app.get_system_metrics();
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("System Monitor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Tick => {
                self.get_system_metrics();
                self.log_to_csv();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let host_name_text = format!("Host Name: {}", self.host_name);
        let system_name_text = format!("System Name: {} {}", self.system_name, self.system_version);
        let cpu_text = format!("CPU Usage: {:.2}%", self.cpu_usage);
        let memory_text = format!(
            "Memory Usage: {}/{} MB",
            self.memory_used / (1024 * 1024),
            self.memory_total / (1024 * 1024)
        );
        let disk_text = format!(
            "Disk Usage: {}/{} GB",
            self.disk_used / (1024 * 1024 * 1024),
            self.disk_total / (1024 * 1024 * 1024)
        );
        let network_text = format!(
            "Network: In: {} KB, Out: {} KB",
            self.network_in / 1024,
            self.network_out / 1024
        );

        Column::new()
            .padding(20)
            .spacing(15)
            .push(Text::new("System Monitor").size(50))
            .push(Text::new(host_name_text))
            .push(Text::new(system_name_text))
            .push(Text::new(cpu_text))
            .push(Text::new(memory_text))
            .push(Text::new(disk_text))
            .push(Text::new(network_text))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        time::every(Duration::from_secs(1)).map(|_| Message::Tick)
    }
}


fn main() -> iced::Result {
    SystemMonitorApp::run(Settings::default())
}