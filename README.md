# Desktop GMT - Minimalist Time Zone Converter

A sleek, minimalist desktop application for tracking time across different time zones. Built with Tauri, Svelte, and Rust, Desktop GMT combines elegant design with efficient performance.

## Features

- **Real-time Updates**: Live time display with smooth animations
- **Multiple Time Zones**: Support for major time zones worldwide:
  - EST (New York)
  - CST (Chicago)
  - MST (Denver)
  - PST (Los Angeles)
  - GMT (London)
  - CET (Paris)
  - JST (Tokyo)
  - AEST (Sydney)
  - NZST (Auckland)
- **Customizable Themes**: Six carefully crafted color themes
  - Orange Night
  - Mint Dark
  - Purple Dream
  - Ocean Breeze
  - Forest Night
  - Rose Gold
- **Persistent Settings**: Automatically saves your preferred theme and time zone
- **Minimal Interface**: Clean, distraction-free design
- **System Integration**: Lightweight system tray application
- **Cross-Platform**: Works on Windows, macOS, and Linux

## Interface Guide

The application features a minimalist interface with four main control buttons that appear on hover:

- **Settings (⚙)**: Top-left corner - Opens the timezone selection menu
- **Close (×)**: Top-right corner - Closes the application
- **Theme Picker (🎨)**: Bottom-right corner - Opens the color theme selection menu
- **Show Seconds (🕒)**: Bottom-left corner - Toggles the display of seconds

The main display shows:
- Your local time at the top
- A dividing line
- The selected timezone's time below (when set)

## Installation

Download the latest release for your operating system:
- Windows (.msi)
- macOS (.dmg)
- Linux (.AppImage)

## Development

Prerequisites:
- Node.js (v16 or later)
- Rust (latest stable)
- Tauri CLI

Clone the repository:

``` bash
git clone https://github.com/forever8896/desktop-gmt.git

cd desktop-gmt 
```

Install dependencies:

``` bash
npm install
```

Run in development mode:

``` bash
npm run tauri dev
```

Build for production:

``` bash
npm run tauri build
```

## Technical Stack

- **Frontend**: Svelte
- **Backend**: Rust + Tauri
- **Styling**: CSS3 with custom properties
- **Animations**: CSS transitions and keyframes
- **Data Persistence**: JSON file storage
- **Font**: Inter (300 weight)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - [LICENSE](LICENSE) file for details.

## Acknowledgments

- Time zone data provided by IANA Time Zone Database
- Built with Tauri
- UI powered by Svelte
- Font by Inter

---

Made with ❤️ by Brian Pistar