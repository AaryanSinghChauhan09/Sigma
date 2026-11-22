# SigmaOS GitHub Wiki Migration Plan

## Overview

This document outlines the plan to migrate all documentation from the main repository to GitHub Wikis for each component repository.

## Current Documentation Files

### Main Repository
- README.md - Project overview and quick start
- INSTALL.md - Installation guide
- DESKTOP.md - Desktop environment setup
- TOOLS.md - System tools documentation
- BOOTABLE_PLAN.md - Bootable ISO plan
- BOOTSTRAP.md - Bootstrap documentation
- REPOSITORY_RESTRUCTURING_PLAN.md - Repository restructuring plan

### Documentation Directory
- docs/ARCHITECTURE.md - Core library architecture

## Target Wiki Structure

### Main Repository Wiki (Sigma)
- Home.md - Project overview (from README.md)
- Installation.md - Installation guide (from INSTALL.md)
- Desktop.md - Desktop environment setup (from DESKTOP.md)
- Tools.md - System tools documentation (from TOOLS.md)
- Architecture.md - Bootable ISO plan (from BOOTABLE_PLAN.md)
- Bootstrap.md - Bootstrap documentation (from BOOTSTRAP.md)
- Repository-Restructuring.md - Repository restructuring plan (from REPOSITORY_RESTRUCTURING_PLAN.md)
- Core-Library-Architecture.md - Core library architecture (from docs/ARCHITECTURE.md)
- Development.md - Development guide
- Contributing.md - Contributing guidelines
- Testing.md - Testing guide
- Security.md - Security guidelines
- API.md - API reference
- Changelog.md - Changelog
- FAQ.md - Frequently asked questions
- Troubleshooting.md - Troubleshooting guide

## Migration Steps

### Phase 1: Main Repository Wiki
1. Create GitHub Wiki for main repository
2. Migrate all .md files to wiki pages
3. Create additional wiki pages
4. Remove .md files from main repository (keep minimal README.md)

### Phase 2: Component Repository Wikis
1. For each component repository, create its own wiki
2. Create standard wiki pages for each component

## Benefits

1. Cleaner Repository - Main branch free from documentation files
2. Better Organization - Documentation in dedicated wiki
3. Easier Updates - Wiki can be updated without commits
4. Collaboration - Community can contribute to wiki
5. Versioning - Wiki has its own version history
