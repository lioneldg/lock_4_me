# 🚀 Build Guide - Modular Platform Builds

This project uses a modular build system with separate workflows for each platform, allowing you to build only what you need and save GitHub Actions minutes.

## 📋 Available Workflows

### Individual Platform Builds

- **🐧 `build-linux`** - Builds for Ubuntu/Linux (.deb, .AppImage)
- **🪟 `build-windows`** - Builds for Windows (.msi, .exe)
- **🍎 `build-macos`** - Builds for macOS (.dmg, .app) - Universal binary

### Complete Build

- **🌍 `build-all-platforms`** - Triggers all platform builds with the same version

## 🎯 How to Use

### Option 1: Build Individual Platforms

1. Go to **Actions** tab in GitHub
2. Select the workflow you want (e.g., `build-linux`)
3. Click **"Run workflow"**
4. Fill in the parameters:
   - **Version**: `1.0.0` (your version number)
   - **Release type**: `draft`, `prerelease`, or `release`
   - **Create new release**: `true` for first build, `false` to add to existing

### Option 2: Build All Platforms at Once

1. Go to **Actions** tab in GitHub
2. Select **`build-all-platforms`**
3. Click **"Run workflow"**
4. Fill in the parameters:
   - **Version**: `1.0.0` (same version for all platforms)
   - **Release type**: `draft`, `prerelease`, or `release`

## 🔄 Version Management

### Same Version Across Platforms

- All workflows use the same version format: `app-v{version}`
- Example: Version `1.0.0` creates tag `app-v1.0.0`
- Multiple platforms can contribute to the same release

### Release Logic

- **First build** (create_new_release: true): Creates new release
- **Additional builds** (create_new_release: false): Adds to existing release
- **build-all-platforms**: Automatically handles this logic

## 💰 Cost Optimization

### Minutes Usage (approximate)

- **Linux**: ~3-5 minutes (cheapest)
- **Windows**: ~5-8 minutes (medium cost)
- **macOS**: ~8-15 minutes (most expensive)

### Strategies

1. **Development**: Use `build-linux` for quick testing
2. **Beta releases**: Use individual builds as needed
3. **Production**: Use `build-all-platforms` for complete releases

## 📦 Release Artifacts

Each platform produces:

### Linux (Ubuntu 22.04)

- `.deb` package for Debian/Ubuntu
- `.AppImage` universal Linux binary

### Windows (Latest)

- `.msi` installer
- `.exe` portable executable

### macOS (Latest)

- `.dmg` disk image installer
- `.app` application bundle
- Universal binary (Intel + Apple Silicon)

## 🛠️ Examples

### Quick Linux Test Build

```
Workflow: build-linux
Version: 1.0.0-beta
Release type: draft
Create new release: true
```

### Add Windows to Existing Release

```
Workflow: build-windows
Version: 1.0.0-beta (same as Linux)
Release type: draft
Create new release: false
```

### Complete Production Release

```
Workflow: build-all-platforms
Version: 1.0.0
Release type: release
```

## 🎉 Benefits

✅ **Flexible**: Build only what you need  
✅ **Cost-effective**: Save GitHub Actions minutes  
✅ **Unified versioning**: Same version across all platforms  
✅ **Incremental**: Add platforms to existing releases  
✅ **Parallel**: All platforms build simultaneously when using build-all

Happy building! 🚀
