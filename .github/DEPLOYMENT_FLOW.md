# Deployment Flow and Process

## Deployment Flow Diagram

```mermaid
graph TD
    A[Developer Push to mainline] --> B[GitHub Actions Trigger]
    
    B --> C[Build Workflow]
    B --> D[Docker Dev Build]
    
    C --> C1[Clippy Analysis]
    C --> C2[Multi-platform Build]
    C --> C3[Test Validation]
    
    D --> D1[Build Dev Container]
    D --> D2[Push to Docker Hub]
    D --> D3[Tag: joshs85/lxp-bridge:dev]
    
    E[GitHub Release] --> F[Release Workflow]
    F --> F1[Build Production Container]
    F --> F2[Push to Docker Hub]
    F --> F3[Tag: joshs85/lxp-bridge:latest]
    F --> F4[Tag: joshs85/lxp-bridge:vX.X.X]
    
    G[User Adds Repository] --> H[Home Assistant Discovery]
    H --> I[Both Add-ons Available]
    I --> I1[LXP Bridge - Production]
    I --> I2[LXP Bridge (dev) - Development]
```

## Detailed Process Steps

### 1. Development Workflow

#### Trigger: Push to `mainline` branch
```bash
git push origin mainline
```

#### Automated Actions:
1. **Build Workflow** (`.github/workflows/build.yaml`)
   - Runs Clippy code analysis
   - Builds for all target platforms
   - Runs unit tests
   - Validates multi-architecture compatibility

2. **Development Docker Build** (`.github/workflows/docker-dev.yaml`)
   - Builds development container
   - Multi-platform build (amd64, arm64, armv7)
   - Pushes to Docker Hub as `joshs85/lxp-bridge:dev`

### 2. Production Release Workflow

#### Trigger: GitHub Release Published
1. Create new release in GitHub
2. Tag with semantic version (e.g., `v0.16.0`)
3. Publish release

#### Automated Actions:
1. **Release Workflow** (`.github/workflows/release.yaml`)
   - Builds production container
   - Multi-platform build
   - Pushes to Docker Hub:
     - `joshs85/lxp-bridge:latest`
     - `joshs85/lxp-bridge:v0.16.0`

### 3. Home Assistant Integration

#### Repository Addition
Users add this repository to Home Assistant:
```yaml
# In Home Assistant > Settings > Add-ons > Add-on Store
# Click the three dots menu and select "Repositories"
# Add: https://github.com/joshs85/lxp-bridge
```

#### Add-on Discovery
Once added, both add-ons become available:

**Production Add-on:**
- **Name**: LXP Bridge
- **Version**: Latest stable release (e.g., v0.16.0)
- **Slug**: `lxp_bridge`
- **Description**: Production-ready inverter communication

**Development Add-on:**
- **Name**: LXP Bridge (dev)
- **Version**: Always "dev"
- **Slug**: `lxp_bridge_dev`
- **Description**: Latest development features

## Container Lifecycle

### Development Container
- **Source**: `mainline` branch
- **Build**: Every push to mainline
- **Tag**: `joshs85/lxp-bridge:dev`
- **Purpose**: Testing new features
- **Stability**: May contain experimental code

### Production Container
- **Source**: GitHub releases
- **Build**: Only on release publish
- **Tags**: `joshs85/lxp-bridge:latest` + version tags
- **Purpose**: Stable production use
- **Stability**: Tested and validated

## Multi-Platform Support

### Target Architectures
```yaml
platforms:
  - linux/amd64    # x86_64 servers, desktops
  - linux/arm64    # ARM64 servers, Raspberry Pi 4 64-bit
  - linux/arm/v7   # ARMv7 devices, Raspberry Pi 3
```

### Build Process
1. **QEMU Setup**: Enables cross-platform builds
2. **Docker Buildx**: Multi-platform container builds
3. **Platform-Specific**: Optimized for each architecture
4. **Single Manifest**: One image tag, multiple architectures

## Configuration Management

### Add-on Configuration Files
Both add-ons use identical configuration schemas:

```yaml
# lxp_bridge/config.yaml (Production)
name: "LXP Bridge"
version: "v0.16.0"
slug: "lxp_bridge"

# lxp_bridge_dev/config.yaml (Development)
name: "LXP Bridge (dev)"
version: "dev"
slug: "lxp_bridge_dev"
```

### User Configuration
Users configure the add-on through Home Assistant UI:
- Inverter settings
- MQTT configuration
- Logging levels
- Scheduler options

## Monitoring and Debugging

### GitHub Actions Monitoring
- **Workflow Status**: Check Actions tab in GitHub
- **Build Logs**: Detailed logs for each step
- **Artifact Downloads**: Binary artifacts for testing

### Container Monitoring
- **Docker Hub**: Image tags and sizes
- **Health Checks**: Built-in container health monitoring
- **Logs**: Structured logging with configurable levels

### Home Assistant Integration
- **Add-on Logs**: Available in Home Assistant UI
- **Configuration Validation**: Schema-based validation
- **Status Monitoring**: Running state and health

## Troubleshooting Guide

### Common Deployment Issues

#### 1. Build Failures
**Symptoms**: GitHub Actions failing
**Solutions**:
- Check Rust compilation errors
- Verify dependency versions
- Review platform-specific build issues

#### 2. Container Push Failures
**Symptoms**: Docker Hub push errors
**Solutions**:
- Verify Docker Hub credentials in GitHub Secrets
- Check Docker Hub rate limits
- Ensure repository permissions

#### 3. Add-on Not Appearing
**Symptoms**: Add-ons not visible in Home Assistant
**Solutions**:
- Verify repository URL is correct
- Check GitHub repository accessibility
- Ensure `repository.yaml` is properly formatted

#### 4. Configuration Errors
**Symptoms**: Add-on fails to start
**Solutions**:
- Validate YAML configuration syntax
- Check required field values
- Review Home Assistant logs

### Debug Commands

#### Local Testing
```bash
# Build locally
cargo build --release

# Test locally
cargo test

# Run with debug logging
RUST_LOG=debug ./target/release/lxp-bridge
```

#### Container Testing
```bash
# Build container locally
docker build -t lxp-bridge:test .

# Run container locally
docker run -it --rm lxp-bridge:test

# Check container logs
docker logs <container_id>
```

## Best Practices

### Development
1. **Branch Strategy**: Use feature branches, merge to mainline
2. **Testing**: Run tests locally before pushing
3. **Code Quality**: Ensure Clippy passes
4. **Documentation**: Update docs with new features

### Deployment
1. **Version Management**: Use semantic versioning
2. **Release Notes**: Document changes in releases
3. **Rollback Strategy**: Keep previous versions available
4. **Monitoring**: Watch for deployment issues

### Maintenance
1. **Regular Updates**: Keep dependencies current
2. **Security Scanning**: Monitor for vulnerabilities
3. **Performance Monitoring**: Track resource usage
4. **User Feedback**: Monitor issues and feature requests
