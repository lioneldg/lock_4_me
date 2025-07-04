# Backend Rust Tests - Tauri Lock-4-Me Application

This document describes the complete test suite developed for the Tauri Lock-4-Me application backend (Rust).

## Test Structure

### 1. Unit Tests by Module

#### `src-tauri/src/read_write_settings.rs`
- **Coverage**: Settings management (JSON read/write)
- **Tests included**:
  - Settings serialization/deserialization
  - File saving and loading
  - Nested directory creation
  - Error handling (non-existent files, invalid JSON)
  - Tauri command tests for `write_settings` and `read_settings`

#### `src-tauri/src/lock_screen.rs`
- **Coverage**: Cross-platform screen locking
- **Tests included**:
  - Command validation for each platform (Linux, macOS, Windows)
  - Command structure and format
  - Error handling
  - Linux desktop environment specific tests
  - Command argument validation

#### `src-tauri/src/listen_bluetooth.rs`
- **Coverage**: Bluetooth management and event listening
- **Tests included**:
  - Custom Bluetooth error handling
  - RSSI calculations and threshold logic
  - UUID validation
  - JSON event structure
  - Timeout and reconnection logic
  - Asynchronous tests with Tokio

#### `src-tauri/src/lib.rs`
- **Coverage**: Main Tauri application configuration
- **Tests included**:
  - Bluetooth handle management
  - Log configuration
  - Window visibility logic
  - System tray configuration
  - Menu event handling
  - Activation policies (macOS)

#### `crates/bt_discover/src/lib.rs`
- **Coverage**: Bluetooth discovery crate
- **Tests included**:
  - `DiscoveredDevice` structure creation and manipulation
  - UUID filtering logic
  - Bluetooth event type handling
  - RSSI value validation
  - Bluetooth adapter error handling
  - UUID parsing tests

### 2. Integration Tests

#### `src-tauri/tests/integration_tests.rs`
- **Coverage**: Inter-module tests and complete workflows
- **Tests included**:
  - Complete settings workflow (write → read → validation)
  - Inter-module compatibility (settings UUID ↔ Bluetooth module)
  - Cross-module error handling
  - Persistence with special characters
  - Bluetooth handle lifecycle
  - Error message consistency
  - Integrated RSSI calculations
  - Integrated asynchronous tests
  - Complete serialization/deserialization

## Execution Commands

### Run all tests
```bash
cd src-tauri
cargo test
```

### Run specific tests by module
```bash
# Settings tests
cargo test read_write_settings

# Screen lock tests
cargo test lock_screen

# Bluetooth tests
cargo test listen_bluetooth

# Main library tests
cargo test lib

# bt_discover crate tests
cd ../crates/bt_discover
cargo test
```

### Run integration tests only
```bash
cd src-tauri
cargo test integration_tests
```

### Run with detailed output
```bash
cargo test -- --nocapture
```

### Run ignored tests (system integration tests)
```bash
cargo test -- --ignored
```

## Test Dependencies

The following dependencies have been added for testing:

### `src-tauri/Cargo.toml`
```toml
[dev-dependencies]
tokio-test = "0.4"      # Asynchronous tests
mockall = "0.12"        # Mocking (for future tests)
tempfile = "3.8"        # Temporary files for tests
serial_test = "3.0"     # Sequential tests if necessary
```

### `crates/bt_discover/Cargo.toml`
```toml
[dev-dependencies]
tokio-test = "0.4"      # Asynchronous tests
mockall = "0.12"        # Mocking
```

## Test Coverage

### Tested Features

1. **Settings Management** ✅
   - JSON read/write
   - Data validation
   - File error handling

2. **Screen Locking** ✅
   - Cross-platform support
   - Command validation
   - System error handling

3. **Bluetooth** ✅
   - Device discovery
   - RSSI calculations
   - Event handling
   - UUID validation

4. **Tauri Configuration** ✅
   - Application setup
   - Window management
   - System configuration

5. **Integration** ✅
   - Complete workflows
   - Inter-module communication
   - Data persistence

### Test Types

- **Unit Tests**: ~50 tests covering each function/module
- **Integration Tests**: ~12 tests covering complete workflows
- **Asynchronous Tests**: Tests with Tokio for async operations
- **Cross-platform Tests**: Linux/macOS/Windows specific tests
- **Error Tests**: Validation of all error cases

## Special Test Cases

### Tests Ignored by Default
Some tests are marked `#[ignore]` because they:
- Require system permissions (screen locking)
- Require Bluetooth hardware
- May interfere with the development environment

To run them: `cargo test -- --ignored`

### Platform-Conditional Tests
- Linux tests: `#[cfg(target_os = "linux")]`
- macOS tests: `#[cfg(target_os = "macos")]`
- Windows tests: `#[cfg(target_os = "windows")]`

## CI/CD Tests

Tests are designed to work in CI/CD environments:
- No external dependencies (except Bluetooth for specific tests)
- Use of temporary files
- Appropriate error handling for headless environments

## Future Improvements

1. **Advanced mocking**: Use of `mockall` to simulate Bluetooth APIs
2. **Performance tests**: Benchmarks for critical operations
3. **Stress tests**: Load testing for Bluetooth streams
4. **Security tests**: Validation of malicious inputs

## Maintenance

- Tests must be updated with each API modification
- New tests required for each new feature
- Periodic test coverage review with `cargo tarpaulin`