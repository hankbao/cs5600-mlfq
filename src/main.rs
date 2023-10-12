// main.rs
// main entry point for the MLFQ scheduler.
// Author: Hank Bao

mod config;
mod process;
mod queue;
mod scheduler;

use std::process::ExitCode;

use clap::Parser;

use config::{JobConfig, QueueConfig, SchedulerConfig};

use crate::scheduler::Scheduler;

/// Options:
///   -q QUANTUMS, --quantum_list=QUANTUMS
///                         length of time slice per queue level, specified as
///                         x,y,z,... where x is the quantum length for the
///                         highest priority queue, y the next highest, and so
///                         forth
///   -a ALLOTMENTS, --allotment_list=ALLOTMENTS
///                         length of time allotment per queue level, specified as
///                         x,y,z,... where x is the # of time slices for the
///                         highest priority queue, y the next highest, and so
///                         forth
///   -l JOBS, --job_list=JOBS
///                         a comma-separated list of jobs to run, in the form
///                         x1,y1,z1,u4:x2,y2,z2,u2:... where x is start time, y is run
///                         time, and z is how often the job issues an I/O request,
///                         and u is how long the I/O request lasts
///   -b BOOST, --boost=BOOST
///                         how often to boost the priority of all jobs back to
///                         high priority
///   -s, --stay            reset and stay at same priority level when issuing I/O
///   -i, --io_bump         if specified, jobs that finished I/O move immediately
///                         to front of current queue
///   -h, --help            show this help message and exit
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "QUANTUMS")]
    quantum_list: String,
    #[arg(short, long, value_name = "ALLOTMENTS")]
    allotment_list: String,
    #[arg(short, long, value_name = "JOBS")]
    job_list: String,
    #[arg(short, long, value_name = "BOOST", default_value = "0")]
    boost: u32,
    #[arg(short, long, default_value = "false")]
    io_bump: bool,
    #[arg(short, long, default_value = "false")]
    stay: bool,
}

fn main() -> ExitCode {
    match Args::try_parse() {
        Ok(args) => {
            let scheduler_config = match parse_scheduler_config(args.boost, args.io_bump, args.stay)
            {
                Some(config) => config,
                None => return ExitCode::FAILURE,
            };

            let queue_config = match parse_queue_configs(args.quantum_list, args.allotment_list) {
                Some(config) => config,
                None => return ExitCode::FAILURE,
            };

            let job_configs = match parse_job_configs(args.job_list) {
                Some(config) => config,
                None => return ExitCode::FAILURE,
            };

            let mut scheduler = Scheduler::new(scheduler_config, queue_config);
            scheduler.add_jobs(job_configs);

            // let the scheduler ticks
            while !scheduler.is_finished() {
                scheduler.run_tick();
            }

            println!("All jobs finished.");
            println!("Total idle time: {}.", scheduler.total_idle_time());
            println!(
                "Average turnaround time: {}.",
                scheduler.average_turnaround_time()
            );
            println!(
                "Average response time: {}.",
                scheduler.average_response_time()
            );

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}

fn parse_scheduler_config(
    priority_boost_interval: u32,
    io_bump: bool,
    io_stay: bool,
) -> Option<SchedulerConfig> {
    Some(SchedulerConfig::new(
        priority_boost_interval,
        io_bump,
        io_stay,
    ))
}

fn parse_queue_configs(quantums: String, allotments: String) -> Option<Vec<QueueConfig>> {
    let mut all_quantum_valid = true;
    let quantum_list = quantums
        .split(',')
        .map(|x| match x.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                all_quantum_valid = false;
                0
            }
        })
        .collect::<Vec<u32>>();

    if !all_quantum_valid {
        eprintln!("Invalid value found in quantum_list.");
        return None;
    }

    let mut all_allotment_valid = true;
    let allotment_list = allotments
        .split(',')
        .map(|x| match x.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                all_allotment_valid = false;
                0
            }
        })
        .collect::<Vec<u32>>();

    if !all_allotment_valid {
        eprintln!("Invalid value found in allotment_list.");
        return None;
    }

    if quantum_list.len() != allotment_list.len() {
        eprintln!("quantum_list and allotment_list must have the same length");
        return None;
    }

    let push_front = true; // FIXME: make this configurable
    let queue_config = std::iter::zip(quantum_list, allotment_list)
        .map(|(quantum, allotment)| QueueConfig::new(quantum, allotment, push_front))
        .collect::<Vec<QueueConfig>>();

    Some(queue_config)
}

fn parse_job_configs(jobs: String) -> Option<Vec<JobConfig>> {
    let mut all_job_valid = true;

    let job_list = jobs
        .split(':')
        .map(|s| {
            s.split(',')
                .map(|x| match x.parse::<u32>() {
                    Ok(x) => x,
                    Err(_) => {
                        all_job_valid = false;
                        0
                    }
                })
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    if !all_job_valid {
        eprintln!("Invalid value found in job_list.");
        return None;
    }

    for job in job_list.iter() {
        if job.len() != 4 {
            all_job_valid = false;
            break;
        }
    }
    if !all_job_valid {
        eprintln!("job_list must be in the form x1,y1,z1,u1:x2,y2,z2,u2:...");
        return None;
    }

    let job_configs = job_list
        .iter()
        .map(|job| JobConfig::new(job[0], job[1], job[2], job[3]))
        .collect::<Vec<JobConfig>>();

    Some(job_configs)
}
