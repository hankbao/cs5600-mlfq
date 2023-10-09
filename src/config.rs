// config.rs
// Config store parameters for the MLFQ scheduler.
// Author: Hank Bao

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct QueueConfig {
    quantum: u32,
    allotment: u32,
}

impl QueueConfig {
    pub fn new(quantum: u32, allotment: u32) -> QueueConfig {
        QueueConfig { quantum, allotment }
    }

    pub fn quantum(&self) -> u32 {
        self.quantum
    }

    pub fn allotment(&self) -> u32 {
        self.allotment
    }
}

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
