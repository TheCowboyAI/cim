use cim_client::{MCPClient, Result};
use tracing::{info, error};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create a new MCP client connected to the local NATS server
    info!("Connecting to NATS server...");
    let client = MCPClient::new("nats://localhost:4222")
        .await?
        .with_timeout(10); // Set a 10-second timeout

    // List all ontologies
    info!("Listing ontologies...");
    match client.list_ontologies().await {
        Ok(ontologies) => {
            if ontologies.is_empty() {
                info!("No ontologies found. Creating a sample ontology...");
                
                // Create a sample ontology
                let ontology = client.create_ontology(
                    "Sample Ontology",
                    "An example ontology created by the simple client example",
                    "Example",
                ).await?;
                
                info!("Created ontology: {} (ID: {})", ontology.name, ontology.id);
                
                // Create some terms
                info!("Creating terms...");
                let term1 = client.create_term(
                    &ontology.id,
                    "Term 1",
                    "The first example term",
                    None,
                ).await?;
                
                let term2 = client.create_term(
                    &ontology.id,
                    "Term 2",
                    "The second example term",
                    None,
                ).await?;
                
                // Create a relationship
                info!("Creating relationship...");
                let relationship = client.create_relationship(
                    &ontology.id,
                    "related_to",
                    &term1.id,
                    &term2.id,
                    None,
                ).await?;
                
                info!("Created relationship: {} -> {} -> {}", 
                    term1.name, 
                    relationship.relationship_type, 
                    term2.name
                );
                
                // Fetch the ontology to see the full structure
                info!("Fetching complete ontology...");
                let complete_ontology = client.get_ontology(&ontology.id).await?;
                
                info!("Ontology structure:");
                info!("  Name: {}", complete_ontology.name);
                info!("  Description: {}", complete_ontology.description);
                info!("  Domain: {}", complete_ontology.domain);
                info!("  Terms: {}", complete_ontology.terms.len());
                info!("  Relationships: {}", complete_ontology.relationships.len());
            } else {
                info!("Found {} ontologies:", ontologies.len());
                for (i, ontology) in ontologies.iter().enumerate() {
                    info!("{}. {} (ID: {})", i + 1, ontology.name, ontology.id);
                    info!("   Domain: {}", ontology.domain);
                    info!("   Terms: {}", ontology.terms.len());
                    info!("   Relationships: {}", ontology.relationships.len());
                }
            }
        },
        Err(e) => {
            error!("Failed to list ontologies: {}", e);
        }
    }

    Ok(())
} 