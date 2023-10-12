// main.rs
// main entry point for the MLFQ scheduler.
// Author: Hank Bao

mod config;
mod process;
mod queue;
mod scheduler;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::Parser;

use config::{JobConfig, QueueConfig, SchedulerConfig};

/// Options:
///   -c CONFIG, --config=CONFIG
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
            let interval = args.boost;
            let io_bump = args.io_bump;
            let io_stay = args.stay;

            let scheduler_config = SchedulerConfig::new(interval, io_bump, io_stay);

            let quantum_list = args
                .quantum_list
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            let allotment_list = args
                .allotment_list
                .split(',')
                .map(|x| x.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();

            if quantum_list.len() != allotment_list.len() {
                eprintln!("quantum_list and allotment_list must have the same length");
                return ExitCode::FAILURE;
            }

            let push_front = true; // FIXME: make this configurable
            let queue_config = std::iter::zip(quantum_list, allotment_list)
                .map(|(quantum, allotment)| QueueConfig::new(quantum, allotment, push_front))
                .collect::<Vec<QueueConfig>>();

            let job_list = args
                .job_list
                .split(':')
                .map(|s| {
                    s.split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>();

            let mut all_job_valid = true;
            for job in job_list.as_ref() {
                if job.len() != 4 {
                    all_job_valid = false;
                    break;
                }
            }
            if !all_job_valid {
                eprintln!("job_list must be in the form x1,y1,z1,u1:x2,y2,z2,u2:...");
                return ExitCode::FAILURE;
            }

            let job_configs = job_list
                .iter()
                .map(|job| JobConfig::new(job[0], job[1], job[2], job[3]))
                .collect::<Vec<JobConfig>>();

            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("{}", e);
            ExitCode::FAILURE
        }
    }
}
