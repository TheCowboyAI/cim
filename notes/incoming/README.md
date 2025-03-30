# Notes Incoming Directory

This directory serves as a staging area for documents that are ready for inspection and processing according to the workflow defined in the project memory management system.

## Purpose

The `/notes/incoming` directory follows the workflow specified in `vocabulary.mdc`:

```
/notes -> /docs -> /vocabulary -> /code
```

Documents placed in this directory will be:
1. Analyzed for terms and concepts
2. Processed for claims and decisions 
3. Integrated into the appropriate sections of the knowledge base

## How to Use

1. **Place new documents here** - When you have new research, findings, or documents that need to be integrated into the project memory
2. **Format documents properly** - Use Markdown format with clear headings and emphasized text
3. **Run the vocabulary manager** - Process documents using the `./scripts/vocabulary_manager.sh --align` command
4. **Review the extracted claims** - Claims will be placed in the `/notes/claims` directory

## Document Guidelines

For optimal processing, documents should follow these guidelines:

- Use clear, descriptive headings for main concepts
- Mark important terms with **bold text**
- Include a context section explaining the document's purpose
- Group related concepts under hierarchical headings
- Tag documents with relevant domain terms

## Processing Flow

Once documents are placed here, they will be:

1. Scanned for terms and concepts
2. Added to the documentation if appropriate
3. Integrated into the vocabulary system
4. Moved to a processed archive after successful integration

## Notes

- Documents in this directory are considered ready for processing
- The system will extract terms, concepts, and claims automatically
- Processed documents can be archived or removed after integration 