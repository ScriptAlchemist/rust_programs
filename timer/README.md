# timer

A terminal countdown timer and stopwatch written in Rust. It can run as a full-screen terminal UI or in the tmux status line without requiring a pane to stay open.

## Install

From this repository:

```bash
cargo install --path /Users/justinbender/projects/rust_programs/timer
```

Or run without installing:

```bash
cargo run -- 25m
```

## Terminal Usage

Run a timer:

```bash
timer 25m
timer 1h30m
timer 90s
timer 90
timer 1:30
```

Start a stopwatch that counts up until stopped:

```bash
timer stopwatch
timer --stopwatch
timer stopwatch --color green
```

Stop the timer or stopwatch while it is running:

```text
q
Esc
Ctrl-C
```

Supported duration formats:

```text
25m       25 minutes
1h30m     1 hour, 30 minutes
1h30m5s   1 hour, 30 minutes, 5 seconds
90s       90 seconds
90        90 seconds
1:30      1 minute, 30 seconds
1:02:03   1 hour, 2 minutes, 3 seconds
```

## Colors

Red is the default. Set a color with `--color` or `-c`:

```bash
timer 25m --color green
timer --color blue 90s
timer -c cyan 1:30
timer 1h30m --color light-blue
```

Supported colors:

```text
black
red
green
yellow
blue
magenta
cyan
gray
dark-gray
white
light-red
light-green
light-yellow
light-blue
light-magenta
light-cyan
```

Aliases:

```text
purple -> magenta
grey -> gray
dark-grey -> dark-gray
bright-* -> light-*
```

## tmux Status-Line Mode

Use `--tmux` to show the countdown in `status-right` instead of opening the full-screen UI:

```bash
timer --tmux 25m
timer --tmux --color green 25m
timer --tmux -c cyan 1:30
timer --tmux stopwatch
```

For background tmux launches, use `--quiet`:

```bash
tmux run-shell -b 'timer --tmux --quiet --tmux-target "#{session_name}" --color green 25m'
tmux run-shell -b 'timer --tmux --quiet --tmux-target "#{session_name}" stopwatch'
```

The timer or stopwatch stores its background process id in the current tmux session option `@timer_pid`, so it can be canceled later:

```bash
pid="$(tmux show-option -qv -t "#{session_name}" @timer_pid)"
kill -TERM "$pid"
```

Pause, resume, and restart are controlled through tmux options:

```bash
tmux set-option -q -t "#{session_name}" @timer_control pause
tmux set-option -q -t "#{session_name}" @timer_control resume
tmux set-option -q -t "#{session_name}" @timer_control restart
```

Each tmux session gets its own `status-right`, `@timer_status`, `@timer_pid`, `@timer_control`, and `@timer_paused` state. That means you can run one timer or stopwatch in session `work` and another in session `personal` without them overwriting each other. The included tmux bindings pass `--tmux-target "#{session_name}"` for this reason.

On normal completion, cancel, `SIGHUP`, `SIGTERM`, or `SIGINT`, the program restores the original `status-right`. It cannot clean up after `kill -9` or if the tmux server itself dies.

## tmux Key Bindings

This repo includes [tmux/timer.tmux.conf](tmux/timer.tmux.conf). After installing the binary, add this to `~/.tmux.conf`:

```tmux
source-file /Users/justinbender/projects/rust_programs/timer/tmux/timer.tmux.conf
```

Reload tmux:

```bash
tmux source-file ~/.tmux.conf
```

Default bindings from the included snippet:

```text
prefix + T       prompt for duration and start timer
prefix + Ctrl-t  prompt for duration and color, then start timer
prefix + S       start stopwatch
prefix + P       pause or resume active timer/stopwatch
prefix + R       restart active timer or reset stopwatch
prefix + X       cancel active timer or stopwatch
```

With a `Ctrl-a` prefix, those are:

```text
Ctrl-a then Shift-t
Ctrl-a then Ctrl-t
Ctrl-a then Shift-s
Ctrl-a then Shift-p
Ctrl-a then Shift-r
Ctrl-a then Shift-x
```

## Manual tmux Config

If you prefer to paste the bindings directly into `~/.tmux.conf`, use:

```tmux
set -g status on
set -g status-interval 1
set -g status-right-length 80

bind-key T command-prompt -p "timer duration" \
  "run-shell -b '/Users/justinbender/.cargo/bin/timer --tmux --quiet --tmux-target \"#{session_name}\" %%'"

bind-key C-t command-prompt -p "timer duration,timer color" \
  "run-shell -b '/Users/justinbender/.cargo/bin/timer --tmux --quiet --tmux-target \"#{session_name}\" --color %2 %1'"

bind-key S run-shell -b '/Users/justinbender/.cargo/bin/timer --tmux --quiet --tmux-target "#{session_name}" stopwatch'

bind-key P run-shell -b 'pid="$(tmux show-option -qv -t "#{session_name}" @timer_pid)"; if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then paused="$(tmux show-option -qv -t "#{session_name}" @timer_paused)"; if [ "$paused" = "1" ]; then tmux set-option -q -t "#{session_name}" @timer_control resume; tmux display-message "Timer/stopwatch resumed"; else tmux set-option -q -t "#{session_name}" @timer_control pause; tmux display-message "Timer/stopwatch paused"; fi; else tmux display-message "No timer or stopwatch running"; fi'

bind-key R run-shell -b 'pid="$(tmux show-option -qv -t "#{session_name}" @timer_pid)"; if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then tmux set-option -q -t "#{session_name}" @timer_control restart; tmux display-message "Timer/stopwatch restarted"; else tmux display-message "No timer or stopwatch running"; fi'

bind-key X run-shell -b 'pid="$(tmux show-option -qv -t "#{session_name}" @timer_pid)"; if [ -n "$pid" ] && kill -0 "$pid" 2>/dev/null; then kill -TERM "$pid"; tmux display-message "Timer/stopwatch stopped"; else tmux display-message "No timer or stopwatch running"; fi'
```

## Development

Run checks:

```bash
cargo check
cargo test
```
