// Job.rs
// Job description
// Author: Hank Bao

pub struct Job {
    arrival_time: u32,
    workload: u32,
    io_interval: u32,
    io_duration: u32,
}

impl Job {
    pub fn new(arrival_time: u32, workload: u32, io_interval: u32, io_duration: u32) -> Job {
        Job {
            arrival_time,
            workload,
            io_interval,
            io_duration,
        }
    }

    pub fn arrival_time(&self) -> u32 {
        self.arrival_time
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
