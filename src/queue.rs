// queue.rs
// Queue struct and implementation.
// Author: Hank Bao

use crate::config::QueueConfig;
use crate::process::Process;

pub struct Queue {
    quantum: u32,
    allotment: u32,
    new_process_first: bool,
    processes: Vec<Process>,
}

impl Queue {
    pub fn new(quantum: u32, allotment: u32, new_process_first: bool) -> Queue {
        Queue {
            quantum,
            allotment,
            new_process_first,
            processes: Vec::new(),
        }
    }

    pub fn quantum(&self) -> u32 {
        self.quantum
    }

    pub fn allotment(&self) -> u32 {
        self.allotment
    }

    pub fn number_of_processes(&self) -> usize {
        self.processes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.processes.is_empty()
    }

    pub fn pop_all(&mut self) -> Vec<Process> {
        std::mem::replace(&mut self.processes, Vec::new())
    }

    pub fn add_process(&mut self, mut process: Process) {
        // Update the allotment of the process when adding it to the queue
        process.set_allotment(self.allotment);

        if self.new_process_first {
            self.processes.insert(0, process);
        } else {
            self.processes.push(process);
        }
    }

    pub fn has_schedulable_process(&self, current_time: u32) -> bool {
        self.processes
            .iter()
            .any(|p| p.next_schedule_time() <= current_time)
    }

    pub fn take_next_schedulable_process(&mut self, current_time: u32) -> Option<Process> {
        self.processes
            .iter()
            .position(|p| p.next_schedule_time() <= current_time)
            .map(|i| {
                let process = self.processes.remove(i);
                process
            })
    }

    pub fn put_process_back(&mut self, process: Process, bump: bool) {
        if bump {
            if let Some(idx) = self
                .processes
                .iter()
                .position(|p| p.next_schedule_time() > process.next_schedule_time())
            {
                // Put the process before the first process that has a later next schedule time,
                // so that it may be scheduled earlier
                self.processes.insert(idx, process);
            } else {
                // The process has the latest next schedule time, put it at the end of the queue
                self.processes.push(process);
            }
        } else {
            // Put the process at the end of the queue, essentially performing round-robin
            self.processes.push(process);
        }
    }
}

impl From<QueueConfig> for Queue {
    fn from(config: QueueConfig) -> Self {
        // FIXME: add new_process_first to QueueConfig
        Queue::new(config.quantum(), config.allotment(), true)
    }
}
