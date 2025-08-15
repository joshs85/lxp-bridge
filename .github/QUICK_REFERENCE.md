# Quick Reference Guide

## 🚀 Quick Start

### For Developers
```bash
# 1. Make changes and test locally
cargo test
cargo build --release

# 2. Push to mainline (triggers dev build)
git push origin mainline

# 3. Check GitHub Actions for build status
# https://github.com/joshs85/lxp-bridge/actions
```

### For Releases
```bash
# 1. Create and publish GitHub release
# 2. Tag with semantic version (e.g., v0.16.0)
# 3. Production container builds automatically
```

## 📋 Workflow Summary

| Workflow | Trigger | Purpose | Output |
|----------|---------|---------|---------|
| `build.yaml` | Push/PR to mainline | Test & validate | Binary artifacts |
| `docker-dev.yaml` | Push to mainline | Dev container | `joshs85/lxp-bridge:dev` |
| `release.yaml` | GitHub release | Production container | `joshs85/lxp-bridge:latest` + version |

## 🏗️ Build Targets

| Platform | Architecture | Use Case |
|----------|--------------|----------|
| `linux/amd64` | x86_64 | Servers, desktops |
| `linux/arm64` | ARM64 | Raspberry Pi 4 64-bit, ARM servers |
| `linux/arm/v7` | ARMv7 | Raspberry Pi 3, older ARM |

## 🐳 Docker Images

| Image | Tag | Purpose | Stability |
|-------|-----|---------|-----------|
| `joshs85/lxp-bridge` | `dev` | Development testing | Experimental |
| `joshs85/lxp-bridge` | `latest` | Production use | Stable |
| `joshs85/lxp-bridge` | `vX.X.X` | Versioned releases | Stable |

## 🏠 Home Assistant Integration

### Repository URL
```
https://github.com/joshs85/lxp-bridge
```

### Available Add-ons
- **LXP Bridge** - Production version
- **LXP Bridge (dev)** - Development version

## ⚙️ Configuration

### Add-on Configs
- **Production**: `lxp_bridge/config.yaml`
- **Development**: `lxp_bridge_dev/config.yaml`

### Key Differences
```yaml
# Production
name: "LXP Bridge"
version: "v0.16.0"
slug: "lxp_bridge"

# Development  
name: "LXP Bridge (dev)"
version: "dev"
slug: "lxp_bridge_dev"
```

## 🔍 Monitoring

### GitHub Actions
- **Status**: Check Actions tab
- **Logs**: Click on workflow run
- **Artifacts**: Download build outputs

### Docker Hub
- **Images**: https://hub.docker.com/r/joshs85/lxp-bridge
- **Tags**: View all available versions
- **Build History**: Check recent builds

### Home Assistant
- **Add-on Logs**: Available in HA UI
- **Configuration**: Schema validation
- **Status**: Running state monitoring

## 🚨 Common Issues

### Build Fails
```bash
# Check locally first
cargo check
cargo clippy
cargo test

# Verify dependencies
cargo update
```

### Container Issues
```bash
# Test locally
docker build -t test .
docker run -it --rm test

# Check platform support
docker buildx ls
```

### Add-on Not Visible
- Verify repository URL is correct
- Check GitHub repository accessibility
- Ensure `repository.yaml` exists

## 📚 Documentation

### Protocol Specs
- **[MODBUS_PROTOCOL.md](../doc/MODBUS_PROTOCOL.md)** - Complete protocol
- **[HOLD_REGISTERS.md](../doc/HOLD_REGISTERS.md)** - Read/write registers
- **[INPUT_REGISTERS.md](../doc/INPUT_REGISTERS.md)** - Read-only registers
- **[REGISTER_MAPPING.md](../doc/REGISTER_MAPPING.md)** - Quick reference

### Development
- **[README.md](../README.md)** - Project overview
- **[CHANGELOG.md](../CHANGELOG.md)** - Version history
- **[.cursor/rules/](../.cursor/rules/)** - Cursor IDE rules

## 🔧 Development Commands

### Local Development
```bash
# Build
cargo build
cargo build --release

# Test
cargo test
cargo test --release

# Code quality
cargo clippy
cargo fmt

# Run
cargo run
RUST_LOG=debug cargo run
```

### Container Development
```bash
# Build container
docker build -t lxp-bridge:local .

# Run container
docker run -it --rm lxp-bridge:local

# Test multi-platform
docker buildx build --platform linux/amd64,linux/arm64 .
```

### Integration Testing
```bash
# Ruby tests
cd integration_tests
bundle install
bundle exec rspec

# Test specific features
bundle exec rspec spec/inverter_comms_spec.rb
```

## 📝 Release Process

### 1. Prepare Release
```bash
# Update version in Cargo.toml
# Update version in lxp_bridge/config.yaml
# Update CHANGELOG.md
```

### 2. Create Release
- Go to GitHub Releases
- Create new release
- Tag: `vX.X.X` (semantic version)
- Title: `LXP Bridge vX.X.X`
- Description: Copy from CHANGELOG.md

### 3. Publish
- Click "Publish release"
- GitHub Actions builds automatically
- Container pushed to Docker Hub
- Home Assistant add-on updated

## 🆘 Support

### GitHub
- **Issues**: Bug reports, feature requests
- **Discussions**: Questions, help
- **Wiki**: Additional documentation

### Community
- **Home Assistant Forum**: Add-on support
- **Discord**: Real-time help
- **Documentation**: Protocol specs

## 📊 Status Dashboard

### Current Status
- **Build Status**: [![Build](https://github.com/joshs85/lxp-bridge/workflows/Build/badge.svg)](https://github.com/joshs85/lxp-bridge/actions)
- **Docker Hub**: [joshs85/lxp-bridge](https://hub.docker.com/r/joshs85/lxp-bridge)
- **Latest Version**: Check releases page
- **Dev Version**: Always available as `dev` tag
