# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build and Run Commands

```bash
# Build the project
cargo build --release

# Run the application
cargo run --release

# Run tests
cargo test
```

## Architecture Overview

This is a GTK4-based virtual keyboard application written in Rust with cross-platform input simulation.

### Module Structure

- **main.rs**: GTK4 application initialization and window setup. Creates a non-focusable window to prevent stealing focus from target applications when buttons are clicked.
- **keyboard.rs**: UI layer that creates the keyboard grid layout and buttons. Each button is configured with `can_focus(false)` and `focus_on_click(false)` to maintain focus in the target application.
- **input.rs**: Platform abstraction layer for simulating keyboard input via external tools.

### Input Simulation Strategy

The application uses external command-line tools to simulate keyboard input:

- **Linux**: Uses `ydotool type` (not `xdotool key`) for Wayland compatibility
- **macOS**: Uses AppleScript via `osascript` to control TextEdit

**Critical: Linux Wayland Note**
- The code currently uses `ydotool` which requires the `ydotoold` daemon (not included in Ubuntu's ydotool package)
- Alternative tools for Wayland: `wtype` (recommended), `dotool`
- X11 systems can use `xdotool key` instead

### Testing Strategy

Tests use a file-based simulation approach rather than actual keyboard input:
- Test builds set a `test_file` path on `InputSimulator`
- Instead of calling external tools, tests write characters to a file
- This allows unit testing without GUI dependencies or sudo permissions

### Window Focus Behavior

The window is configured to never accept focus:
- `window.set_can_focus(false)` and `window.set_focus_on_click(false)` in main.rs
- All buttons set `can_focus(false)` and `focus_on_click(false)` in keyboard.rs
- `toplevel.focus(0)` called after window presentation to explicitly refuse focus
- This design allows users to click keyboard buttons while maintaining focus in their target application

## Platform-Specific Dependencies

### Linux
- `libgtk-4-dev`: GTK4 development libraries
- `ydotool` or `wtype`: Wayland input simulation (choose one)
- `xdotool`: X11 input simulation (legacy)

### macOS
- `gtk4`: via Homebrew
- `cliclick`: for keyboard simulation (not currently used - code uses AppleScript instead)
