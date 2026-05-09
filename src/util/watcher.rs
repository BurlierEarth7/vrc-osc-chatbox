use std::sync::Arc;

use arc_swap::ArcSwap;
use notify::{RecommendedWatcher, Watcher, event};

use crate::util::config::Config;

// TODO: Verify watcher runs without needing 2nd path

pub fn watch_config(
    path: String,
    config: Arc<ArcSwap<Config>>,
) -> notify::Result<RecommendedWatcher> {
    let watch_path = path.clone();
    let mut watcher = RecommendedWatcher::new(
        move |res: Result<notify::Event, notify::Error>| {
            let Ok(event) = res else { return };

            match event.kind {
                event::EventKind::Modify(_) | event::EventKind::Create(_) => {
                    println!("Configuration changed, reloading...");

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
        },
        notify::Config::default(),
    )?;

    watcher.watch(watch_path.as_ref(), notify::RecursiveMode::NonRecursive)?;

    Ok(watcher)
}