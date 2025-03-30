use clap::{Parser, Subcommand};
use std::collections::HashMap;
use tokio::signal;
use tracing::{error, info};

use cim_client::{MCPClient, Result};

/// CIM Ontology Tool NATS-based client
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// NATS server URL
    #[arg(short, long, default_value = "nats://localhost:4222")]
    nats_url: String,

    /// Authentication token for MCP
    #[arg(short, long)]
    token: Option<String>,

    /// Timeout in seconds for requests
    #[arg(short, long, default_value_t = 30)]
    timeout: u64,

    /// Subcommand to execute
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all ontologies
    ListOntologies,

    /// Get a specific ontology by ID
    GetOntology {
        /// Ontology ID
        id: String,
    },

    /// Create a new ontology
    CreateOntology {
        /// Ontology name
        name: String,
        
        /// Ontology description
        description: String,
        
        /// Ontology domain
        domain: String,
    },

    /// Update an existing ontology
    UpdateOntology {
        /// Ontology ID
        id: String,
        
        /// New ontology name (optional)
        #[arg(long)]
        name: Option<String>,
        
        /// New ontology description (optional)
        #[arg(long)]
        description: Option<String>,
        
        /// New ontology domain (optional)
        #[arg(long)]
        domain: Option<String>,
    },

    /// Delete an ontology
    DeleteOntology {
        /// Ontology ID
        id: String,
    },

    /// Create a new term
    CreateTerm {
        /// Ontology ID
        ontology_id: String,
        
        /// Term name
        name: String,
        
        /// Term description
        description: String,
    },

    /// Create a relationship between terms
    CreateRelationship {
        /// Ontology ID
        ontology_id: String,
        
        /// Relationship type
        relationship_type: String,
        
        /// Source term ID
        source_term_id: String,
        
        /// Target term ID
        target_term_id: String,
    },

    /// Listen for MCP events
    Listen,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Create MCP client
    let mut client = MCPClient::new(&cli.nats_url).await?;

    // Add token if provided
    if let Some(token) = cli.token {
        client = client.with_token(token);
    }

    // Set timeout
    client = client.with_timeout(cli.timeout);

    // Execute command
    match cli.command {
        Commands::ListOntologies => {
            let ontologies = client.list_ontologies().await?;
            if ontologies.is_empty() {
                println!("No ontologies found.");
            } else {
                println!("Found {} ontologies:", ontologies.len());
                for (i, ont) in ontologies.iter().enumerate() {
                    println!("{}. {} (ID: {})", i + 1, ont.name, ont.id);
                    println!("   Domain: {}", ont.domain);
                    println!("   Description: {}", ont.description);
                    println!("   Terms: {}", ont.terms.len());
                    println!("   Relationships: {}", ont.relationships.len());
                    println!();
                }
            }
        }
        
        Commands::GetOntology { id } => {
            let ontology = client.get_ontology(&id).await?;
            println!("Ontology: {} (ID: {})", ontology.name, ontology.id);
            println!("Domain: {}", ontology.domain);
            println!("Description: {}", ontology.description);
            println!("Created: {}", ontology.created_at);
            println!("Updated: {}", ontology.updated_at);
            
            println!("\nTerms:");
            if ontology.terms.is_empty() {
                println!("  No terms defined.");
            } else {
                for (i, term) in ontology.terms.iter().enumerate() {
                    println!("{}. {} (ID: {})", i + 1, term.name, term.id);
                    println!("   Description: {}", term.description);
                }
            }
            
            println!("\nRelationships:");
            if ontology.relationships.is_empty() {
                println!("  No relationships defined.");
            } else {
                for (i, rel) in ontology.relationships.iter().enumerate() {
                    println!("{}. {} (ID: {})", i + 1, rel.relationship_type, rel.id);
                    println!("   Source: {}", rel.source_term_id);
                    println!("   Target: {}", rel.target_term_id);
                }
            }
        }
        
        Commands::CreateOntology { name, description, domain } => {
            let ontology = client.create_ontology(&name, &description, &domain).await?;
            println!("Created ontology: {} (ID: {})", ontology.name, ontology.id);
        }
        
        Commands::UpdateOntology { id, name, description, domain } => {
            let ontology = client.update_ontology(
                &id, 
                name.as_deref(), 
                description.as_deref(), 
                domain.as_deref()
            ).await?;
            println!("Updated ontology: {} (ID: {})", ontology.name, ontology.id);
        }
        
        Commands::DeleteOntology { id } => {
            client.delete_ontology(&id).await?;
            println!("Deleted ontology with ID: {}", id);
        }
        
        Commands::CreateTerm { ontology_id, name, description } => {
            let term = client.create_term(&ontology_id, &name, &description, None).await?;
            println!("Created term: {} (ID: {})", term.name, term.id);
            println!("In ontology: {}", term.ontology_id);
        }
        
        Commands::CreateRelationship { ontology_id, relationship_type, source_term_id, target_term_id } => {
            let relationship = client.create_relationship(
                &ontology_id,
                &relationship_type,
                &source_term_id,
                &target_term_id,
                None
            ).await?;
            println!("Created relationship: {} (ID: {})", relationship.relationship_type, relationship.id);
            println!("Between: {} -> {}", relationship.source_term_id, relationship.target_term_id);
        }
        
        Commands::Listen => {
            println!("Listening for MCP events. Press Ctrl+C to exit.");
            
            let mut events = client.subscribe_events().await?;
            let mut interrupt = signal::ctrl_c();
            
            loop {
                tokio::select! {
                    Some(event) = events.next() => {
                        println!("Event: {:?}", event);
                    }
                    _ = &mut interrupt => {
                        println!("\nReceived interrupt signal, exiting...");
                        break;
                    }
                }
            }
        }
    }

    Ok(())
} 