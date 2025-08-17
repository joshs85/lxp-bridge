# GitHub Deployment Mechanism

## Overview

This repository uses GitHub Actions to automatically build and deploy Home Assistant add-ons. The deployment system creates two separate add-ons:

1. **LXP Bridge** (`lxp_bridge/`) - Production version
2. **LXP Bridge (dev)** (`lxp_bridge_dev/`) - Development version

## Repository Structure

```
lxp-bridge-fork/
├── .github/
│   ├── workflows/
│   │   ├── build.yaml          # Multi-platform builds and testing
│   │   ├── docker-dev.yaml     # Development Docker builds
│   │   └── release.yaml        # Production releases
│   └── README.md               # This file
├── lxp_bridge/                 # Production add-on configuration
│   ├── config.yaml             # Production add-on config
│   ├── Dockerfile              # Production container
│   └── run.sh                  # Production startup script
├── lxp_bridge_dev/             # Development add-on configuration
│   ├── config.yaml             # Development add-on config
│   ├── Dockerfile              # Development container
│   └── run.sh                  # Development startup script
├── repository.yaml              # Home Assistant add-on repository config
└── ci/
    └── Dockerfile              # CI/CD container definition
```

## Workflow Overview

### 1. Build Workflow (`.github/workflows/build.yaml`)

**Triggers**: 
- Push to `mainline` branch
- Pull requests to `mainline` branch

**Jobs**:
- **Clippy**: Rust code quality analysis with SARIF reporting
- **Multi-platform Build**: Builds for multiple architectures:
  - `linux-amd64` (x86_64)
  - `linux-arm64` (aarch64)
  - `linux-arm-v7` (armv7)
  - `darwin-amd64` (macOS Intel)
  - `darwin-arm64` (macOS Apple Silicon)

**Outputs**: Binary artifacts for each platform

### 2. Development Docker Build (`.github/workflows/docker-dev.yaml`)

**Triggers**: Push to `mainline` branch

**Purpose**: Builds and pushes development Docker images

**Outputs**: 
- `joshs85/lxp-bridge:dev` (multi-platform)
- Platforms: linux/amd64, linux/arm64, linux/arm/v7

### 3. Production Release (`.github/workflows/release.yaml`)

**Triggers**: GitHub release published

**Purpose**: Builds and pushes production Docker images

**Outputs**:
- `joshs85/lxp-bridge:latest` (multi-platform)
- `joshs85/lxp-bridge:{version}` (versioned tag)
- Platforms: linux/amd64, linux/arm64, linux/arm/v7

## Home Assistant Add-on Integration

### Repository Configuration

The `repository.yaml` file configures this repository as a Home Assistant add-on repository:

```yaml
name: lxp-bridge Home Assistant add-on repository
url: https://github.com/joshs85/lxp-bridge
maintainer: Josh Spain <joshs85@users.noreply.github.com>
```

### Add-on Configurations

#### Production Add-on (`lxp_bridge/config.yaml`)
- **Name**: "LXP Bridge"
- **Version**: Semantic versioning (e.g., "v0.16.0")
- **Slug**: `lxp_bridge`
- **Architectures**: aarch64, amd64, armv7

#### Development Add-on (`lxp_bridge_dev/config.yaml`)
- **Name**: "LXP Bridge (dev)"
- **Version**: "dev" (always development)
- **Slug**: `lxp_bridge_dev`
- **Architectures**: aarch64, amd64, armv7

## Deployment Process

### 1. Development Deployment
1. Code is pushed to `mainline` branch
2. GitHub Actions triggers:
   - Build workflow (testing and validation)
   - Docker development build
3. Development Docker image is pushed to Docker Hub
4. Home Assistant add-on repository is updated

### 2. Production Release
1. GitHub release is created and published
2. Release workflow triggers
3. Production Docker image is built and tagged with version
4. Image is pushed to Docker Hub with both `latest` and version tags

### 3. Home Assistant Discovery
1. User adds this repository to Home Assistant:
   ```
   https://github.com/joshs85/lxp-bridge
   ```
2. Both add-ons become available:
   - **LXP Bridge** (production)
   - **LXP Bridge (dev)** (development)
3. Users can install either version based on their needs

## Container Architecture

### Multi-Platform Support
All containers are built for multiple architectures:
- **linux/amd64**: Standard x86_64 servers and desktops
- **linux/arm64**: ARM64 servers (Raspberry Pi 4 64-bit, ARM servers)
- **linux/arm/v7**: ARMv7 devices (Raspberry Pi 3, older ARM devices)

### Container Features
- **Base**: Alpine Linux for minimal size
- **Runtime**: Rust binary with minimal dependencies
- **Configuration**: YAML-based configuration with environment overrides
- **Health Checks**: Built-in health monitoring
- **Logging**: Structured logging with configurable levels

## Configuration Options

Both add-ons support the same configuration schema:

### Inverter Configuration
- Host and port settings
- Serial number configuration
- Data logging options
- Heartbeat monitoring
- Connection behavior

### MQTT Integration
- Broker connection settings
- Authentication
- Topic namespacing
- Home Assistant discovery
- Retry and circuit breaker logic

### Scheduler
- Time synchronization
- Cron-based scheduling
- Automated operations

## Security and Best Practices

### Secrets Management
- Docker Hub credentials stored in GitHub Secrets
- No sensitive data in repository
- Environment-based configuration

### Container Security
- Non-root user execution
- Minimal attack surface
- Regular base image updates
- Multi-stage builds for optimization

### Testing and Validation
- Automated testing on all platforms
- Code quality analysis with Clippy
- Multi-architecture validation
- Integration testing with mock inverters

## Troubleshooting

### Common Issues

1. **Build Failures**: Check GitHub Actions logs for specific error details
2. **Container Issues**: Verify platform compatibility and Docker configuration
3. **Add-on Not Appearing**: Ensure repository URL is correct and accessible
4. **Configuration Errors**: Validate YAML syntax and required fields

### Debug Information
- Enable debug logging in add-on configuration
- Check Home Assistant logs for add-on specific errors
- Verify network connectivity for inverter and MQTT communication

## Contributing

### Development Workflow
1. Fork the repository
2. Create feature branch from `mainline`
3. Make changes and test locally
4. Submit pull request
5. Automated testing and validation
6. Code review and merge

### Testing Requirements
- All tests must pass
- Code must pass Clippy analysis
- Multi-platform builds must succeed
- Integration tests must validate functionality

## Support and Resources

- **Issues**: GitHub Issues for bug reports and feature requests
- **Documentation**: See `doc/` directory for detailed protocol specifications
- **Wiki**: Additional information on GitHub Wiki
- **Community**: Home Assistant community forums and Discord

## Version History

See [CHANGELOG.md](../CHANGELOG.md) for detailed version history and release notes.
