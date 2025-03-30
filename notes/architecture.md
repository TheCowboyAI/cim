# CIM Architecture Notes

[← Back to Index](index.md) | [→ Implementation Guide](implementation.md)

*For terminology, see [Vocabulary and Terms](vocabulary.md)*

## Core Architecture
*Implementation details can be found in [Technical Infrastructure](technical.md)*

### Foundation
- [Domain-Driven Design (DDD)](vocabulary.md#technical-terms) principles
- [Event-Driven Architecture (EDA)](vocabulary.md#technical-terms) - [Implementation Decision](decisions/event-driven-architecture-implementation.md)
- Bottom-up approach starting from single packages
- Modular construction methodology
- Minimal environment requirements

### Infrastructure Components
*Security details in [Security Model](security.md)*

1. Container System
   - NixOS containers (systemd-nspawn)
   - NVIDIA GPU sharing capabilities
   - Wayland + Hyprland integration
   - S3 compatible storage (Minio/Wasabi)

2. Security Layer
   - [mTLS](vocabulary.md#security) authentication
   - [YubiKey](vocabulary.md#security) integration
   - [OpenPGP](vocabulary.md#security) support
   - [OpenSSL](vocabulary.md#security) encryption

### State Management
*For implementation examples, see [Implementation Guide](implementation.md)*

1. Object Storage
   - [Content-addressed](vocabulary.md#technical-terms) storage system
   - Two primary buckets:
     - cim-objects: Immutable data storage
     - cim-events: Event history storage
   - Version tracking through graphs
   - Content verification system

2. Event System
   - State change tracking via events
   - Delta storage in Object Store
   - Immutable history maintenance
   - Graph-based relationship tracking
   - Implementation via `EventBus`, `EventDispatcher`, and `EventHandler` pattern
   - Full decoupling of components through event-based messaging
   - Comprehensive event types for all domain operations
   - Centralized event publishing and handling

## Client-Server Architecture
*See detailed [NATS-Based MCP Client Implementation Decision](decisions/2024-05-06-nats-based-mcp-client.md)*

### Server Components
- CIM Ontology Tool implements MCP server over NATS
- Event-driven architecture for request processing
- Neo4j graph database for ontology storage
- Subject-based message routing

### Client Libraries
1. Core Library (cim-client)
   - NATS-based communication
   - Domain entity models
   - Asynchronous API
   - Request/response handling
   - Event subscription

2. Interface Options
   - Programmable API for embedded usage
   - CLI for command-line operations
   - GUI for visual management and exploration
   - All built on the same core library

3. Design Principles
   - Domain-Driven Design architecture
   - Clean separation of concerns
   - Consistent error handling
   - Feature flags for optional components
   - Full async support via Tokio

## Pod Architecture
*For specific pod implementations, see [Domain Implementations](domain_implementations.md)*

### Core Pods
1. AI Pod (pod.ai.cim)
   - Local LLM integration via Ollama
   - Model management system
   - Chat functionality
   - Embedding generation
   - Object storage integration
   - Settings management
   - [NATS](vocabulary.md#technical-terms) event integration

2. Communications Pod (pod.comms.cim)
   - Matrix external messaging
   - [NATS](vocabulary.md#technical-terms) internal messaging
   - Platform bridges (Telegram, WhatsApp, Discord, IRC)
   - E2E encryption
   - User/Profile management
   - Federation support

### Service Pods
*For detailed service documentation, see [Implementation Guide](implementation.md)*
- APIs (pod.apis.cim)
- Database (pod.db.cim)
- Documentation (pod.docs.cim)
- Feed Processing (pod.feeds.cim)
- Git Integration (pod.git.cim)
- Email Services (pod.mail.cim)
- Namespace Services (pod.ns.cim)
- Note Management (pod.notes.cim)
- Policy Enforcement (pod.policy.cim)
- Search Services (pod.search.cim)
- Secure Storage (pod.vault.cim)
- Web Interface (pod.web.cim)
- Workflow Management (pod.wf.cim)

## Implementation Patterns
*For specific examples, see [Implementation Guide](implementation.md)*

### Package System
1. Build Functionality
   - Cargo.toml integration
   - Extensible build sequence
   - CI/CD pipeline support
   - Build status event emission

2. Runtime Capabilities
   - Package execution
   - Service management
   - Module integration
   - Event handling
   - Container orchestration
   - Web interface support
   - [NATS](vocabulary.md#technical-terms) integration
   - Monitoring system

### NixOS-Style Modules
*See detailed [Cursor Rules Modularization Decision](decisions/cursor-rules-modularization.md)*

1. Module Approach
   - Declarative configuration using NixOS module patterns
   - Type-safe option definition
   - Component encapsulation
   - Standardized interfaces

2. Cursor Rules Module
   - Structured rule definition
   - Version-controlled rule content
   - Management tooling
   - Reproducible rule deployment
   - Integration with development workflow

### Communication Patterns
*See also [Technical Infrastructure](technical.md)*

1. Message Flow
   - Subject-based routing
   - Event-driven communication (implemented via `EventBus`)
   - Command processing
   - State observation

2. Integration Points
   - External service connections
   - Inter-pod communication
   - Client-server interactions
   - Federation protocols

### Event-Driven Architecture Implementation
*See detailed [Implementation Decision](decisions/event-driven-architecture-implementation.md)*

1. Core Components
   - `Event`: Data structure with metadata, payload, and context
   - `EventBus`: Message broker for event distribution
   - `EventDispatcher`: Routes events to appropriate handlers
   - `EventHandler`: Processes specific event types

2. Event Flow
   - API requests converted to events
   - Events published to the event bus
   - Handlers subscribe to relevant event types
   - Asynchronous processing of events

3. Benefits
   - Loose coupling between components
   - Enhanced scalability
   - Improved system resilience
   - Comprehensive audit trail
   - Better extensibility

## Deployment Models
*For specific implementations, see [Domain Implementations](domain_implementations.md)*

### Local Deployment
- Single machine setup
- Development environment
- Testing configuration
- Production staging

### Distributed Deployment
- Multi-node architecture
- Service distribution
- Load balancing
- Failover support

### Hybrid Setup
- Mixed deployment models
- Cloud integration
- Edge computing support
- Resource optimization

---
*For contribution guidelines, see [Contributing](index.md#contributing)* 