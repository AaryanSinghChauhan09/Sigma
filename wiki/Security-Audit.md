# SigmaOS Security Audit Module

## Overview
Vulnerability scanning and compliance checking inspired by CIS benchmarks and Lynis.

## Features
- Vulnerability tracking with CVSS scores
- CIS benchmark rules
- Lynis audit rules
- Compliance scoring (0-100%)
- Remediation script generation

## Usage

```rust
use sigma_system::{AuditManager, Vulnerability, VulnerabilitySeverity};

let mut mgr = AuditManager::new();
mgr.add_vulnerability(Vulnerability::new("CVE-2024-0001", "openssl", VulnerabilitySeverity::High));
let score = mgr.compliance_score();
let script = mgr.generate_remediation_script();
```
