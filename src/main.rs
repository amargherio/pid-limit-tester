use anyhow::Context;
use log::{debug, info, warn, error};

use std::env;
use std::process::{Child, Command};

fn main() -> anyhow::Result<()> {
    env_logger::init();
    debug!("Completed logging init, starting configuration stage.");

    // Read the `TARGET_PID_COUNT` environment variable for the number of
    // processes we should attempt to create. There's two possible failure
    // scenarios here:
    //   1. The value of `TARGET_PID_COUNT` is not parsable as a usize value.
    //   2. An error was returned by env::var(). Because we're checking for a
    //  specific key, this either means the value contains a NUL character
    //  or an ASCII '=' sign.
    //
    // If either of those scenarios comes up, we use `anyhow::bail!()` to exit
    // early with a returned error.
    let mut target_count = 0;
    if let Ok(count) = env::var("TARGET_PID_COUNT") {
        debug!("Found 'TARGET_PID_COUNT' environment variable, checking value and assigning.");
        target_count = count.parse().unwrap_or_else(|_| {
            warn!("The value of 'TARGET_PID_COUNT' was not parsable as an i32.");
            0
        });
        if target_count == 0 {
            anyhow::bail!("Unable to parse the 'TARGET_PID_COUNT' as an i32. Found value: {}", count);
        }
    } else {
        error!("Failed to read 'TARGET_PID_COUNT' - it was not assigned a value or the key was incorrectly entered.");
        anyhow::bail!("Environment variable 'TARGET_PID_COUNT' was not found.");
    }
    info!("Completed configuring the target PID value for the test. The test will attempt to spawn {} PIDs within the pod.", target_count);

    let mut child_vec: Vec<Child> = Vec::with_capacity(target_count);
    let mut spawn_error = false; // used to trach 

    for i in 0..target_count {
        if i == 0 {
            info!("Starting the first child process.");
        } else if i+1 % 10 == 0 {
            info!("Starting child process number {} of {}", i+1, target_count);
        } else {
            debug!("Starting child process number {} of {}", i+1, target_count);
        }

        // We're going to start a child process that tails /dev/null.
        // This process should stick around until either the full PID count is
        // started or we run out of available PIDs for the pod to use. If we
        // run out, we should receive an error when attempting to spawn a new
        // process. When the spawn fails, we will clean up all remaining child
        // processes and exit 0.
        let child_res = Command::new("/bin/tail")
            .arg("-f")
            .arg("/dev/null")
            .spawn()
            .with_context(|| format!("Attempting to spawn child task "));

        match child_res {
            Ok(child) => child_vec.push(child),
            Err(_) => {
                spawn_error = true;
                todo!()
            }
        };

        // If an error was returned attempting to spawn the last child process,
        // we break out of the spawning logic since we're already up against
        // the PID limit.
        if spawn_error {
            break;
        }
    }

    if spawn_error {
        warn!(
            "Errors were encountered while attempting to spawn new child processes. {} child processes were spawned before the error.",
            child_vec.len(),
        );
    } else {
        info!(
            "Successfully spawned {} processes without issue. Cleaning up the tracked child processes spawned during testing.",
            target_count
        );
    }
    kill_spawned_processes(&mut child_vec);

    Ok(())
}

fn kill_spawned_processes(child_vec:&mut Vec<Child>) -> anyhow::Result<()> {
    debug!("Beginning clean up of existing child processes. {} to clean up.", child_vec.len());

    let iter = child_vec.iter_mut();

    for child in iter {
        let cpid = child.id();
        let res = child.kill();

        match res {
            Ok(_) => debug!("Successfully terminated child process {}.", cpid),
            Err(e) => {
                if e.kind() == std::io::ErrorKind::InvalidInput {
                    info!(
                        "Child PID {} was already stopped.",
                        cpid
                    );
                }
                warn!(
                    "Child process {} was not running when kill was called.",
                    cpid);
            }
        }
    }

    Ok(())
}
