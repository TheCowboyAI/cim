# Infrastructure Domain Vocabulary

## Domain Objects

### Nodes

```cypher
(:NixOS:Platform {
  domain: "Infrastructure",
  term: "NixOS",
  definition: "A Linux distribution that uses declarative configuration and pure functional package management for reproducible system builds.",
  taxonomy: "Infrastructure",
  usage_context: "System deployment and configuration",
  code_reference: "cim/src/infrastructure/nixos"
})

(:WebAssemblyRuntime:Platform {
  domain: "Infrastructure",
  term: "WebAssemblyRuntime",
  definition: "A distributed WebAssembly runtime that enables secure, portable, and composable applications through components and capability providers.",
  taxonomy: "Runtime",
  usage_context: "Application runtime and deployment",
  code_reference: "cim/src/infrastructure/wasm_runtime"
})

(:Ollama:Tool {
  domain: "Infrastructure",
  term: "Ollama",
  definition: "A framework for running and managing large language models locally with standardized interfaces.",
  taxonomy: "AI Infrastructure",
  usage_context: "Local AI model deployment",
  code_reference: "cim/src/ai/ollama"
})

(:Leptos:Framework {
  domain: "Infrastructure",
  term: "Leptos",
  definition: "A Rust-based frontend framework for building reactive web applications with fine-grained reactivity.",
  taxonomy: "Frontend",
  usage_context: "Web interface development",
  code_reference: "cim/src/ui/leptos"
})

(:Matrix:Protocol {
  domain: "Infrastructure",
  term: "Matrix",
  definition: "An open protocol for secure, decentralized real-time communication with end-to-end encryption.",
  taxonomy: "Communication",
  usage_context: "Secure communication infrastructure",
  code_reference: "cim/src/comms/matrix"
})

(:PaperlessNGX:Service {
  domain: "Infrastructure",
  term: "PaperlessNGX",
  definition: "A document management system that processes, indexes, and archives digital documents.",
  taxonomy: "Document Management",
  usage_context: "Document management and archival",
  code_reference: "cim/src/docs/paperless"
})

(:FreshRSS:Service {
  domain: "Infrastructure",
  term: "FreshRSS",
  definition: "A feed aggregator and reader that collects and organizes RSS/Atom feeds.",
  taxonomy: "Feed Management",
  usage_context: "Information aggregation and monitoring",
  code_reference: "cim/src/feeds/fresh"
})

(:VaultWarden:Service {
  domain: "Infrastructure",
  term: "VaultWarden",
  definition: "A password management system that provides secure storage and sharing of credentials and secrets.",
  taxonomy: "Security",
  usage_context: "Credential and secret management",
  code_reference: "cim/src/security/vault"
})

(:SearxNG:Service {
  domain: "Infrastructure",
  term: "SearxNG",
  definition: "A privacy-respecting, self-hosted metasearch engine that aggregates results from multiple search engines.",
  taxonomy: "Search",
  usage_context: "Privacy-focused search functionality",
  code_reference: "cim/src/search/searx"
})

(:CorePlatform:Platform {
  domain: "Infrastructure",
  term: "CorePlatform",
  definition: "The foundational platform components (NixOS and WebAssembly Runtime) that provide the base infrastructure for the system.",
  taxonomy: "Infrastructure",
  usage_context: "Core system infrastructure",
  code_reference: "cim/src/infrastructure/core"
})

(:DevelopmentTools:Tool {
  domain: "Infrastructure",
  term: "DevelopmentTools",
  definition: "A collection of tools and utilities used for system development, testing, and deployment.",
  taxonomy: "Development",
  usage_context: "Development and testing",
  code_reference: "cim/src/infrastructure/tools"
})

(:SecurityInfrastructure:Service {
  domain: "Infrastructure",
  term: "SecurityInfrastructure",
  definition: "The security components and services that provide authentication, authorization, and data protection.",
  taxonomy: "Security",
  usage_context: "System security management",
  code_reference: "cim/src/infrastructure/security"
})
```

## Relationships

