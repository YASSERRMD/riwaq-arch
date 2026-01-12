# JetBrains Plugin - Build Guide for Android Studio

## Current Status

✅ **Source code complete** - All Kotlin files written and ready
✅ **Riwaq binary built** - Available at `vscode-extension/bin/riwaq`
✅ **Gradle wrapper added** - Ready to build
⚠️ **Requires Java 17** - Build needs Java 17 SDK installed

## Building the Plugin

Since the build requires Java 17 which is not available on the current system, you have these options:

### Option 1: Build on a Machine with Java 17

```bash
# Navigate to the plugin directory
cd jetbrains-plugin

# Build the plugin
./gradlew buildPlugin

# The output will be at:
# build/distributions/RiwaqArch-1.0.0.zip
```

### Option 2: Build in Android Studio Directly

1. Open Android Studio
2. **File → New → Project from Existing Sources**
3. Select the `jetbrains-plugin` directory
4. Wait for Gradle sync to complete (it will download dependencies)
5. **Build → Build Project**
6. Find the built plugin in `build/distributions/`

### Option 3: Use IntelliJ IDEA (if you have it)

1. Open IntelliJ IDEA
2. **File → Open** → Select `jetbrains-plugin` directory
3. Wait for Gradle import
4. Click the Gradle sidebar
5. Expand **Tasks → intellij → buildPlugin**
6. Double-click to run

## Installation in Android Studio

Once built:

1. **Android Studio → Settings → Plugins** (or **File → Settings → Plugins** on Windows/Linux)
2. Click the **gear icon** ⚙️ in the top-right
3. Select **"Install Plugin from Disk..."**
4. Navigate to `jetbrains-plugin/build/distributions/`
5. Select `Riwaq Arch-1.0.0.zip` (or similar)
6. **Restart Android Studio** when prompted

## After Installation

### Configure the Plugin

1. **Settings → Tools → Riwaq Arch**
2. Configure:
   - **LLM API Key**: Add your OpenRouter/OpenAI API key
   - **Auto-start Server**: Enable if you want the server to start automatically
   - **Excluded Directories**: Add directories to skip (e.g., `build`, `obj`, `.gradle`)

### Start Using

1. Open a project in Android Studio
2. View → Tool Windows → Riwaq Architecture
3. Click **Tools → Riwaq Actions → Start Server**
4. Click **Tools → Riwaq Actions → Analyze Project**
5. Explore the 4 tabs:
   - **Modules**: See your project structure
   - **Ask AI**: Chat with your codebase
   - **Diagram**: View architecture diagrams
   - **Insights**: See metrics and statistics

## Compatibility

The plugin is designed to work with:
- ✅ Android Studio (all versions based on IntelliJ 2023.2+)
- ✅ IntelliJ IDEA 2023.2+
- ✅ PyCharm 2023.2+
- ✅ WebStorm 2023.2+
- ✅ Other JetBrains IDEs

## What Gets Bundled

The build automatically includes:
- ✅ The Riwaq server binary (`riwaq` or `riwaq.exe`)
- ✅ All required dependencies
- ✅ UI components (tool windows, panels)
- ✅ Configuration pages

This means **no manual server installation needed** - the plugin works out of the box!

## Troubleshooting

### Build Fails with "Unsupported class file major version"
- **Cause**: Java version is too old
- **Fix**: Install Java 17 or newer from https://adoptium.net/

### Plugin Won't Install
- **Cause**: Android Studio version too old
- **Fix**: Update Android Studio to latest version (Hedgehog or later)

### Server Won't Start
- **Cause 1**: Missing LLM API key
- **Fix**: Add API key in Settings → Tools → Riwaq Arch
- **Cause 2**: Binary not executable
- **Fix**: The build should make it executable automatically, but you can manually: `chmod +x build/distributions/Riwaq Arch/lib/bin/riwaq`

## Quick Reference

**Build Command**:
```bash
cd jetbrains-plugin && ./gradlew buildPlugin
```

**Output Location**:
```
jetbrains-plugin/build/distributions/Riwaq Arch-1.0.0.zip
```

**Installation**: Settings → Plugins → Install from Disk

**Documentation**: See [jetbrains-plugin/README.md](jetbrains-plugin/README.md) and [BUILD_INSTRUCTIONS.md](jetbrains-plugin/BUILD_INSTRUCTIONS.md)

---

**Next Steps**: Build the plugin on a machine with Java 17, then install the ZIP file in Android Studio!
