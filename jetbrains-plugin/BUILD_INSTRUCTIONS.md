# Building the JetBrains Plugin

The JetBrains plugin requires Java 17 and the Gradle build system to compile.

## Prerequisites

1. **Java 17** or later
   - Download from: https://adoptium.net/ or https://www.oracle.com/java/technologies/downloads/
   - Set JAVA_HOME environment variable

2. **Gradle 8.5+** (will be downloaded automatically)

## Build Steps

### On macOS/Linux:

```bash
# Navigate to the plugin directory
cd jetbrains-plugin

# Make gradlew executable
chmod +x gradlew

# Build the plugin
./gradlew buildPlugin
```

### On Windows:

```cmd
cd jetbrains-plugin
gradlew.bat buildPlugin
```

## Output

After successful build, the plugin ZIP file will be at:
- `build/distributions/Riwaq Arch-1.0.0.zip`

## Installation in Android Studio

1. **Android Studio → Settings → Plugins**
2. Click the gear icon ⚙️
3. Select **"Install Plugin from Disk..."**
4. Navigate to and select the built ZIP file
5. Restart Android Studio

## Alternative: Using IntelliJ IDEA

If you have IntelliJ IDEA installed:

1. Open the `jetbrains-plugin` directory as a project
2. Wait for Gradle sync to complete
3. Click **Build → Build Project**
4. The plugin will be built automatically

## Troubleshooting

### "Unable to locate a Java Runtime"
Install Java 17 and set JAVA_HOME:
```bash
export JAVA_HOME=/path/to/java17
export PATH=$JAVA_HOME/bin:$PATH
```

### "Gradle sync failed"
- Check your internet connection (Gradle needs to download dependencies)
- Try running: `./gradlew clean --refresh-dependencies`

### Build succeeds but plugin doesn't work
- Verify IntelliJ Platform version compatibility (plugin targets 2023.2+)
- Check Android Studio version (should be 2022.2.1 or later, which is based on IntelliJ 2022.2)

## Quick Build Command

```bash
# One-line build command
cd jetbrains-plugin && ./gradlew clean buildPlugin && echo "Plugin built successfully!" && open build/distributions
```

The built plugin will include the Riwaq server binary bundled automatically during the build process.