```cypher
// NixOS relationships
(:NixOS)-[:USES {type: "language"}]->(:NixLanguage)
(:NixOS)-[:PROVIDES {type: "management"}]->(:PackageManagement)
(:NixOS)-[:IMPLEMENTS {type: "configuration"}]->(:SystemConfiguration)

// WebAssemblyRuntime relationships
(:WebAssemblyRuntime)-[:USES {type: "technology"}]->(:WebAssembly)
(:WebAssemblyRuntime)-[:CONTAINS {type: "runtime"}]->(:Components)
(:WebAssemblyRuntime)-[:CONTAINS {type: "runtime"}]->(:Providers)
(:WebAssemblyRuntime)-[:IMPLEMENTS {type: "computing"}]->(:DistributedComputing)

// Ollama relationships
(:Ollama)-[:PROVIDES {type: "runtime"}]->(:LLMRuntime)
(:Ollama)-[:USED_BY {type: "ai"}]->(:AIAgents)
(:Ollama)-[:IMPLEMENTS {type: "inference"}]->(:ModelInference)

// Leptos relationships
(:Leptos)-[:USES {type: "technology"}]->(:WebAssembly)
(:Leptos)-[:PROVIDES {type: "ui"}]->(:UIComponents)
(:Leptos)-[:IMPLEMENTS {type: "ui"}]->(:ReactiveUI)

// Matrix relationships
(:Matrix)-[:PROVIDES {type: "communication"}]->(:Messaging)
(:Matrix)-[:IMPLEMENTS {type: "security"}]->(:E2EEncryption)
(:Matrix)-[:USED_BY {type: "system"}]->(:CommunicationSystem)

// PaperlessNGX relationships
(:PaperlessNGX)-[:MANAGES {type: "content"}]->(:Documents)
(:PaperlessNGX)-[:USES {type: "technology"}]->(:OCR)
(:PaperlessNGX)-[:IMPLEMENTS {type: "processing"}]->(:DocumentProcessing)

// FreshRSS relationships
(:FreshRSS)-[:MANAGES {type: "content"}]->(:Feeds)
(:FreshRSS)-[:IMPLEMENTS {type: "aggregation"}]->(:FeedAggregation)
(:FreshRSS)-[:PROVIDES {type: "organization"}]->(:ContentOrganization)

// VaultWarden relationships
(:VaultWarden)-[:MANAGES {type: "security"}]->(:Secrets)
(:VaultWarden)-[:IMPLEMENTS {type: "management"}]->(:PasswordManagement)
(:VaultWarden)-[:USES {type: "security"}]->(:Encryption)

// SearxNG relationships
(:SearxNG)-[:PROVIDES {type: "search"}]->(:SearchAggregation)
(:SearxNG)-[:IMPLEMENTS {type: "privacy"}]->(:PrivacyProtection)
(:SearxNG)-[:USED_BY {type: "system"}]->(:SearchSystem)

// CorePlatform relationships
(:CorePlatform)-[:CONTAINS {type: "platform"}]->(:NixOS)
(:CorePlatform)-[:CONTAINS {type: "platform"}]->(:WebAssemblyRuntime)
(:CorePlatform)-[:IMPLEMENTS {type: "foundation"}]->(:SystemFoundation)
(:CorePlatform)-[:PROVIDES {type: "environment"}]->(:RuntimeEnvironment)

// DevelopmentTools relationships
(:DevelopmentTools)-[:CONTAINS {type: "tools"}]->(:BuildTools)
(:DevelopmentTools)-[:CONTAINS {type: "tools"}]->(:TestFramework)
(:DevelopmentTools)-[:IMPLEMENTS {type: "workflow"}]->(:DevelopmentWorkflow)
(:DevelopmentTools)-[:USED_BY {type: "users"}]->(:Developers)

// SecurityInfrastructure relationships
(:SecurityInfrastructure)-[:CONTAINS {type: "security"}]->(:VaultWarden)
(:SecurityInfrastructure)-[:CONTAINS {type: "security"}]->(:Matrix)
(:SecurityInfrastructure)-[:IMPLEMENTS {type: "policy"}]->(:SecurityPolicies)
(:SecurityInfrastructure)-[:PROVIDES {type: "service"}]->(:SecurityServices)
```

