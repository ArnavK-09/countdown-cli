# ⏰ countdown_terminal

A terminal-based countdown timer application built with **ratatui** and **Rust**, featuring a customizable countdown time, progress bar, and keyboard shortcuts for user interaction.

## Features

- **Customizable Countdown Time:** Set the countdown duration from command line arguments (e.g., `60s` for 60 seconds, `1m` for 1 minute).
- **Progress Visualization:** See the countdown progress in real-time with a dynamic progress bar.
- **Keyboard Shortcuts:** Use `Esc` or `Ctrl+C` to stop the countdown.
- **Responsive TUI:** Interactive terminal UI with real-time updates.

## Screenshots

> Include a screenshot of the app here.

## Usage

###### terminal

```bash
npx countdown-terminal 60s
```

## Locally Build and Run

First, build the project using Cargo:

```bash
cargo build --release
```

Then, run the app with:

```bash
cargo run
```

You can specify the countdown time as a command line argument, such as:

```bash
cargo run 1m  # Set a countdown of 1 minute
```

### Default Countdown Time

If no argument is provided, the app defaults to a 60-second countdown.

## Keybindings

- **Start Countdown:** Simply run the application and the countdown begins automatically.
- **Stop Countdown:** Press `Esc` or `Ctrl+C` to stop the countdown and exit the app.

## Key Features

- **Command-Line Arguments:**
  - You can provide the countdown time as an argument (e.g., `60s`, `1m`, `1h`).
- **Dynamic Progress Bar:** A progress bar updates in real-time as the countdown progresses.
- **Keyboard Shortcuts:**
  - `Esc` or `Ctrl+C` to exit the countdown.

### Contributing

We welcome contributions! If you'd like to contribute to the project, follow these steps:

1. Fork the repository.
2. Clone your forked repository.
3. Create a new branch for your feature or bug fix.
4. Commit your changes.
5. Push your changes to your fork.
6. Open a pull request to the main repository.
