# Cursor Rules Module

This module encapsulates Cursor rules for the CIM project as a NixOS-style module, providing a structured and declarative way to manage Cursor rules.

## Overview

Cursor rules are AI assistant instructions that help guide code interactions and enforce project patterns and standards. By treating these rules as a NixOS-style module, we gain several benefits:

1. **Declarative Configuration**: Rules are defined in a structured, typesafe way
2. **Version Control**: Rule definitions are tracked alongside code
3. **Reproducibility**: Rules can be consistently deployed across environments
4. **Modularity**: Rules can be composed and extended like other modules

## Structure

The module consists of the following components:

- `default.nix`: The module definition, defining the structure and options
- `config.nix`: The module configuration, containing the actual rule definitions
- `update-rules.sh`: A utility script for managing rules
- `README.md`: This documentation file

## Usage

### Using the Cursor Rules Module

To use this module, import it in your main flake.nix or configuration:

```nix
{
  imports = [
    ./modules/cursor-rules
  ];
  
  cursor.rules = {
    enable = true;
    # Additional configuration if needed
  };
}
```

### Managing Rules with the Utility Script

The module includes a utility script for managing rules:

```bash
# Show status of all rules
./modules/cursor-rules/update-rules.sh status

# List all configured rules
./modules/cursor-rules/update-rules.sh list

# Export rules from the module to .cursor/rules directory
./modules/cursor-rules/update-rules.sh export

# Import rules from .cursor/rules directory to the module
./modules/cursor-rules/update-rules.sh import

# Synchronize rules between module and .cursor/rules directory
./modules/cursor-rules/update-rules.sh sync
```

### Adding a New Rule

To add a new rule, edit `config.nix` and add an entry to the `rules` attribute set:

```nix
"my-new-rule" = {
  name = "my-new-rule";
  description = "Description of what this rule does";
  enabled = true;
  content = ''
    # My New Rule
    
    This is the content of my new rule.
    
    ## Section 1
    - Rule guideline 1
    - Rule guideline 2
    
    ## Section 2
    More rule content...
  '';
};
```

## Integration with Cursor

The rules defined in this module are intended to be used with the Cursor IDE. The module can automatically generate the rule files in the `.cursor/rules` directory, where Cursor will pick them up and apply them to your coding sessions.

## Benefits Over Manual Rule Management

1. **Consistency**: Rules are defined in a consistent format
2. **Validation**: The module can validate rule structure
3. **Documentation**: Rules are self-documenting
4. **Automation**: Rules can be generated, updated, and managed through automation

## Future Enhancements

- Complete implementation of rule synchronization
- Add validation for rule content
- Integrate with CI/CD pipeline
- Add rule inheritance and composition capabilities 