## Taxonomies

### Infrastructure Taxonomy

```cypher
(:Taxonomy {name: "InfrastructureTaxonomy"})
-[:CONTAINS]->(:Category {name: "Platforms"})
-[:CONTAINS]->(:Operation {name: "SystemConfiguration"})
-[:CONTAINS]->(:Operation {name: "PackageManagement"})
-[:CONTAINS]->(:Operation {name: "ApplicationDeployment"})

(:Category {name: "Services"})
-[:CONTAINS]->(:Operation {name: "DocumentManagement"})
-[:CONTAINS]->(:Operation {name: "SecurityManagement"})
-[:CONTAINS]->(:Operation {name: "SearchFunctionality"})
-[:CONTAINS]->(:Operation {name: "FeedManagement"})

(:Category {name: "Development"})
-[:CONTAINS]->(:Operation {name: "BuildProcess"})
-[:CONTAINS]->(:Operation {name: "Testing"})
-[:CONTAINS]->(:Operation {name: "Deployment"})
-[:CONTAINS]->(:Operation {name: "Monitoring"})
```

## Usage Contexts

```cypher
(:UsageContext {name: "SystemDeployment"})
-[:APPLIES_TO]->(:NixOS)
-[:REQUIRES]->(:SystemConfiguration)
-[:PRODUCES]->(:DeployedSystem)

(:UsageContext {name: "ApplicationRuntime"})
-[:APPLIES_TO]->(:WebAssemblyRuntime)
-[:REQUIRES]->(:Components)
-[:PRODUCES]->(:RunningApplications)

(:UsageContext {name: "AIModelDeployment"})
-[:APPLIES_TO]->(:Ollama)
-[:REQUIRES]->(:LLMRuntime)
-[:PRODUCES]->(:AICapabilities)

(:UsageContext {name: "WebDevelopment"})
-[:APPLIES_TO]->(:Leptos)
-[:REQUIRES]->(:WebAssembly)
-[:PRODUCES]->(:WebInterfaces)

(:UsageContext {name: "SecureCommunication"})
-[:APPLIES_TO]->(:Matrix)
-[:REQUIRES]->(:E2EEncryption)
-[:PRODUCES]->(:SecureMessaging)
```

## Code References

```cypher
(:CodeBase {path: "cim/src/infrastructure/nixos"})
-[:IMPLEMENTS]->(:NixOS)

(:CodeBase {path: "cim/src/infrastructure/wasm_runtime"})
-[:IMPLEMENTS]->(:WebAssemblyRuntime)

(:CodeBase {path: "cim/src/ai/ollama"})
-[:IMPLEMENTS]->(:Ollama)

(:CodeBase {path: "cim/src/ui/leptos"})
-[:IMPLEMENTS]->(:Leptos)

(:CodeBase {path: "cim/src/comms/matrix"})
-[:IMPLEMENTS]->(:Matrix)

(:CodeBase {path: "cim/src/docs/paperless"})
-[:IMPLEMENTS]->(:PaperlessNGX)

(:CodeBase {path: "cim/src/feeds/fresh"})
-[:IMPLEMENTS]->(:FreshRSS)

(:CodeBase {path: "cim/src/security/vault"})
-[:IMPLEMENTS]->(:VaultWarden)

(:CodeBase {path: "cim/src/search/searx"})
-[:IMPLEMENTS]->(:SearxNG)

(:CodeBase {path: "cim/src/infrastructure/core"})
-[:IMPLEMENTS]->(:CorePlatform)

(:CodeBase {path: "cim/src/infrastructure/tools"})
-[:IMPLEMENTS]->(:DevelopmentTools)

(:CodeBase {path: "cim/src/infrastructure/security"})
-[:IMPLEMENTS]->(:SecurityInfrastructure)
``` 