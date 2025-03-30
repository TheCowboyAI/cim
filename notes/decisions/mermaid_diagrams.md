# Decision: GitHub-Compatible Mermaid Diagram Formats

## Context
- The project dashboard uses Mermaid diagrams for visualization
- Initial attempts at complex Mermaid diagrams failed to render on GitHub
- Need to establish consistent, working formats for project documentation

## Decision
Established standardized Mermaid diagram formats that work reliably on GitHub:

1. **Gantt Charts**
   - Use simple date-based tasks
   - Include `excludes weekends`
   - Status indicators: `:done`, `:active`
   - Avoid complex dependencies
   - Use explicit dates

2. **Pie Charts**
   - Always include `showData`
   - Quote category names with spaces
   - Keep titles simple

3. **Flow Diagrams**
   - Use `graph LR` or `graph TD`
   - HTML line breaks with `<br/>`
   - Style definitions after nodes
   - Simple subgraph structure

## Consequences
### Positive
- Reliable rendering on GitHub
- Consistent visualization across project
- Clear documentation standards
- Better readability

### Negative
- Some advanced features unavailable
- More manual date management in Gantt charts
- Limited complexity in diagrams

## Implementation
- Created `.cursor/rules/mermaid.mdc` with detailed guidelines
- Updated project dashboard with compatible formats
- All diagrams now rendering correctly

## References
- [GitHub Mermaid Documentation](https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-diagrams#creating-mermaid-diagrams)
- [Mermaid Live Editor](https://mermaid.live)
- Project dashboard: `docs/dashboard.md`
- Guidelines: `.cursor/rules/mermaid.mdc`

## Status
- **Date**: 2024-03-29
- **Deciders**: Project team
- **Status**: Accepted and implemented 