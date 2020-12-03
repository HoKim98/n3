use std::thread;
use std::time::Duration;

use chrono::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

use n3_builder::{dirs, Result};
use n3_net_client::Work;

use crate::Command;

pub fn execute(command: Command) -> Result<()> {
    let machines = command.env.get_string_list(dirs::N3_MACHINES)?;

    // Step 1. build a program
    let program = command.args.unwrap().build_with_env()?;

    // Step 2. spawn a work
    let work = Work::spawn(&program, command.command, &machines).unwrap();

    // Step 3. wait the work
    let pb = ProgressBar::new(1000000); // 100.0000%
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} {elapsed_precise} {eta} [{bar:40.cyan/blue}] {percent:>2}%")
            .progress_chars("#>-"),
    );

    'pb: loop {
        for i in 0..10 {
            thread::sleep(Duration::from_millis(100));

            if i == 0 {
                let status = work.status().unwrap();
                if let Some(date_end) = status.date_end {
                    let date_begin = status.date_begin.unwrap();
                    let current = (Utc::now() - date_begin).num_seconds() as f64;
                    let total = (date_end - date_begin).num_seconds() as f64;
                    let progress = (1000000.0 * current / total) as u64;
                    pb.set_position(progress);
                }

                if !status.is_running {
                    break 'pb;
                }
            } else {
                pb.inc(0);
            }
        }
    }

    drop(work);

    // Step 4. finalize
    Ok(())
}
