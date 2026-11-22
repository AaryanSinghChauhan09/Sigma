# SigmaOS Security Management Module

## Overview
The security management module provides secure boot verification, TPM 2.0 integration, and integrity checks. It is inspired by Linux kernel integrity subsystem and Arch Linux security practices.

## Features
- **Secure Boot**: Configure and verify secure boot state
- **TPM Integration**: Support for TPM 1.2 and TPM 2.0 with PCR banks
- **Integrity Checks**: Verify system component integrity
- **Security Scoring**: Calculate overall security score (0-100)
- **Key Management**: Manage secure boot keys

## Usage

```rust
use sigma_system::{SecurityManager, SecureBootState, TpmVersion, TpmConfig, SecureBootKey};

// Create a security manager with TPM 2.0
let tpm_config = TpmConfig::new(TpmVersion::Tpm20)
    .with_pcr_bank("sha256")
    .enable_attestation();
let mut mgr = SecurityManager::new(SecureBootState::Enabled, tpm_config);

// Add secure boot keys
let key = SecureBootKey::new("sigmaos-key", "rsa")
    .with_owner("SigmaOS")
    .with_fingerprint("ABCD1234");
mgr.add_key(key).unwrap();

// Add integrity checks
mgr.add_integrity_check("kernel", sigma_system::IntegrityCheck::Passed);
mgr.add_integrity_check("initramfs", sigma_system::IntegrityCheck::Passed);

// Generate configurations
let efi_config = mgr.generate_efi_vars_config();
let tpm_config = mgr.generate_tpm_config();
let report = mgr.generate_integrity_report();

println!("Security Score: {}", mgr.security_score());
println!("Recommendation: {}", mgr.get_security_recommendation());
```

## Configuration

### SecureBootState
- `Enabled`: Secure boot is enabled
- `Disabled`: Secure boot is disabled
- `Unknown`: Secure boot state unknown

### TpmConfig
- `version`: TPM version (None, 1.2, 2.0)
- `enabled`: TPM enabled
- `pcr_banks`: PCR banks to use (sha1, sha256, etc.)
- `attestation_enabled`: Enable remote attestation

### SecureBootKey
- `key_id`: Key identifier
- `key_type`: Key type (rsa, ecdsa, etc.)
- `owner`: Key owner
- `fingerprint`: Key fingerprint

## API Reference

### IntegrityCheck
- `Passed`: Check passed
- `Failed`: Check failed
- `Skipped`: Check skipped

### Methods
- `add_key()`: Add a secure boot key
- `add_integrity_check()`: Add an integrity check result
- `security_score()`: Calculate overall security score
- `all_integrity_checks_passed()`: Check if all checks passed
- `get_security_recommendation()`: Get security recommendations

## Examples

### Setting Up TPM 2.0
```rust
let tpm_config = TpmConfig::new(TpmVersion::Tpm20)
    .with_pcr_bank("sha256")
    .with_pcr_bank("sha1")
    .enable_attestation();
```

### Managing Keys
```rust
let platform_key = SecureBootKey::new("PK", "rsa")
    .with_owner("Platform")
    .with_fingerprint("PK1234");

let key_exchange_key = SecureBootKey::new("KEK", "rsa")
    .with_owner("Platform")
    .with_fingerprint("KEK5678");

let db_key = SecureBootKey::new("db", "rsa")
    .with_owner("SigmaOS")
    .with_fingerprint("DB9012");
```

### Security Score Calculation
The security score is calculated as:
- Secure Boot enabled: +30 points
- TPM 2.0 enabled: +40 points
- TPM 1.2 enabled: +20 points
- Keys registered: +10 points
- All integrity checks passed: +20 points

## Integration with Other Modules
The security module integrates with:
- **Boot module**: Secure boot verification during boot
- **Filesystem module**: Disk encryption (LUKS)
- **Network module**: Firewall rules for security
- **Monitoring module**: Security event monitoring

## Principles Applied
- Single Responsibility: Only manages security subsystem
- Fail-Fast: Validates keys before adding
- Clear Abstraction: Well-defined public API
- AAA Testing: All tests follow Arrange-Act-Assert pattern
