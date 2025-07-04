# Backend Rust Tests - Tauri Lock-4-Me Application

This document describes the complete test suite developed for the Tauri Lock-4-Me application backend (Rust).

## Test Philosophy

**Important**: All tests focus exclusively on **user-written code**. Tests do not validate external crates (uuid, serde, btleplug, etc.) as that would not be our responsibility. Tests validate our business logic, error handling, and integration between our modules.

## Test Structure

### 1. Unit Tests by Module

#### `src-tauri/src/read_write_settings.rs` (9 tests)
- **Coverage**: Settings management (JSON read/write)
- **Tests included**:
  - Settings serialization/deserialization of our custom struct
  - File saving and loading with our custom logic
  - Nested directory creation (our implementation)
  - Error handling for non-existent files, invalid JSON
  - Tauri command tests for `write_settings` and `read_settings`

#### `src-tauri/src/lock_screen.rs` (8 tests)
- **Coverage**: Cross-platform screen locking
- **Tests included**:
  - Command validation for each platform (Linux, macOS, Windows)
  - Command structure and format validation
  - Error message format consistency
  - Linux desktop environment fallback logic
  - Command argument validation
  - **Note**: 1 test is ignored (requires actual screen locking)

#### `src-tauri/src/listen_bluetooth.rs` (11 tests)
- **Coverage**: Bluetooth management and event listening
- **Tests included**:
  - Custom `BluetoothError` enum implementation
  - RSSI difference calculation logic
  - RSSI delta max threshold logic (our business logic)
  - JSON event structure (our custom format)
  - Timeout and successive timeout logic
  - Bluetooth listener handle management
  - Device name fallback logic (our implementation)

#### `src-tauri/src/lib.rs` (15 tests)
- **Coverage**: Main Tauri application configuration
- **Tests included**:
  - Bluetooth handle initialization and lifecycle
  - Log filter configuration (our custom logic)
  - Window visibility toggle logic
  - System tray configuration logic
  - Menu event handling logic
  - Error message format consistency
  - Plugin configuration validation

#### `crates/bt_discover/src/lib.rs` (9 tests)
- **Coverage**: Bluetooth discovery crate
- **Tests included**:
  - `DiscoveredDevice` structure creation and manipulation
  - UUID filtering logic (our implementation)
  - Bluetooth event type determination (our logic)
  - Error handling for no adapters (our error construction)
  - Adapter selection logic (our implementation)
  - Event processing match logic (our implementation)

### 2. Integration Tests

#### `src-tauri/tests/integration_tests.rs` (8 tests)
- **Coverage**: Inter-module tests and complete workflows
- **Tests included**:
  - Complete settings workflow (write → read → validation)
  - Inter-module compatibility (settings UUID ↔ Bluetooth module)
  - Cross-module error handling integration
  - Settings persistence with edge cases
  - Bluetooth handle lifecycle integration
  - Error message format consistency across modules
  - JSON event structure integration
  - Platform-specific functionality integration

## Execution Commands

### Run all tests
```bash
cd src-tauri
cargo test
```

