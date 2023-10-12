// scheduler.rs
// Scheduler for the Multi-Level Feedback Queue (MLFQ) scheduling algorithm.
// Author: Hank Bao

use crate::config::{JobConfig, QueueConfig, SchedulerConfig};
use crate::process::Process;
use crate::queue::Queue;

pub struct Scheduler {
    queues: Vec<Queue>,
    current_time: u32,
    last_boost_time: u32,
    config: SchedulerConfig,
    pid_counter: u32,
    idle_counter: u32,
    idle_total: u32,
    turnaround_total: u32,
    response_total: u32,
}

impl Scheduler {
    pub fn new(config: SchedulerConfig, queue_configs: Vec<QueueConfig>) -> Scheduler {
        Scheduler {
            queues: queue_configs.into_iter().map(Queue::from).collect(),
            current_time: 0,
            last_boost_time: 0,
            config,
            pid_counter: 0,
            idle_counter: 0,
            idle_total: 0,
            turnaround_total: 0,
            response_total: 0,
        }
    }

    pub fn add_jobs(&mut self, jobs: Vec<JobConfig>) {
        for job in jobs {
            self.add_job(job);
        }
    }

    pub fn add_job(&mut self, job: JobConfig) {
        let proc = Process::new(
            self.pid_counter,
            job.io_interval(),
            job.io_length(),
            job.workload(),
            job.arrival_time(),
        );
        self.pid_counter += 1;

        self.queues[0].add_process(proc);
    }

    pub fn is_finished(&self) -> bool {
        self.queues.iter().all(|q| q.is_empty())
    }

    pub fn total_idle_time(&self) -> u32 {
        self.idle_total
    }

    pub fn average_turnaround_time(&self) -> u32 {
        self.turnaround_total / self.pid_counter
    }

    pub fn average_response_time(&self) -> u32 {
        self.response_total / self.pid_counter
    }

    // Based on the MLFQ rules described in "Operating Systems: Three Easy Pieces"
    // 1. If Priority(A) > Priority(B), A runs (B doesn’t).
    // 2. If Priority(A) = Priority(B), A & B run in round-robin fashion
    //   using the time slice (quantum length) of the given queue.
    // 3. When a job enters the system, it is placed at the highest priority (the topmost queue).
    // 4. Once a job uses up its time allotment at a given level, its priority is reduced
    // 5. After some time period S, move all the jobs in the system to the topmost queue.
    pub fn run_tick(&mut self) {
        // Check if we need to do a priority boost
        if self.priority_boost_check() {
            self.do_priority_boost();
        }

        // Find the next schedulable process
        if let Some(index) = self.find_runnable_queue() {
            let process = self.queues[index].take_next_schedulable_process(self.current_time);

            if let Some(mut process) = process {
                println!("CPU idle for {} ticks.", self.idle_counter);
                self.idle_counter = 0;

                let quantum = self.queues[index].quantum();
                let run_time = process.run(quantum, self.current_time, index);
                self.current_time += run_time;

                if process.is_finished() {
                    // Process finished, print its response time & turnaround time
                    println!(
                        "Process {} finished. Response time: {}. Turnaround time: {}.",
                        process.pid(),
                        process.response_time(),
                        process.turnaround_time()
                    );

                    self.turnaround_total += process.turnaround_time();
                    self.response_total += process.response_time();
                } else {
                    // Rule 4, reduce the priority of the process
                    let pid = process.pid();
                    let do_io_stay = self.config.io_stay() && process.is_blocked();

                    if process.allotment() == 0 && !do_io_stay && index < self.queues.len() - 1 {
                        // reset the next schedule time for the process
                        self.queues[index + 1].add_process(process);

                        println!("Process {} priority reduced to {}.", pid, index + 1);
                    } else {
                        if do_io_stay {
                            println!("Process {} stay in queue {} after I/O.", pid, index);
                        }

                        let do_io_bump = self.config.io_bump() && process.is_blocked();
                        self.queues[index].put_process_back(process, do_io_bump);

                        if do_io_bump {
                            println!("Process {} bumped in queue {} after I/O.", pid, index);
                        }
                    }
                }
            }
        } else {
            self.idle_counter += 1;
            self.idle_total += 1;
            self.current_time += 1;
        }
    }

    fn priority_boost_check(&self) -> bool {
        let interval = self.config.priority_boost_interval();
        if interval == 0 {
            return false;
        }

        self.current_time - self.last_boost_time >= interval
    }

    fn do_priority_boost(&mut self) {
        for i in 1..self.queues.len() {
            let q = self.queues[i].pop_all();
            for p in q {
                self.queues[0].add_process(p);
            }
        }

        self.last_boost_time = self.current_time;
    }

    fn find_runnable_queue(&self) -> Option<usize> {
        let current_time = self.current_time;
        self.queues
            .iter()
            .enumerate()
            .find(move |(_, q)| q.has_schedulable_process(current_time))
            .map(|(i, _)| i)
    }
}
