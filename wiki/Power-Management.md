# SigmaOS Power Management Module

## Overview
The power management module provides power control, battery optimization, and energy efficiency tuning. It is inspired by Linux kernel power management and Arch Linux power saving tools.

## Features
- **Battery Monitoring**: Track battery status, capacity, and time remaining
- **Power Profiles**: Performance, Balanced, and Power-Saver profiles
- **TDP Control**: Configure thermal design power (15W-90W + custom)
- **Auto-Optimization**: Automatically adjust settings based on battery level
- **Configuration Generation**: Generate TLP, power-profiles-daemon, and throttled configs

## Usage

```rust
use sigma_system::{PowerManager, BatteryInfo, BatteryStatus, PowerProfile, TdpLevel};

// Create a power manager
let battery = BatteryInfo::new(75, BatteryStatus::Discharging).unwrap();
let mut mgr = PowerManager::new(PowerProfile::Balanced, TdpLevel::T45)
    .with_battery(battery);

// Auto-optimize based on battery level
mgr.auto_optimize();

// Generate configurations
let tlp_config = mgr.generate_tlp_config();
let ppd_config = mgr.generate_power_profiles_daemon_config();
let throttled_config = mgr.generate_throttled_config();

println!("Status: {}", mgr.get_battery_status_summary());
println!("Estimated life: {:?}", mgr.estimated_battery_life());
```

## Configuration

### BatteryStatus
- `Charging`: Battery is charging
- `Discharging`: Battery is discharging
- `Full`: Battery is full
- `Unknown`: Status unknown

### PowerProfile
- `Performance`: Maximum performance
- `Balanced`: Balanced performance and power
- `PowerSaver`: Maximum power savings

### TdpLevel
- `T15`: 15W TDP
- `T30`: 30W TDP
- `T45`: 45W TDP
- `T60`: 60W TDP
- `T90`: 90W TDP
- `Custom(w)`: Custom TDP in watts

### BatteryInfo
- `capacity`: Battery capacity percentage (0-100)
- `status`: Current battery status
- `energy_full`: Full energy in mWh
- `energy_now`: Current energy in mWh
- `voltage`: Battery voltage in mV

## API Reference

### Methods
- `set_profile()`: Set power profile
- `set_tdp_level()`: Set TDP level
- `enable_wifi_power_save()`: Enable Wi-Fi power saving
- `disable_wifi_power_save()`: Disable Wi-Fi power saving
- `enable_bluetooth()`: Enable Bluetooth
- `disable_bluetooth()`: Disable Bluetooth
- `auto_optimize()`: Automatically optimize based on battery
- `get_battery_status_summary()`: Get battery status string
- `estimated_battery_life()`: Get estimated time remaining

## Examples

### Manual Profile Selection
```rust
// Performance mode for gaming
mgr.set_profile(PowerProfile::Performance);
mgr.set_tdp_level(TdpLevel::T90).unwrap();

// Power saver for office work
mgr.set_profile(PowerProfile::PowerSaver);
mgr.set_tdp_level(TdpLevel::T15).unwrap();
```

### Battery-Based Auto-Optimization
```rust
// Automatically adjusts based on battery level
mgr.auto_optimize();

// If battery < 20%:
// - Profile: PowerSaver
// - Wi-Fi power save: enabled
// - Bluetooth: disabled

// If battery < 50%:
// - Profile: Balanced
// - Wi-Fi power save: enabled
```

### Generating Power Configurations
```rust
// Save TLP configuration
std::fs::write("/etc/tlp.conf", mgr.generate_tlp_config())?;

// Save power-profiles-daemon configuration
std::fs::write("/etc/power-profiles-daemon/config.yaml", 
    mgr.generate_power_profiles_daemon_config())?;

// Save throttled configuration
std::fs::write("/etc/throttled.conf", mgr.generate_throttled_config())?;
```

## Profile-Specific Settings

### Performance Profile
- CPU governor: performance
- TDP: 90W (or custom)
- Wi-Fi power save: disabled
- Bluetooth: enabled

### Balanced Profile
- CPU governor: schedutil
- TDP: 45W
- Wi-Fi power save: conditional
- Bluetooth: enabled

### Power-Saver Profile
- CPU governor: powersave
- TDP: 15W
- Wi-Fi power save: enabled
- Bluetooth: disabled

## Integration with Other Modules
The power module integrates with:
- **Memory module**: Adjusts swappiness based on power profile
- **Monitoring module**: Reports battery metrics
- **Init module**: Power-related service management

## Principles Applied
- Single Responsibility: Only manages power subsystem
- Fail-Fast: Validates TDP and capacity values
- Clear Abstraction: Well-defined public API
- AAA Testing: All tests follow Arrange-Act-Assert pattern
