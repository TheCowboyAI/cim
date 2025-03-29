# CIM Core Architecture Components

## Metadata
- Category: facts
- Date: 2024-03-29
- Status: verified
- Related: 
  - theories/architecture_patterns.md
  - models/system_architecture.md
  - claims/architecture_benefits.md
- Source: cim/readme.md, cim-cursor/README.md

## Content

CIM's core architecture consists of the following verified components:

1. Event Store System
   - Uses Git as the primary event store
   - Maintains immutable history of all changes
   - Provides versioning and branching capabilities
   - Enables event sourcing patterns

2. Object Store System
   - Content-addressed storage (CAS)
   - Two primary buckets:
     - cim-objects: For immutable data
     - cim-events: For event history
   - Version tracking through directed graphs
   - Content verification mechanisms

3. Message Transport
   - NATS for internal communication
   - Matrix for external messaging
   - Subject-based message routing
   - Event-driven state updates

4. Security Infrastructure
   - mTLS authentication
   - YubiKey integration
   - OpenPGP support
   - OpenSSL encryption

5. Container System
   - NixOS containers (systemd-nspawn)
   - NVIDIA GPU sharing support
   - Wayland + Hyprland integration
   - S3 compatible storage (Minio/Wasabi)

## References
- [CIM Repository](../cim/readme.md)
- [CIM Cursor Documentation](../cim-cursor/README.md)
- [Architecture Implementation](../cim-start/readme.md)

## Updates
- 2024-03-29: Initial documentation of core architecture components
- 2024-03-29: Added verification of container system components 