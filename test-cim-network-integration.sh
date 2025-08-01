#!/usr/bin/env bash
set -euo pipefail

echo "Testing CIM Network Module Integration"
echo "======================================"

# Create a test project
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

echo "Creating test Rust project..."
cargo init --name cim-network-test

# Add cim-network as a git dependency pointing to local path
cat > Cargo.toml << EOF
[package]
name = "cim-network-test"
version = "0.1.0"
edition = "2021"

[dependencies]
cim-network = { git = "https://github.com/TheCowboyAI/cim-network.git" }
tokio = { version = "1", features = ["full"] }
EOF

# Create a simple test program
cat > src/main.rs << 'EOF'
use cim_network::{NetworkId, SubnetId, NetworkCommand, NetworkEvent};

#[tokio::main]
async fn main() {
    println!("Testing CIM Network Module Integration");
    
    // Create some IDs
    let network_id = NetworkId::new();
    let subnet_id = SubnetId::new();
    
    println!("Created Network ID: {:?}", network_id);
    println!("Created Subnet ID: {:?}", subnet_id);
    
    // Create a network command
    let command = NetworkCommand::CreateNetwork {
        id: network_id,
        name: "test-network".to_string(),
        cidr: "10.0.0.0/16".to_string(),
    };
    
    println!("Created command: {:?}", command);
    
    println!("✅ CIM Network module is working!");
}
EOF

echo "Building test project..."
cargo build 2>&1 || {
    echo "❌ Build failed. This is expected since the GitHub repo is empty."
    echo "   Once cim-network is published to GitHub, this test will work."
}

echo
echo "Test directory: $TEST_DIR"
echo
echo "To make this work:"
echo "1. Push the cim-network code to https://github.com/TheCowboyAI/cim-network"
echo "2. Update the flake.nix to uncomment the cim-network input"
echo "3. Run 'nix flake update' to fetch the input"
echo "4. Run 'nix build .#cim-network' to build the package"