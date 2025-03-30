# Notes Directory

This directory contains research notes, decisions, meeting notes, reviews, and exploratory documents related to the CIM project.

## Directory Structure

- `/notes/incoming/`: Staging area for new documents awaiting processing
- `/notes/claims/`: Repository for extracted claims from documents
- `/notes/research/`: Research findings and evaluations
- `/notes/decisions/`: Technical and architectural decisions
- `/notes/meetings/`: Meeting notes and outcomes
- `/notes/reviews/`: Code and design review notes
- `/notes/explorations/`: Technical explorations and experiments
- `/notes/comments/`: General comments and observations

## Knowledge Flow Process

We follow a structured knowledge flow process:

1. **Document Creation**: New documents are placed in `/notes/incoming/`
2. **Document Processing**: Documents are analyzed and claims are extracted
3. **Claim Extraction**: Individual claims are stored in `/notes/claims/`
4. **Knowledge Organization**: Processed information is organized into appropriate notes subdirectories
5. **Documentation**: Key information is consolidated into formal documentation in `/docs/`
6. **Vocabulary Management**: Domain vocabulary is extracted and managed in `/vocabulary/`
7. **Code Implementation**: Implementation based on the established vocabulary

The complete flow: `/notes` → `/docs` → `/vocabulary` → `/code`

## File Naming Convention

Files should follow the naming convention:
```
YYYY-MM-DD-brief-title.md
```

## Required Sections

Each note should include:
- **Context**: Why this note exists
- **Details**: Main content
- **Decisions**: Any decisions made
- **Next Steps**: Follow-up actions
- **References**: Links to related notes/issues

## Working with Notes

### Adding New Documents
1. Place new documents in `/notes/incoming/`
2. Run the claim extraction process
3. Organize extracted claims and documents into appropriate directories

### Processing Claims
1. Review extracted claims in `/notes/claims/`
2. Validate and categorize claims
3. Update related documentation based on claims

### Updating Documentation
1. Review notes for significant information
2. Update relevant files in `/docs/`
3. Ensure cross-references are maintained
4. Update project memory as needed

## Scripts and Tools

Several scripts are available to assist with the notes workflow:
- `vocabulary_manager.sh`: Unified interface for vocabulary management
- Claim extraction scripts
- Notes-vocabulary alignment tools

## Knowledge Graph

Notes are organized into a knowledge graph structure using Mermaid diagrams, allowing for visualization of relationships between notes and concepts.

## Structure

```
notes/
├── claims/          # Ideas with repeatable construction
├── comments/        # Comments about progress
├── decisions/           # Decisions 
├── ideas/           # Preliminary thoughts
├── arguments/       # Support for/against claims
├── goals/           # Achievement targets
├── organizations/   # Structural entities

## Guidelines

1. Notes are raw research material
2. Each note focuses on a single concept/idea
3. Use cross-references between related notes
4. Include source materials and references
5. Track note status and updates
6. Use consistent terminology from vocabulary
7. Keep notes atomic and focused 


## Note Format

Each note should follow this format:

```markdown
# Note Title

## Metadata
- Category: [category name]
- Date: YYYY-MM-DD
- Status: [draft|review|verified]
- Related: [links to related notes]
- Source: [if applicable]

## Content
[note content]

## References
- [reference links]
- [source materials]

## Updates
- YYYY-MM-DD: [update description]
```

### TARGET DOCS FROM NOTES

### Knowledge Foundation
- **Facts**: Proven claims with verifiable evidence
- **Claims**: Ideas with repeatable construction
- **Theories**: Beliefs with context and sources
- **Ideas**: Preliminary thoughts
- **Arguments**: Support or opposition for claims

### Organizational Elements
- **Goals**: Achievement targets
- **Organizations**: Structural entities
- **Operators**: System controllers
- **Models**: System representations
- **People**: Human resources
- **Agents**: Autonomous entities
- **Accounts**: Managed groups

### Business Components
- **Value Propositions**: Benefit offerings
- **Solutions**: Problem resolutions
- **Proposals**: Formal suggestions
- **Business Models**: Operational frameworks

### Governance
- **Policies**: Operational guidelines
- **Laws**: Regulatory framework
- **Ethics**: Moral principles
- **Politics**: Power dynamics
- **Relationships**: Inter-entity connections

### Technical & Environmental
- **Equipment**: Physical resources
- **Environment**: Contextual settings
- **Location**: Spatial information
- **Secrets**: Protected information
- **Behaviors**: Action patterns
- **Axioms**: Fundamental truths
- **Sources**: Information origins

## Contributing
1. Place notes in appropriate categories
2. Follow the note format
3. Maintain cross-references
4. Keep information atomic
5. Include all relevant metadata
6. Update related notes when needed 