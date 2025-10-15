# Eternine - Color Detection Auto-Clicker

A high-performance Rust application that captures a small region in the center of the screen, detects specific colors, and automatically clicks when the target color is found.

## Features

- **Fast Screen Capture**: Uses xcap for efficient screen capture with minimal delay
- **Small Region Detection**: Captures a small 100x100 pixel region in the center of the screen for optimal performance
- **Color Detection**: Detects colors based on hex values with configurable tolerance
- **Auto-Click**: Automatically clicks when the target color is detected using enigo
- **Performance Monitoring**: Displays FPS counter and capture performance metrics
- **Cooldown System**: Prevents multiple rapid clicks with configurable cooldown
- **Capture Saving**: Automatically saves capture frames to folder for debugging

## Requirements

- Rust 2021 edition
- Windows 10/11 (enigo has platform-specific dependencies)

## Installation

1. Clone the repository
2. Run `cargo build --release`
3. The executable will be located at `target/release/eternine.exe`

## Usage

1. Run the application: `cargo run`
2. The application will start monitoring for the target color (default: #FF0000 - red)
3. When the color is detected in the center region, it will automatically click at that position
4. Capture frames are automatically saved to the `captures/` folder for debugging
5. Press Ctrl+C to stop the application

## Configuration

To change the target color, modify the `target_hex` variable in `src/main.rs`:

```rust
let target_hex = "#FF0000"; // Change to your desired hex color
```

### Capture Saving

The application automatically saves screen captures to help with debugging:
- Regular saves: Every 100 frames
- Event saves: When target color is detected
- Location: `captures/` folder
- Format: PNG with timestamp and frame number

## Performance

The application is optimized for speed:
- Captures only a small 100x100 pixel region in the center of the screen
- Uses efficient color detection algorithms
- Implements adaptive delay based on loop performance
- Monitors and displays FPS counter

## Architecture

The application is structured into three main modules:

- **`main.rs`**: Main application loop and orchestration
- **`screen_capture.rs`**: Screen capture functionality using xcap
- **`color_detector.rs`**: Color detection and matching algorithms

## License

This project is open source and available under the MIT License.