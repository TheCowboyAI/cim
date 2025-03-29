### Term: NixOS
- **Category**: Technical Concept
- **Type**: Platform
- **Taxonomy**: Infrastructure
- **Definition**: A Linux distribution that uses declarative configuration and pure functional package management for reproducible system builds.
- **Relationships**:
  * Uses: Nix Language
  * Provides: Package Management
  * Implements: System Configuration
- **Usage Context**: System deployment and configuration
- **Code Reference**: `cim/src/infrastructure/nixos`

### Term: wasmCloud
- **Category**: Technical Concept
- **Type**: Platform
- **Taxonomy**: Runtime
- **Definition**: A distributed WebAssembly runtime that enables secure, portable, and composable applications through actors and capability providers.
- **Relationships**:
  * Uses: WebAssembly
  * Contains: Actors, Providers
  * Implements: Distributed Computing
- **Usage Context**: Application runtime and deployment
- **Code Reference**: `cim/src/infrastructure/wasmcloud`

### Term: Ollama
- **Category**: Technical Concept
- **Type**: Tool
- **Taxonomy**: AI Infrastructure
- **Definition**: A framework for running and managing large language models locally with standardized interfaces.
- **Relationships**:
  * Provides: LLM Runtime
  * Used-By: AI Agents
  * Implements: Model Inference
- **Usage Context**: Local AI model deployment
- **Code Reference**: `cim/src/ai/ollama`

### Term: Leptos
- **Category**: Technical Concept
- **Type**: Framework
- **Taxonomy**: Frontend
- **Definition**: A Rust-based frontend framework for building reactive web applications with fine-grained reactivity.
- **Relationships**:
  * Uses: WebAssembly
  * Provides: UI Components
  * Implements: Reactive UI
- **Usage Context**: Web interface development
- **Code Reference**: `cim/src/ui/leptos`

### Term: Matrix
- **Category**: Technical Concept
- **Type**: Protocol
- **Taxonomy**: Communication
- **Definition**: An open protocol for secure, decentralized real-time communication with end-to-end encryption.
- **Relationships**:
  * Provides: Messaging
  * Implements: E2E Encryption
  * Used-By: Communication System
- **Usage Context**: Secure communication infrastructure
- **Code Reference**: `cim/src/comms/matrix`

### Term: Paperless-NGX
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Document Management
- **Definition**: A document management system that processes, indexes, and archives digital documents.
- **Relationships**:
  * Manages: Documents
  * Uses: OCR
  * Implements: Document Processing
- **Usage Context**: Document management and archival
- **Code Reference**: `cim/src/docs/paperless`

### Term: FreshRSS
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Feed Management
- **Definition**: A feed aggregator and reader that collects and organizes RSS/Atom feeds.
- **Relationships**:
  * Manages: Feeds
  * Implements: Feed Aggregation
  * Provides: Content Organization
- **Usage Context**: Information aggregation and monitoring
- **Code Reference**: `cim/src/feeds/fresh`

### Term: VaultWarden
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Security
- **Definition**: A password management system that provides secure storage and sharing of credentials and secrets.
- **Relationships**:
  * Manages: Secrets
  * Implements: Password Management
  * Uses: Encryption
- **Usage Context**: Credential and secret management
- **Code Reference**: `cim/src/security/vault`

### Term: SearxNG
- **Category**: Technical Concept
- **Type**: Service
- **Taxonomy**: Search
- **Definition**: A privacy-respecting, self-hosted metasearch engine that aggregates results from multiple search engines.
- **Relationships**:
  * Provides: Search Aggregation
  * Implements: Privacy Protection
  * Used-By: Search System
- **Usage Context**: Privacy-focused search functionality
- **Code Reference**: `cim/src/search/searx` 