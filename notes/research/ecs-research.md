# Entity Component System (ECS) in Messaging Systems

## Core ECS Concepts

### 1. Entity Definition
- **Purpose**: Unique identifier for domain objects
- **Characteristics**:
  - Immutable identity
  - No behavior
  - No state
  - Used for component attachment
- **Implementation**:
  ```rust
  #[derive(Debug, Clone, Hash, Eq, PartialEq)]
  pub struct EntityId(Uuid);

  impl EntityId {
      pub fn to_subject(&self) -> String {
          format!("entity.{}", self.0)
      }
  }
  ```

### 2. Component Structure
- **Purpose**: Pure data containers
- **Characteristics**:
  - Immutable data
  - No behavior
  - Versioned
  - Timestamped
- **Implementation**:
  ```rust
  #[derive(Debug, Clone, Serialize, Deserialize)]
  pub struct Component<T> {
      entity_id: EntityId,
      data: T,
      version: u64,
      timestamp: DateTime<Utc>,
  }
  ```

### 3. System Implementation
- **Purpose**: Process components and communicate
- **Characteristics**:
  - Stateless processing
  - Message-based communication
  - Error handling
  - Async operations
- **Implementation**:
  ```rust
  pub trait System {
      type Component: Serialize + DeserializeOwned;
      
      fn process(&self, component: &Component<Self::Component>) -> Result<Command, SystemError>;
      fn subject_pattern(&self) -> String;
  }
  ```

## DDD Integration

### 1. Domain Model Mapping
- **Entity to Domain Entity**:
  - ECS EntityId maps to DDD Entity identity
  - Components map to Entity state
  - Systems map to Domain Services
- **Value Objects**:
  - Implemented as immutable components
  - No identity tracking
  - Pure data representation

### 2. Aggregate Roots
- **Implementation**:
  - Root entity with components
  - Invariant enforcement
  - Transaction boundaries
  - Consistency rules
- **Example**:
  ```rust
  pub struct Order {
      id: EntityId,
      components: HashMap<ComponentType, Component>,
  }

  impl Order {
      pub fn add_item(&mut self, item: OrderItem) -> Result<(), OrderError> {
          let item_component = Component::new(self.id, item);
          self.validate_invariants(&item_component)?;
          self.components.insert(ComponentType::Item, item_component);
          Ok(())
      }
  }
  ```

### 3. Domain Events
- **Implementation**:
  - Component changes as events
  - Event sourcing
  - Event publishing
  - Event handling
- **Example**:
  ```rust
  pub struct OrderCreated {
      order_id: EntityId,
      customer_id: EntityId,
      items: Vec<OrderItem>,
      timestamp: DateTime<Utc>,
  }

  impl From<Component<Order>> for OrderCreated {
      fn from(component: Component<Order>) -> Self {
          OrderCreated {
              order_id: component.entity_id,
              customer_id: component.data.customer_id,
              items: component.data.items,
              timestamp: component.timestamp,
          }
      }
  }
  ```

## Messaging Patterns

### 1. Component Registration
- **Purpose**: Track component availability
- **Implementation**:
  ```rust
  pub struct ComponentRegistry {
      components: HashMap<String, Vec<EntityId>>,
  }

  impl ComponentRegistry {
      pub async fn register<T: Serialize>(
          &mut self,
          nats: &async_nats::Client,
          component: &Component<T>,
      ) -> Result<(), RegistryError> {
          let subject = format!("registry.{}", std::any::type_name::<T>());
          self.components
              .entry(subject.clone())
              .or_default()
              .push(component.entity_id.clone());
              
          nats.publish(subject, serde_json::to_vec(&component)?.into()).await?;
          Ok(())
      }
  }
  ```

### 2. State Management
- **Purpose**: Track entity state changes
- **Implementation**:
  ```rust
  pub struct EntityState {
      entity_id: EntityId,
      components: HashSet<String>,
      version: u64,
  }

  impl EntityState {
      pub async fn update_component<T: Serialize>(
          &mut self,
          nats: &async_nats::Client,
          component: &Component<T>,
      ) -> Result<(), StateError> {
          self.version += 1;
          self.components.insert(std::any::type_name::<T>().to_string());
          
          nats.publish(
              self.entity_id.to_subject(),
              serde_json::to_vec(&self)?.into(),
          ).await?;
          
          Ok(())
      }
  }
  ```

### 3. Query Interface
- **Purpose**: Find entities with components
- **Implementation**:
  ```rust
  pub struct QueryBuilder {
      required_components: Vec<String>,
      nats: async_nats::Client,
  }

  impl QueryBuilder {
      pub async fn with_component<T>(&mut self) -> &mut Self {
          self.required_components.push(std::any::type_name::<T>().to_string());
          self
      }
      
      pub async fn execute(&self) -> Result<Vec<EntityId>, QueryError> {
          let subject = format!("query.{}", self.required_components.join("."));
          let response = self.nats
              .request(subject, vec![].into())
              .await?;
              
          let entities: Vec<EntityId> = serde_json::from_slice(&response.payload)?;
          Ok(entities)
      }
  }
  ```

## Best Practices

### 1. Component Design
- Keep components pure data
- Version component schemas
- Include metadata
- Handle component evolution

### 2. System Implementation
- Stateless processing
- Clear error handling
- Async operations
- Proper logging

### 3. Performance
- Component caching
- Batch operations
- Efficient queries
- Resource monitoring

### 4. Testing
- Component unit tests
- System integration tests
- State management tests
- Query interface tests

## References
1. "Entity Component System" by Robert Nystrom
2. "Domain-Driven Design" by Eric Evans
3. "Implementing Domain-Driven Design" by Vaughn Vernon
4. NATS Documentation: https://docs.nats.io/ 