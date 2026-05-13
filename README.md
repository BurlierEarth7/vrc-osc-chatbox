# VRChat OSC Chatbox

>[!CAUTION]
> This project is only available for UNIX operating systems which support playerctl + MPRIS via D-Bus. It is not intended to be used professionally. This project was originally designed to be a learning experience for Rust and OSC, and will remain so. Expect bugs, and janky code.

This is a simple project designed to allow for hot-swappable chatbox inputs using playerctl.

>[!WARNING]
> playerctl is a required dependency to run this application. If it is not installed, the application will not run.

## Running

To run the project, either use `cargo run`, or if using a prebuilt version, run the binary executable (e.g. `./vrc-osc-chatbox`)

## Configuration

> [!NOTE]
> On first run, your config will auto populate with default data. If this data ever becomes invalid, or is removed from your config, it will revert to this default value

The following configuration options are available:

```toml
# "sync" - Synchronize to music, will update in intervals
# "swap" - Display on song change, will only send once until the song is changed
display_mode

# A list of players to listen to (e.g. "spotify,vlc")
players

# The UDP bind address 
# (if unsure, set to "0.0.0.0:0")
# (See https://doc.rust-lang.org/stable/std/net/struct.UdpSocket.html#method.bind)
bind_address

# The hosting address, this is the local IP of your computer
# (if unsure, set to "127.0.0.1:9000")
host_address

# The message to display when Sync is enabled
# Uses playerctl formatting (e.g. {{title}}, {{position}}, {{length}}, etc...)
# position and length are specially formatted to be in mins + seconds, rather than ms
sync_message

# The delay in seconds between message updates
sync_refresh_interval_seconds

# The message to display when Swap is enabled
# Uses playerctl formatting (e.g. {{title}}, {{position}}, {{length}}, etc...)
# position and length are specially formatted to be in mins + seconds, rather than ms
swap_message

# If set to false, a dialogue box will be displayed
# allowing you to edit or modify the message before sending
send_immediately

# Trigger a notify sfx callback
# (see https://docs.vrchat.com/docs/osc-as-input-controller#chatbox)
# Not very useful unless you're using it for your own project
notify_on_send

```

### Example configuration

>[!NOTE]
> Configuration is hot swappable, and can be reloaded during runtime.

```toml
display_mode = "sync"
players = "spotify,vlc,firefox"

bind_address = "0.0.0.0:0"
host_address = "127.0.0.1:9000"

sync_message = "{{title}} - {{artist}} [{{position}}/{{length}}]"
sync_refresh_interval_seconds = 1

swap_message = "Now Playing: {{title}} - {{artist}}"

send_immediately = true
notify_on_send = false
```

---

>[!CAUTION]
> This project was created with the **assistance** of generative AI as an introduction to Rust, and to OSC. Due to this, bugs may be present. This project should ***NOT*** be used in production code, and instead should be a demonstration of OSC capabilities using Rust. All code has been manually written by a human, AI was only used to learn key Rust and OSC concepts.
