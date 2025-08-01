# TDD/DDD/EDA Refactoring Plan for CIM

## Executive Summary

This document outlines the comprehensive refactoring plan to fully align the CIM (Composable Information Machine) project with Test-Driven Development (TDD), Domain-Driven Design (DDD), and Event-Driven Architecture (EDA) principles.

## Current State Assessment

### Strengths
- Well-structured domain layer with clear bounded contexts
- Proper event/command separation following CQRS principles
- NATS-first messaging infrastructure
- Strong typing with domain-specific value objects
- Event correlation/causation tracking implemented

### Critical Gaps
1. **TDD**: Tests exist but were written after implementation
2. **DDD**: CQRS projections and aggregates need strengthening
3. **EDA**: Event sourcing lacks persistence and versioning

## Refactoring Phases

### Phase 1: Foundation (Weeks 1-2)
**Goal**: Establish TDD workflow and enhance event sourcing

#### 1.1 TDD Workflow Implementation
- [ ] Create TDD guidelines and workflow documentation
- [ ] Set up property-based testing with `proptest`
- [ ] Implement BDD specifications using `cucumber-rust`
- [ ] Create test-first examples for each domain pattern

#### 1.2 Event Store Enhancement
- [ ] Implement persistent event store with CID chains
- [ ] Add event versioning and schema evolution
- [ ] Create snapshot strategy for aggregates
- [ ] Implement event replay capabilities

### Phase 2: CQRS Completion (Weeks 3-4)
**Goal**: Separate command and query paths completely

#### 2.1 Projection Implementation
- [ ] Create projection handlers for each aggregate
- [ ] Implement read models with proper indexing
- [ ] Add projection replay mechanisms
- [ ] Create projection consistency checks

#### 2.2 Query Optimization
- [ ] Implement query-specific data stores
- [ ] Add caching strategies for read models
- [ ] Create GraphQL API for complex queries
- [ ] Implement real-time subscriptions

### Phase 3: Advanced Patterns (Weeks 5-6)
**Goal**: Implement saga patterns and cross-domain integration

#### 3.1 Saga Implementation
- [ ] Create saga/process manager framework
- [ ] Implement compensation strategies
- [ ] Add distributed transaction patterns
- [ ] Create saga visualization tools

#### 3.2 Domain Integration
- [ ] Implement anti-corruption layers
- [ ] Create domain event translation
- [ ] Add context mapping patterns
- [ ] Implement shared kernel properly

### Phase 4: Production Readiness (Weeks 7-8)
**Goal**: Replace in-memory implementations with production systems

#### 4.1 Persistence Layer
- [ ] Integrate PostgreSQL for projections
- [ ] Implement S3/IPFS for event store
- [ ] Add proper backup/restore mechanisms
- [ ] Create data migration tools

#### 4.2 Observability
- [ ] Add OpenTelemetry integration
- [ ] Implement distributed tracing
- [ ] Create monitoring dashboards
- [ ] Add performance metrics

## Implementation Strategy

### 1. Test-First Development Process

```rust
// Example TDD workflow for new features
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // 1. Write the test first
    #[test]
    fn network_creation_requires_valid_cidr() {
        // Given
        let invalid_cidr = "invalid";
        
        // When
        let result = Network::create(invalid_cidr);
        
        // Then
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), NetworkError::InvalidCidr);
    }

    // 2. Property-based testing
    proptest! {
        #[test]
        fn network_cidr_roundtrip(cidr in valid_cidr_strategy()) {
            let network = Network::create(&cidr).unwrap();
            prop_assert_eq!(network.cidr().to_string(), cidr);
        }
    }
}

// 3. Then implement the minimal code to pass
```

### 2. Enhanced Event Sourcing

```rust
// Event versioning strategy
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "version")]
pub enum NetworkEvent {
    #[serde(rename = "1.0")]
    V1(NetworkEventV1),
    
    #[serde(rename = "2.0")]
    V2(NetworkEventV2),
}

// Event store with CID chains
pub trait EventStore {
    async fn append_events(
        &self,
        stream_id: &str,
        events: Vec<Event>,
        expected_version: ExpectedVersion,
    ) -> Result<EventMetadata>;
    
    async fn read_events(
        &self,
        stream_id: &str,
        from_version: u64,
        max_count: usize,
    ) -> Result<Vec<StoredEvent>>;
}
```

### 3. CQRS Projection Pattern

```rust
// Projection handler
pub trait ProjectionHandler {
    type Event;
    type State;
    
    async fn handle(
        &self,
        state: &mut Self::State,
        event: &Self::Event,
    ) -> Result<()>;
}

// Read model
#[derive(Debug, Clone)]
pub struct NetworkReadModel {
    pub id: NetworkId,
    pub name: String,
    pub cidr: Cidr,
    pub subnets: Vec<SubnetSummary>,
    pub last_updated: DateTime<Utc>,
    pub version: u64,
}
```

### 4. Saga Pattern Implementation

```rust
// Saga definition
pub trait Saga {
    type Command;
    type Event;
    type State;
    
    fn handle_event(
        &self,
        state: &Self::State,
        event: &Self::Event,
    ) -> Result<Vec<Self::Command>>;
    
    fn compensate(
        &self,
        state: &Self::State,
        error: &Error,
    ) -> Result<Vec<Self::Command>>;
}
```

## Success Metrics

### Technical Metrics
- Test coverage > 90% with mutation testing
- All new features developed test-first
- Event replay time < 5 seconds for 1M events
- Projection lag < 100ms for 99th percentile

### Process Metrics
- 100% of PRs include tests written first
- All domain logic has BDD specifications
- Zero untested production code paths
- Saga completion rate > 99.9%

## Migration Path

### Week 1-2: Foundation
1. Set up TDD tooling and documentation
2. Create first test-driven feature as example
3. Implement basic persistent event store

### Week 3-4: Gradual Adoption
1. Require TDD for all new features
2. Backfill tests for critical paths
3. Implement first production projection

### Week 5-6: Full Implementation
1. Complete saga framework
2. Migrate all in-memory stores
3. Add monitoring and tracing

### Week 7-8: Validation
1. Performance testing and optimization
2. Chaos engineering tests
3. Production readiness review

## Risk Mitigation

### Technical Risks
- **Risk**: Performance degradation from event sourcing
  - **Mitigation**: Implement snapshots and projections
  
- **Risk**: Complexity from saga patterns
  - **Mitigation**: Start with simple workflows, add visualization

### Process Risks
- **Risk**: Team resistance to TDD
  - **Mitigation**: Provide training, pair programming, clear benefits

- **Risk**: Increased development time
  - **Mitigation**: Measure long-term velocity improvements

## Conclusion

This refactoring will transform CIM into a truly event-driven, domain-focused system with comprehensive test coverage. The phased approach ensures continuous delivery while improving architecture quality.

## Next Steps

1. Review and approve this plan
2. Set up TDD tooling and guidelines
3. Create first test-driven feature example
4. Begin Phase 1 implementation

---

*Document Version: 1.0*  
*Last Updated: 2025-07-31*  
*Status: DRAFT*