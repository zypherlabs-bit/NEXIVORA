# macOS Installation

## System Requirements

### Minimum Requirements
- **Operating System**: macOS 10.15 (Catalina) or later
- **Processor**: Intel Core i5 or Apple Silicon M1
- **RAM**: 4 GB (8 GB recommended)
- **Storage**: 500 MB available space
- **Display**: 1280×800 resolution or higher

### Recommended Requirements
- **Operating System**: macOS 12.0 (Monterey) or later
- **Processor**: Apple Silicon M1/M2 or Intel Core i7/i9
- **RAM**: 16 GB or more
- **Storage**: 1 GB available space (SSD recommended)
- **Display**: Retina display or equivalent

### Supported Versions

**Officially Supported**:
- macOS 10.15 (Catalina)
- macOS 11 (Big Sur)
- macOS 12 (Monterey)
- macOS 13 (Ventura)
- macOS 14 (Sonoma)

## Installation Methods

### DMG Installer (Recommended)

1. **Download** the appropriate DMG file from [GitHub Releases](https://github.com/zypherlabs-bit/NEXIVORA/releases)
   - For Apple Silicon: `Nexivora-*-macOS-AppleSilicon.dmg`
   - For Intel: `Nexivora-*-macOS-Intel.dmg`
   - Universal (both): `Nexivora-*-macOS-Universal.dmg`

2. **Open** the DMG file by double-clicking
3. **Drag** Nexivora to your Applications folder
4. **Eject** the DMG volume
5. **Launch** Nexivora from Applications or Spotlight

### First Launch (Gatekeeper)

When launching for the first time:
1. You may see: "Nexivora cannot be opened because the developer cannot be verified"
2. Click "OK"
3. Open System Preferences > Security & Privacy
4. Under "General" tab, click "Open Anyway" next to the Nexivora warning
5. Confirm by clicking "Open"

## Verification

### Checksum Verification

After downloading, verify the integrity:

```bash
# Open Terminal
shasum -a 256 ~/Downloads/Nexivora-*.dmg
```

Compare the output with the SHA-256 checksum listed in the release notes.

### Notarization

Nexivora is signed and notarized by Apple **when** Developer ID credentials are configured for the release build
(`APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_APP_PASSWORD`, `APPLE_TEAM_ID`, `APPLE_P12_BASE64`). You can verify:

```bash
# Check notarization status
spctl -a -v ~/Applications/Nexivora.app
```

If notarized, it should return: `source=Notarized Developer ID`

If the build was not signed (e.g. open-source CI without paid Apple Developer credentials), you may see
"developer cannot be verified" — use the Gatekeeper "Open Anyway" instructions below.

## Troubleshooting

### Gatekeeper Issues

**Problem**: "App is damaged and can't be opened"
**Solution**:
```bash
# Remove quarantine flag
xattr -d com.apple.quarantine ~/Applications/Nexivora.app

# Try launching again
open ~/Applications/Nexivora.app
```

**Problem**: Persistent Gatekeeper warnings
**Solution**:
- Ensure you downloaded the official DMG from GitHub
- Check the SHA-256 checksum matches
- Temporarily reduce Gatekeeper security (not recommended):
  ```bash
  sudo spctl --master-disable
  ```
  (Re-enable after installation: `sudo spctl --master-enable`)

### Installation Issues

**Problem**: "The application cannot be opened"
**Solution**:
- Check you have sufficient permissions
- Try installing in a different location
- Check Console.app for detailed error messages

### Performance Issues

**Problem**: Slow performance on Intel Macs
**Solution**:
- Ensure you downloaded the Intel version, not Apple Silicon
- Check Activity Monitor for high CPU usage
- Try closing other applications

## Upgrading

### From Previous Versions

1. **Backup** your documents (see below)
2. **Download** the latest version
3. **Install** the new version (it will replace the old one)
4. **Verify** your documents open correctly

### Backup Important Data

Before upgrading, we recommend backing up:
- `~/Library/Application Support/Nexivora`
- `~/Library/Preferences/org.nexivora.Nexivora.plist`
- Any custom templates or documents

## Uninstallation

### Simple Uninstall

1. **Drag** Nexivora from Applications to Trash
2. **Empty** Trash

### Complete Uninstall

To remove all traces:
```bash
# Remove application
rm -rf ~/Applications/Nexivora.app

# Remove preferences and support files
rm -rf ~/Library/Application Support/Nexivora
rm -rf ~/Library/Preferences/org.nexivora.Nexivora.plist
rm -rf ~/Library/Caches/org.nexivora.Nexivora

# Remove from Dock (if present)
defaults delete com.apple.dock persistent-apps -array-add '<dict><key>tile-data</key><dict><key>file-data</key><dict><key>_CFURLString</key><string>/Applications/Nexivora.app</string><key>_CFURLStringType</key><integer>0</integer></dict></dict></dict>'
killall Dock
```

## Common Errors

### "Nexivora quit unexpectedly"
Check Console.app for crash logs and:
- Ensure you have sufficient RAM
- Update macOS to the latest version
- Try reinstalling Nexivora

### "Cannot open files"
- Check file permissions: `ls -la ~/Documents/`
- Repair disk permissions using Disk Utility
- Try opening files from within Nexivora's File menu

### "Graphics rendering issues"
- Update macOS to the latest version
- Check for graphics driver updates
- Try disabling GPU acceleration in Nexivora settings

## Apple Silicon Specific

### Rosetta Issues
If you accidentally installed the Intel version on Apple Silicon:
1. Uninstall the Intel version
2. Download the Apple Silicon version
3. Install normally

### Universal Binary
The Universal version contains both Intel and Apple Silicon binaries:
```bash
# Check architectures
file ~/Applications/Nexivora.app/Contents/MacOS/Nexivora
```
Should show: `Mach-O universal binary with 2 architectures: [x86_64:Mach-O 64-bit executable x86_64] [arm64:Mach-O 64-bit executable arm64]`

## Support

For additional help:
- Check our [FAQ](https://github.com/zypherlabs-bit/NEXIVORA/wiki/FAQ)
- Open an [Issue](https://github.com/zypherlabs-bit/NEXIVORA/issues)
- Join our [Community Discussions](https://github.com/zypherlabs-bit/NEXIVORA/discussions)