# Windows Installation

## System Requirements

### Minimum Requirements
- **Operating System**: Windows 10 (64-bit) or later
- **Processor**: 1 GHz or faster with 2 or more cores
- **RAM**: 2 GB (4 GB recommended)
- **Storage**: 500 MB available space
- **Graphics**: DirectX 10 or later compatible

### Recommended Requirements
- **Operating System**: Windows 11 (64-bit)
- **Processor**: 2 GHz or faster with 4 or more cores
- **RAM**: 8 GB or more
- **Storage**: 1 GB available space (SSD recommended)
- **Graphics**: DirectX 12 compatible with WDDM 2.0 driver

## Installation Methods

### Standard Installer

1. **Download** the latest Windows installer from [GitHub Releases](https://github.com/zypherlabs-bit/NEXIVORA/releases)
2. **Run** the installer (e.g., `Nexivora-Windows-x64-setup.exe`)
3. **Follow** the installation wizard instructions
4. **Launch** Nexivora from the Start Menu or desktop shortcut

### Portable Version

1. **Download** the portable ZIP archive `Nexivora-Windows-Portable.zip` from [GitHub Releases](https://github.com/zypherlabs-bit/NEXIVORA/releases)
2. **Extract** the ZIP file to your preferred location
3. **Run** `Nexivora.exe` from the extracted folder
4. No installation required - runs directly from the folder

## Verification

### Checksum Verification

After downloading, verify the integrity of the installer:

1. Open PowerShell
2. Run: `Get-FileHash -Algorithm SHA256 Nexivora-Windows-x64-setup.exe`
3. Compare the output with the SHA-256 checksum listed in the release notes (the `.sha256` file next to the binary download on the GitHub Release page)

### Digital Signature

Code signing for Nexivora installers is applied when a legitimate Authenticode (Windows), Developer ID (macOS), or GPG (Linux) certificate is configured in the repository's secure secrets. 

To verify the signature of a signed Windows binary:
1. Right-click the installer file
2. Select "Properties"
3. Go to the "Digital Signatures" tab
4. Verify the signature is valid and the publisher identity matches the release description

If the "Digital Signatures" tab is not present, the build was produced without a configured signing certificate (typical for community OSS releases). In that case, rely on SHA-256 checksum verification to confirm the file has not been tampered with.

> Note: Neither Microsoft, Apple, nor any Linux distribution officially endorses or has approved Nexivora unless explicitly stated by those vendors. First-run SmartScreen/Gatekeeper warnings for unsigned or recently signed applications are normal for new open-source projects with limited reputation history.

## Troubleshooting

### Installation Issues

**Problem**: Installation fails with error code
**Solution**: 
- Ensure you have administrator privileges
- Temporarily disable antivirus software
- Check you have enough disk space
- Try the portable version instead

**Problem**: "Windows protected your PC" warning
**Solution**:
- Click "More info"
- Click "Run anyway"
- Add Nexivora to your antivirus exceptions

### Launch Issues

**Problem**: Nexivora doesn't start after installation
**Solution**:
- Check Task Manager for running Nexivora processes
- Try running as Administrator
- Reinstall the application
- Check Windows Event Viewer for error details

## Upgrading

### From Previous Versions

1. **Backup** your documents (see below)
2. **Download** the latest version
3. **Install** normally - the installer will upgrade your existing installation
4. **Verify** your documents open correctly

### Backup Important Data

Before upgrading, we recommend backing up:
- `C:\Users\<YourUsername>\Documents\Nexivora`
- `C:\Users\<YourUsername>\AppData\Roaming\Nexivora`

## Uninstallation

### Standard Uninstall

1. Open "Settings" > "Apps" > "Apps & features"
2. Find "Nexivora" in the list
3. Click "Uninstall"
4. Follow the uninstallation prompts

### Manual Cleanup

After uninstalling, you may want to remove leftover files:
- `C:\Program Files\Nexivora` (or your custom install location)
- `C:\Users\<YourUsername>\AppData\Roaming\Nexivora`
- `C:\Users\<YourUsername>\AppData\Local\Nexivora`

## Common Errors

### "MSVCRT.dll missing"
Install the latest Visual C++ Redistributable from Microsoft.

### "API-MS-WIN-CRT-RUNTIME-L1-1-0.DLL missing"
Install Windows updates, particularly KB2999226.

### "Application failed to initialize"
- Ensure your system meets minimum requirements
- Update your graphics drivers
- Try compatibility mode for Windows 10

## Support

For additional help:
- Check our [FAQ](https://github.com/zypherlabs-bit/NEXIVORA/wiki/FAQ)
- Open an [Issue](https://github.com/zypherlabs-bit/NEXIVORA/issues)
- Join our [Community Discussions](https://github.com/zypherlabs-bit/NEXIVORA/discussions)