### Run specific tests by module
```bash
# Settings tests only
cargo test read_write_settings

# Screen lock tests only
cargo test lock_screen

# Bluetooth tests only
cargo test listen_bluetooth

# Main library tests only
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

### `src-tauri/Cargo.toml`
```toml
[dev-dependencies]
tempfile = "3.8"        # Temporary files for settings tests
serde_json = "1.0"      # JSON testing (already in main dependencies)
uuid = "1.0"            # UUID testing (already in main dependencies)
```

### `crates/bt_discover/Cargo.toml`
```toml
[dev-dependencies]
tokio-test = "0.4"      # For async test functions
```

## Test Coverage Statistics

### Current Test Count: 55 tests
- **Unit Tests**: 52 tests
  - `read_write_settings.rs`: 9 tests
  - `lock_screen.rs`: 8 tests (1 ignored)
  - `listen_bluetooth.rs`: 11 tests
  - `lib.rs`: 15 tests
  - `bt_discover/lib.rs`: 9 tests
- **Integration Tests**: 8 tests
- **Ignored Tests**: 1 test (screen locking requires system permissions)

### Tested Features

1. **Settings Management** ✅
   - JSON serialization/deserialization of our Settings struct
   - File I/O with our custom error handling
   - Directory creation logic
   - Tauri command integration

2. **Screen Locking** ✅
   - Cross-platform command construction
   - Platform-specific logic validation
   - Error message formatting
   - Command structure validation

3. **Bluetooth Management** ✅
   - Custom error type implementation
   - RSSI calculation business logic
   - Event structure and JSON formatting
   - Timeout and reconnection logic
   - Handle lifecycle management

4. **Tauri Integration** ✅
   - Application setup logic
   - Window management logic
   - System tray configuration
   - Menu event handling
   - Plugin configuration

5. **Bluetooth Discovery** ✅
   - Device structure management
   - Filtering logic implementation
   - Event type determination
   - Error handling patterns

6. **Cross-Module Integration** ✅
   - Settings ↔ Bluetooth UUID compatibility
   - Error handling consistency
   - Event structure compatibility
   - Handle lifecycle integration

## Test Quality Principles

### What We Test
- ✅ **Our business logic**: RSSI calculations, filtering, error handling
- ✅ **Our data structures**: Settings, BluetoothError, DiscoveredDevice
- ✅ **Our integrations**: Module interactions, command handling
- ✅ **Our error handling**: Custom error types, message formatting
- ✅ **Our workflows**: Complete user scenarios

### What We Don't Test
- ❌ **External crates**: uuid parsing, serde serialization, btleplug APIs
- ❌ **Standard library**: File I/O, JSON parsing, String operations
- ❌ **Tauri framework**: Core Tauri functionality
- ❌ **System APIs**: Actual screen locking, Bluetooth hardware

### Test Categories
- **Logic Tests**: Validate our algorithms and business rules
- **Structure Tests**: Validate our data structures and their behavior
- **Integration Tests**: Validate interactions between our modules
- **Error Tests**: Validate our error handling and message formatting
- **Edge Case Tests**: Validate behavior with unusual inputs

## Special Test Cases

### Tests Ignored by Default
- `test_lock_screen_integration`: Requires system permissions and would actually lock the screen

### Platform-Conditional Tests
- Linux tests: `#[cfg(target_os = "linux")]`
- macOS tests: `#[cfg(target_os = "macos")]`
- Windows tests: `#[cfg(target_os = "windows")]`

## CI/CD Compatibility

Tests are designed to work in CI/CD environments:
- No external hardware dependencies
- Use of temporary files for I/O tests
- Appropriate error handling for headless environments
- No network dependencies
- No system permission requirements (except ignored tests)

## Running Tests

### Development Environment
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_name

# Run tests for specific module
cargo test read_write_settings
```

### CI/CD Environment
```bash
# Run all tests except ignored ones
cargo test

# Check test coverage (if using cargo-tarpaulin)
cargo tarpaulin --out html
```

## Maintenance Guidelines

1. **New Features**: Every new function/feature must have corresponding tests
2. **Bug Fixes**: Add regression tests for fixed bugs
3. **Refactoring**: Ensure tests still pass and cover the refactored code
4. **Dependencies**: Don't test external crate functionality
5. **Focus**: Always test YOUR code, not the libraries you use

## Test File Organization

```
src-tauri/
├── src/
│   ├── read_write_settings.rs    # 9 unit tests
│   ├── lock_screen.rs           # 8 unit tests (1 ignored)
│   ├── listen_bluetooth.rs      # 11 unit tests
│   └── lib.rs                   # 15 unit tests
├── tests/
│   └── integration_tests.rs     # 8 integration tests
└── ...

crates/bt_discover/
└── src/
    └── lib.rs                   # 9 unit tests
```

This test suite ensures comprehensive coverage of all user-written code while maintaining focus on validating our business logic rather than external dependencies.