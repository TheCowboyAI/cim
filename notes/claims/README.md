# Notes Claims Directory

This directory stores extracted claims and decisions from documents processed through the project's workflow system.

## Purpose

The `/notes/claims` directory is part of the workflow specified in the project memory management system:

```
/notes -> /docs -> /vocabulary -> /code
```

Claims are extracted from documents in `/notes/incoming` and stored here as part of the knowledge extraction process. These claims can then be:

1. Reviewed for accuracy and relevance
2. Formalized into project decisions
3. Incorporated into the project's vocabulary and knowledge base
4. Used to inform documentation and code development

## Claim Files

Claim files in this directory follow this naming convention:
- `YYYY-MM-DD-document-name-claims.md`

Each claim file includes:
- Source document reference
- Extraction timestamp
- List of extracted claims
- Any additional metadata about the source

## How This Directory Is Used

1. **Automatic Extraction** - Claims are automatically extracted when running `./scripts/vocabulary_manager.sh --align`
2. **Manual Review** - Team members review claims for accuracy and importance
3. **Decision Making** - Important claims become formal decisions
4. **Documentation Integration** - Validated claims are incorporated into project documents

## Relationship to Other Directories

- `/notes/incoming` - Source documents processed to generate claims
- `/notes/decisions` - Formalized decisions based on validated claims
- `/docs` - Project documentation that incorporates important claims
- `/vocabulary` - Domain vocabulary influenced by extracted claims

## Best Practices

- Review all extracted claims for accuracy
- Flag important claims for team discussion
- Move actionable claims to decisions
- Archive processed claim files periodically
- Use claims to identify new vocabulary terms 