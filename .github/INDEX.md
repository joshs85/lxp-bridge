# GitHub Deployment Documentation Index

## 📚 Documentation Overview

This directory contains comprehensive documentation for the GitHub Actions deployment system that automatically builds and deploys Home Assistant add-ons.

## 📖 Documentation Files

### 1. [README.md](README.md) - Main Documentation
**Comprehensive guide covering:**
- Repository structure and architecture
- Workflow overview and triggers
- Home Assistant add-on integration
- Container architecture and features
- Security and best practices
- Troubleshooting guide

**Best for:** Understanding the complete system architecture and deployment process

### 2. [DEPLOYMENT_FLOW.md](DEPLOYMENT_FLOW.md) - Process Details
**Detailed deployment process including:**
- Mermaid flow diagrams
- Step-by-step process breakdown
- Container lifecycle management
- Multi-platform build process
- Configuration management
- Monitoring and debugging

**Best for:** Developers who need to understand the detailed deployment flow

### 3. [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Developer Quick Start
**Quick reference guide with:**
- Fast start commands
- Workflow summary tables
- Common issues and solutions
- Development commands
- Release process steps

**Best for:** Developers who need quick access to commands and common solutions

## 🚀 Quick Start Guide

### For New Developers
1. **Start Here**: [README.md](README.md) - Understand the system
2. **Process Details**: [DEPLOYMENT_FLOW.md](DEPLOYMENT_FLOW.md) - Learn the workflow
3. **Quick Reference**: [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Get commands

### For Experienced Developers
1. **Quick Reference**: [QUICK_REFERENCE.md](QUICK_REFERENCE.md) - Commands and tables
2. **Process Details**: [DEPLOYMENT_FLOW.md](DEPLOYMENT_FLOW.md) - Specific workflows
3. **Main Docs**: [README.md](README.md) - Reference and troubleshooting

## 🔄 Deployment Workflows

### Workflow Files
- **`.github/workflows/build.yaml`** - Multi-platform builds and testing
- **`.github/workflows/docker-dev.yaml`** - Development container builds
- **`.github/workflows/release.yaml`** - Production release builds

### Key Triggers
- **Push to `mainline`**: Triggers development builds and testing
- **GitHub Release**: Triggers production container builds
- **Pull Requests**: Triggers validation and testing

## 🏗️ System Architecture

### Components
- **GitHub Actions**: Automated CI/CD pipelines
- **Docker Hub**: Container registry for images
- **Home Assistant**: Add-on discovery and installation
- **Multi-platform**: Support for amd64, arm64, and armv7

### Add-ons
- **LXP Bridge**: Production version with semantic versioning
- **LXP Bridge (dev)**: Development version with latest features

## 📋 Quick Commands

### Development
```bash
# Test locally
cargo test

# Build locally
cargo build --release

# Push to trigger dev build
git push origin mainline
```

### Monitoring
- **GitHub Actions**: Check Actions tab for build status
- **Docker Hub**: View published images and tags
- **Home Assistant**: Monitor add-on status and logs

## 🆘 Getting Help

### Documentation Issues
- Check all three documentation files
- Look for specific sections in the main README
- Review troubleshooting guides

### Technical Issues
- Check GitHub Actions logs
- Verify Docker Hub credentials
- Test locally before pushing

### Community Support
- GitHub Issues for bug reports
- Home Assistant community forums
- Project discussions and wiki

## 📊 Status and Monitoring

### Build Status
- **Development**: Automatic on every push to mainline
- **Production**: Automatic on GitHub release publish
- **Multi-platform**: All architectures built simultaneously

### Container Status
- **Dev Tag**: `joshs85/lxp-bridge:dev` (latest development)
- **Latest Tag**: `joshs85/lxp-bridge:latest` (stable production)
- **Version Tags**: `joshs85/lxp-bridge:vX.X.X` (specific versions)

## 🔗 Related Documentation

### Project Documentation
- **[../README.md](../README.md)** - Main project overview
- **[../CHANGELOG.md](../CHANGELOG.md)** - Version history
- **[../doc/](../doc/)** - Protocol specifications and technical docs

### Development Tools
- **[../.cursor/rules/](../.cursor/rules/)** - Cursor IDE rules and guidelines
- **[../ci/](../ci/)** - CI/CD configuration and Dockerfiles
- **[../tests/](../tests/)** - Unit and integration test suites

## 📝 Contributing

### Documentation Updates
- Keep all three files in sync
- Update examples and commands as needed
- Add new troubleshooting scenarios
- Maintain accuracy of workflow descriptions

### Process Improvements
- Document new workflows or changes
- Update deployment procedures
- Add new monitoring or debugging tools
- Improve troubleshooting guides

---

**Last Updated**: See git history for latest changes
**Maintained By**: Development team
**For Issues**: Create GitHub issue or discussion
