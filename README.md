# Virtual Keyboard

A cross-platform virtual keyboard application written in Rust using GTK.

## Prerequisites

- Rust (latest stable version)
- GTK 4 development libraries

### Platform-Specific Requirements

#### Linux
- GTK 4 development libraries
- xdotool (for simulating keyboard input)

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install libgtk-4-dev xdotool

# Fedora
sudo dnf install gtk4-devel xdotool
```

#### macOS
- GTK 4 (via Homebrew)
- cliclick (for simulating keyboard input)

```bash
brew install gtk4 cliclick
```

## Building

```bash
cargo build --release
```

## Running

```bash
cargo run --release
```

## Features

- QWERTY keyboard layout
- Number keys
- Space bar
- Cross-platform support:
  - Linux: Uses xdotool for keyboard simulation
  - macOS: Uses cliclick for keyboard simulation

## Platform Notes

### Linux
The application uses xdotool to simulate keyboard input. Make sure you have X11 or Wayland running.

### macOS
The application uses cliclick to simulate keyboard input. You may need to grant accessibility permissions to your terminal or IDE when first running the application.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
