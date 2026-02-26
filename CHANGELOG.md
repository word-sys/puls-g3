# Changelog

All notable changes to this project will be documented in this file

## [v0.8.0] - 2026-02-26

### Added
- **Memory Tab**: Displays Memory Type, Generation, Speed (MT/s), and Temperature, may require sudo
- **Disks Tab**: Added Read and Write rate columns for individual disks and improved NVMe detection
- **GPU Tab**: Restored Memory Clock and added PCIe version/width display
- **UI**: Added version display to top-left of the header bar
- **Layout**: Made navigation bar scrollable and status labels wrapping for better resizing

### Fixed
- **Disk Temps**: Fixed NVMe temperature detection to match original PULS reliability
- **CPU Efficiency**: Normalized load average by core count for accurate ratings
- **Sensors**: Fixed border color mismatch and data display issues

### Changed
- **CPU Cores**: Updated grid to 8 columns for better visibility
- **UI Polish**: Reduced navigation button size for a more compact look

## [v0.7.2] - 2026-02-25

### Added
- **Themes**: Added dark and light themes

### Fixed
- **CSS**: Fixed CSS, GTK3 interface design doesnt touched by theme anymore

### Removed
- **Refrence**: Removed original-puls reference code

## [v0.7.1] - 2026-02-23

### Release
- **Release**: First release of PULS-G3
