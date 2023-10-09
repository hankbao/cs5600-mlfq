// task.rs
// Task description
// Author: Hank Bao

pub struct Task {
    start_time: u32,
    workload: u32,
    io_interval: u32,
    io_duration: u32,
}

impl Task {
    pub fn new(start_time: u32, workload: u32, io_interval: u32, io_duration: u32) -> Task {
        Task {
            start_time,
            workload,
            io_interval,
            io_duration,
        }
    }

    pub fn start_time(&self) -> u32 {
        self.start_time
    }

    pub fn workload(&self) -> u32 {
        self.workload
    }

    pub fn io_duration(&self) -> u32 {
        self.io_duration
    }

    pub fn io_interval(&self) -> u32 {
        self.io_interval
    }
}
