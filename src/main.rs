use arc_swap::ArcSwap;
use notify::RecommendedWatcher;
use std::sync::Arc;
mod constants;
mod util;
use crate::modules::music::modes::ModeHandler;
use crate::modules::music::modes::on_change::OnChange;
use crate::modules::music::modes::sync::SyncMode;
use crate::util::config::Config;
use crate::util::osc::OscClient;
use crate::util::player;
use crate::util::watcher::watch_config;
mod modules;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    if let Err(_) = player::check_has_playerctl() {
        eprintln!("WARNING: Playerctl is not installed, no commands using it will work!");
    }

    if let Err(_) = modules::datetime::date::Date::check_has_date() {
        eprintln!("WARNING: date is not installed, no commands using it will work!");
    }

    let mut app = App::new()?;

    loop {
        let config = app.config.load_full();

        let client = app
            .state
            .client
            .as_ref()
            .expect("client should be initialized");

        match config.display_mode {
            util::mode::Mode::Sync => {
                app.handlers.interval.tick(client, &config)?;
            }
            util::mode::Mode::Swap => {
                app.handlers.on_change.tick(client, &config)?;
            }
        }

        let sleep_duration = match config.display_mode {
            util::mode::Mode::Sync => {
                std::time::Duration::from_secs(config.sync_refresh_interval_seconds)
            }
            _ => {
                // event-driven modes should not be delayed
                std::time::Duration::from_millis(100)
            }
        };

        std::thread::sleep(sleep_duration);
    }
}


struct RuntimeState {
    client: Option<OscClient>,
}

struct Handlers {
    interval: Box<dyn ModeHandler + Send>,
    on_change: Box<dyn ModeHandler + Send>,
}

struct App {
    config: Arc<ArcSwap<Config>>,
    state: RuntimeState,
    handlers: Handlers,
    _watcher: RecommendedWatcher,
}

impl App {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = Arc::new(ArcSwap::from_pointee(Config::load_or_default()));
        let watcher = watch_config(Config::config_path(), config.clone())?;
        let cfg = config.load_full();
        let client = OscClient::new(cfg.bind_address, cfg.host_address)?;

        Ok(Self {
            config,
            state: RuntimeState {
                client: Some(client),
            },
            handlers: Handlers {
                interval: Box::new(SyncMode),
                on_change: Box::new(OnChange::new()),
            },
            _watcher: watcher,
        })
    }
}
