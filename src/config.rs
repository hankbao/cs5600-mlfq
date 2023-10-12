// config.rs
// Config store parameters for the MLFQ scheduler.
// Author: Hank Bao

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct SchedulerConfig {
    io_bump: bool,
    io_stay: bool,
    priority_boost_interval: u32,
}

impl SchedulerConfig {
    pub fn new(priority_boost_interval: u32, io_bump: bool, io_stay: bool) -> SchedulerConfig {
        SchedulerConfig {
            priority_boost_interval,
            io_bump,
            io_stay,
        }
    }

    pub fn priority_boost_interval(&self) -> u32 {
        self.priority_boost_interval
    }

    pub fn io_bump(&self) -> bool {
        self.io_bump
    }

    pub fn io_stay(&self) -> bool {
        self.io_stay
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct JobConfig {
    arrival_time: u32,
    workload: u32,
    io_interval: u32,
    io_duration: u32,
}

impl JobConfig {
    pub fn new(arrival_time: u32, workload: u32, io_interval: u32, io_duration: u32) -> JobConfig {
        JobConfig {
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

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct QueueConfig {
    quantum: u32,
    allotment: u32,
    push_front: bool,
}

impl QueueConfig {
    pub fn new(quantum: u32, allotment: u32, push_front: bool) -> QueueConfig {
        QueueConfig {
            quantum,
            allotment,
            push_front,
        }
    }

    pub fn quantum(&self) -> u32 {
        self.quantum
    }

    pub fn allotment(&self) -> u32 {
        self.allotment
    }

    pub fn push_front(&self) -> bool {
        self.push_front
    }
}
