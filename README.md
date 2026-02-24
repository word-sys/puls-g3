<img src="https://raw.githubusercontent.com/word-sys/puls-g3/refs/heads/main/puls-g3.svg" width="256" height="256" alt="PULS-G3 Icon"/>

# PULS-G3

**A unified system monitoring and management tool for Linux on GTK3**

PULS-G3 is the GTK3 graphical edition of PULS. It combines resource monitoring with system administration capabilities, allowing control over system services, boot configurations, and logs directly from a modern desktop interface — everything in one place.

![PULS Screenshot](https://raw.githubusercontent.com/word-sys/puls-g3/main/screenshots/screenshot.png)

## Architecture

PULS-G3 is built with Rust, using GTK3 for the interface and leverages native Linux APIs and binaries for system interaction:
*   **Interface**: GTK3 with a dark terminal-inspired theme, tabbed layout via StackSwitcher, and keyboard shortcuts for power users.
*   **Monitoring**: Uses `sysinfo` for host metrics, `nvidia-smi` for NVIDIA GPUs, and a native DRM parser for AMD/Intel GPU telemetry. Supports multi-GPU configurations.
*   **System Control**: Interfaces directly with `systemd` (via `systemctl`) and `journald` (via `journalctl`) for service and log management. Uses `pkexec` for privilege escalation.
*   **Process Management**: Advanced sorting logic including a "General" resource usage score combining CPU and Memory usage. Double-click any process to inspect detailed `/proc` data.
*   **Configuration**: Parses and modifies `/etc/default/grub` and other system files with backup generation.

## Features

### 1. Resource Monitoring
*   **CPU & Memory**: Per-core visualization and memory page breakdown with progress bars.
*   **Disk I/O**: Full disk table with mount point, filesystem, usage, temperature, health, and power cycles.
*   **Network**: Real-time upload/download rates for all interfaces in a sortable table.
*   **NVIDIA, AMD & Intel GPUs**: Multi-vendor support with utilization, VRAM usage, temperature, and power telemetry.
*   **Hardware Sensors**: Temperature, fan speed, voltage, and power readings from all system sensors.

### 2. Process & Container Architecture
*   **Process Tree**: Sortable process list exposing PID, user, CPU, memory, disk I/O, and status.
*   **Process Details**: Double-click any process to view full details — command line, environment variables, working directory, thread count, file descriptors, parent PID, and start time — all read directly from `/proc`.
*   **Container Engine Integration**: Connects to the local Docker socket to monitor container lifecycles, resource usage (CPU/Mem), and image info.

### 3. Service Management Subsystem
PULS-G3 provides control over `systemd` units with `pkexec` privilege escalation:
*   **State Control**: Start, Stop, Restart services.
*   **Boot Persistence**: Enable or Disable services at startup.
*   **Status Feedback**: Visual feedback on action success/failure.

### 4. Journal & Logging
*   **Aggregated Logs**: View `journald` logs directly within the GUI with auto-refresh.
*   **Filtering**: Filter logs by specific system services, priority levels (Error/Warning), or search text.

### 5. Boot Configuration (GRUB)
*   **Parameter Editing**: Modify kernel parameters in `/etc/default/grub`.
*   **Safety Backup**: PULS-G3 automatically creates a timestamped backup (e.g., `/etc/default/grub.bak.<timestamp>`) before applying any changes to boot configurations.

### 6. Keyboard Shortcuts
| Key | Action |
| :--- | :--- |
| `1`-`9`, `0`, `-`, `=`, `+` | Switch tabs |
| `Tab` | Cycle to next tab |
| `P` | Process Details tab |

## Installation

### Build from Source

1.  **Dependencies**:
    ```bash
    # Ubuntu/Debian
    sudo apt install libgtk-3-dev pkg-config build-essential

    # Arch Linux
    sudo pacman -S gtk3 pkg-config base-devel

    # Fedora
    sudo dnf install gtk3-devel pkg-config
    ```

2.  **Build**:
    ```bash
    cargo build --release
    ```

### Build .deb Package

1.  **Install cargo-deb**:
    ```bash
    cargo install cargo-deb
    ```

2.  **Build**:
    ```bash
    cargo deb
    ```
    The `.deb` file will be created in `target/debian/`.

### Release Binary
```bash
bash build-release.sh
sudo cp target/x86_64-unknown-linux-gnu/release/puls-g3 /usr/local/bin/
```
The release binary requires `libgtk-3-0` on the target system.

## Usage

PULS-G3 operates in different modes depending on the privileges:

| Command | Capabilities |
| :--- | :--- |
| `puls-g3` | **Monitoring**: CPU, GPU, memory, disk, network, processes, containers. Service actions use `pkexec` for privilege escalation on demand. |
| `sudo puls-g3` | **Full Access**: All monitoring plus direct `systemctl`, journal, and GRUB editing without prompts. |
| `puls-g3 --safe` | **Safety Mode**: Explicitly disables write capability, preventing accidental edits. |

---

*For release notes and updates, please visit the [GitHub Releases](https://github.com/word-sys/puls-g3/releases) page.*
*Verified on Arch Linux and Ubuntu 22.04+. Compatible with any Linux distribution with GTK3 >= 3.22.*
