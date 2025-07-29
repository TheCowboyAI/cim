# CIM Domain Expert AI Prompts

This directory contains AI prompts that act as domain experts for Event Storming sessions. These prompts enable AI assistants to facilitate domain discovery and generate CIM-compatible domain models.

## Available Domain Experts

### Private Mortgage Lending
- **File**: `private-mortgage-lending-expert.md` - Comprehensive guide with detailed expertise
- **File**: `private-mortgage-lending-interactive.md` - Interactive session facilitator
- **File**: `private-mortgage-expert-simple.md` - Simple copy-paste version

**Expertise Areas**:
- Hard money lending
- Fix-and-flip financing
- Bridge loans
- Construction loans
- Loan origination and underwriting
- Servicing and default management

## How to Use Domain Expert Prompts

### Option 1: Quick Start (Recommended)
1. Open `private-mortgage-expert-simple.md`
2. Copy the entire prompt in the code block
3. Paste into your AI assistant (Claude, GPT-4, etc.)
4. Start with: "I need Event Storming help for my lending business"
5. Follow the AI's guidance through all phases
6. Request JSON output at the end

### Option 2: Comprehensive Session
1. Use `private-mortgage-lending-expert.md` for full context
2. Understand the complete process before starting
3. Gather your domain experts
4. Run a 3-4 hour session with the AI facilitating

### Option 3: Self-Guided Interactive
1. Use `private-mortgage-lending-interactive.md`
2. Work through each phase at your own pace
3. Reference the examples and patterns provided

## Output Format

All domain expert prompts generate CIM-compatible JSON:

```json
{
  "domain": {
    "name": "domain-name",
    "bounded_contexts": [
      {
        "name": "context-name",
        "aggregates": [
          {
            "name": "AggregateName",
            "events": [...],
            "commands": [...],
            "states": [...]
          }
        ]
      }
    ],
    "policies": [...],
    "actors": [...]
  }
}
```

## Using Generated JSON with CIM

```bash
# Save AI output
cat > my-domain.json

# Generate CIM domain
cim ai generate-domain \
  --model my-domain.json \
  --name company-domain-name \
  --output ./company-domain-name

# Creates:
# - State machines for aggregates
# - Event definitions
# - Command handlers  
# - Policy implementations
```

## Creating New Domain Expert Prompts

To create a domain expert for a new industry:

1. Copy `private-mortgage-lending-expert.md` as template
2. Replace domain expertise with your industry
3. Update expected events and aggregates
4. Modify the JSON examples
5. Test with an AI assistant
6. Submit PR to add to collection

### Template Structure
```
1. Expert Introduction (industry experience)
2. Domain Knowledge (key concepts)
3. Event Storming Process (6 phases)
4. Expected Patterns (common events)
5. JSON Output Format (CIM-compatible)
6. Interactive Examples
```

## Best Practices

1. **Real Experts**: Have actual domain experts participate
2. **Time Boxing**: Each phase has suggested time limits
3. **Stay Focused**: Events only, no implementation details
4. **Complete JSON**: Ensure all contexts and aggregates are captured
5. **Validation**: Review JSON before generating code

## Planned Domain Experts

- [ ] E-commerce and Retail
- [ ] Healthcare Claims Processing  
- [ ] Manufacturing and Supply Chain
- [ ] Insurance Underwriting
- [ ] Real Estate Transactions
- [ ] SaaS Subscription Management
- [ ] Logistics and Shipping
- [ ] Banking and Payments

## Contributing

To contribute a new domain expert:
1. Have deep knowledge of the domain
2. Understand Event Storming methodology
3. Create comprehensive prompt following template
4. Include realistic examples
5. Test with multiple AI assistants
6. Submit PR with documentation

---

*These AI domain experts enable anyone to run professional Event Storming sessions and generate CIM domains without prior facilitation experience.*