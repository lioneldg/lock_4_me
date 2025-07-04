# Lock-4-Me

> **⚠️ Disclaimer**: This application is a prototype/mockup that was created to experiment with Tauri2. It will not be improved or maintained further and should not be used as a production tool. However, you are welcome to fork this project if you're interested in exploring the codebase.

**Lock-4-Me** is an automatic screen locking application that uses Bluetooth proximity detection to secure your computer when you step away. The app monitors the signal strength (RSSI) of your paired Bluetooth device (such as your phone) and automatically locks your screen when the signal becomes too weak, indicating you've moved too far from your computer.

## Features

- 🔒 **Automatic Screen Locking**: Locks your screen when you move away from your computer
- 📱 **Bluetooth Proximity Detection**: Uses RSSI (signal strength) to determine distance
- 🖥️ **Cross-Platform**: Supports macOS, Linux, and Windows
- 🎨 **Modern UI**: React-based interface with light/dark theme support
- 🌐 **Multilingual**: English and French language support
- ⚙️ **Configurable**: Adjustable RSSI threshold and target device selection
- 🔄 **Discovery Mode**: Scan and select nearby Bluetooth devices
- 📊 **System Tray**: Runs in background with system tray icon

## Technology Stack

### Frontend

- **React 18** with TypeScript
- **Vite** for fast development and building
- **Zustand** for state management
- **React Router** for navigation
- **i18next** for internationalization
- **CSS Modules** for styling

### Backend

- **Rust** with Tauri 2 framework
- **tokio** for asynchronous operations
- **btleplug** for Bluetooth Low Energy communication
- **serde** for JSON serialization
- **Custom Bluetooth discovery crate**

### Development Tools

- **ESLint** & **Prettier** for code formatting
- **Jest** & **Testing Library** for frontend tests
- **Comprehensive Rust test suite** (55+ tests)
- **Knip** for unused code detection

## Installation

### Prerequisites

- **Node.js** (version 16 or higher)
- **Yarn** package manager
- **Rust** (latest stable version)
- **Tauri CLI** (will be installed automatically)

### Setup

1. Clone the repository

```bash
git clone <repository-url>
cd lock-4-me
```

2. Install dependencies

```bash
yarn install
```

3. Run in development mode

```bash
yarn tauri dev
```

4. Build for production

```bash
yarn tauri build
```

## Usage

1. **Launch the application** - It will appear in your system tray
2. **Click the tray icon** to open the main window
3. **Enter Discovery Mode** to scan for nearby Bluetooth devices
4. **Select your target device** (e.g., your smartphone)
5. **Configure the RSSI threshold** in settings if needed
6. **Close the window** - the app continues running in the background
7. **Walk away** from your computer - the screen will lock automatically when your device is out of range

## Configuration

- **Target UUID**: The Bluetooth device to monitor
- **RSSI Threshold**: Signal strength threshold for triggering lock (default: -15 dBm)
- **Theme**: Light or dark mode
- **Language**: English or French

## Development

### Available Scripts

```bash
# Development
yarn dev                 # Start Vite dev server
yarn tauri dev          # Run Tauri in development mode

# Building
yarn build              # Build frontend
yarn tauri build        # Build complete application

# Testing
yarn test               # Run frontend tests
yarn test:watch         # Run tests in watch mode
yarn test:coverage      # Run tests with coverage

# Code Quality
yarn lint               # Run ESLint
yarn format             # Format code with Prettier
yarn knip               # Detect unused code
```

### Testing

The project includes comprehensive test coverage:

- **Frontend**: Jest with React Testing Library
- **Backend**: 55+ Rust unit and integration tests
- **Cross-platform**: Tests for macOS, Linux, and Windows functionality

Run tests with:

```bash
yarn test                    # Frontend tests
cargo test                   # Backend tests (from src-tauri/)
```

## Architecture

```
lock-4-me/
├── src/                     # React frontend
│   ├── components/          # Reusable UI components
│   ├── views/              # Main application views
│   ├── store/              # Zustand state management
│   ├── hooks/              # Custom React hooks
│   └── libs/               # Utilities and i18n
├── src-tauri/              # Rust backend
│   ├── src/                # Tauri application code
│   └── capabilities/       # Tauri permissions
└── crates/                 # Custom Rust crates
    └── bt_discover/        # Bluetooth discovery logic
```

## Permissions

On macOS, the app requires:

- **Bluetooth permission** to scan for nearby devices
- **Screen lock permission** to lock the screen automatically

## Platform Support

- **macOS**: Full support with native screen locking
- **Linux**: Supports various desktop environments (GNOME, KDE, etc.)
- **Windows**: Native Windows screen locking

## Contributing

Since this is a prototype that won't be maintained, contributions are not accepted. However, feel free to fork the project and create your own version!

## License

This project is provided as-is for educational and experimental purposes.
