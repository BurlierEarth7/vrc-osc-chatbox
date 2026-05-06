# VRChat OSC Chatbox

This is a simple project designed to allow for hot-swappable chatbox inputs using playerctl.

>[!WARNING]
> playerctl is a required dependency to run this application. If it is not installed, the application will not run.

## Running

> [!WARNING]
> `config.toml` is required in the CURRENT directory to run. Without it, the project will return an IO failure.

To run the project, either use `cargo run`, or if using a prebuilt version, run the binary executable (e.g. `./vrc-osc-chatbox`)

## Configuration

> [!IMPORTANT]
> All configuration options must be present, this implementation may be improved in future releases

The following configuration options are available:

```toml
# "sync" - Synchronize to music, will update in intervals
# "swap" - Display on song change, will only send once until the song is changed
display_mode

# A list of players to listen to (e.g. "spotify,vlc")
players

# The UDP bind address (if unsure, set to "0.0.0.0:0") (See https://doc.rust-lang.org/stable/std/net/struct.UdpSocket.html#method.bind)
bind_address

# The hosting address, this is the local IP of your computer (if unsure, set to "127.0.0.1:9000")
host_address

# The message to display when Sync is enabled
# Text is taken as literal, with the exception of {length} and {position}
# To use playerctl metadata, use {meta} and set the meta_format variable
sync_message

# The delay in seconds between message updates
sync_refresh_interval_seconds

# The message to display when Swap is enabled
# Text is taken as literal, with the exception of {length} and {position}
# To use playerctl metadata, use {meta} and set the meta_format variable
on_change_message

# If set to false, a dialogue box will be displayed to allow you to edit or modify the message before sending
send_immediately

# Trigger a notify sfx callback (see https://docs.vrchat.com/docs/osc-as-input-controller#chatbox)
notify_on_send

# The format for the {meta} option, use this in sync_message or on_change_message to display playerctl information (e.g. {{title}} or {{artist}})
meta_format
```

### Example configuration

>[!NOTE]
> Configuration is hot swappable, and can be reloaded during runtime.

```toml
display_mode = "sync"
players = "spotify,vlc,firefox"

bind_address = "0.0.0.0:0"
host_address = "127.0.0.1:9000"

sync_message = "{meta} [{position}/{length}]"
sync_refresh_interval_seconds = 1

on_change_message = "Now Playing: {meta}"

meta_format = "{{title}} - {{artist}}"

send_immediately = true
notify_on_send = false
```

---

>[!CAUTION]
> This project was created with the **assistance** of generative AI as an introduction to Rust, and to OSC. Due to this, bugs may be present. This project should ***NOT*** be used in production code, and instead should be a demonstration of OSC capabilities using Rust. All code has been manually written by a human, AI was only used to learn key Rust and OSC concepts.
