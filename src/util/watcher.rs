use std::{
    path::PathBuf,
    sync::{
        mpsc::{channel, RecvTimeoutError},
        Arc,
    },
    thread,
    time::Duration,
};

use arc_swap::ArcSwap;
use notify::{event, RecommendedWatcher, RecursiveMode, Watcher};

use crate::{constants, util::config::Config};

/// Watch a configuration file for changes
/// 
/// * path - The path of the file
/// * config - How to store the config
pub fn watch_config(
    path: PathBuf,
    config: Arc<ArcSwap<Config>>,
) -> notify::Result<RecommendedWatcher> {
    // Setup MPSC Channel
    let (tx, rx) = channel();

    let config_file = path.file_name().unwrap().to_owned();
    let watch_dir = path.parent().unwrap();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            let _ = tx.send(res);
        },
        notify::Config::default(),
    )?;

    // Watch directory for changes
    watcher.watch(watch_dir, RecursiveMode::NonRecursive)?;

    // Loop until file events are received
    thread::spawn(move || {
        loop {
            // Event caught
            let Ok(res) = rx.recv() else {
                break;
            };

            // Invalid event
            let Ok(event) = res else {
                continue;
            };

            // Check if event affects config
            let config_modified = event.paths.iter().any(|p| {
                p.file_name()
                    .map(|name| name == config_file)
                    .unwrap_or(false)
            });

            if !config_modified {
                continue;
            }

            match event.kind {
                event::EventKind::Modify(_)
                | event::EventKind::Create(_) => {

                    // Debounce
                    loop {
                        match rx.recv_timeout(Duration::from_millis(constants::DEBOUNCE_TIMEOUT)) {
                            Ok(_) => continue,
                            Err(RecvTimeoutError::Timeout) => break,
                            Err(RecvTimeoutError::Disconnected) => return,
                        }
                    }

                    println!("Configuration changed, reloading...");

                    // Load new config
                    match Config::load() {
                        Ok(new_config) => {
                            config.store(Arc::new(new_config));
                            println!("Configuration reloaded!");
                        }
                        Err(e) => {
                            eprintln!("Failed to reload config: {e}");
                        }
                    }
                }
                _ => {}
            }
        }
    });

    Ok(watcher)
}