# Cursor Rules Module
#
# This module encapsulates the Cursor rules for the project, treating them
# similar to NixOS modules for better organization and integration.
{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.cursor.rules;

  # Define the structure for a cursor rule
  ruleType = types.submodule {
    options = {
      name = mkOption {
        type = types.str;
        description = "Name of the cursor rule";
      };

      description = mkOption {
        type = types.str;
        description = "Short description of what the rule does";
      };

      content = mkOption {
        type = types.lines;
        description = "The actual content of the rule file";
      };

      path = mkOption {
        type = types.str;
        description = "Path to the rule file, relative to the .cursor/rules directory";
        default = "";
      };

      enabled = mkOption {
        type = types.bool;
        default = true;
        description = "Whether the rule is enabled";
      };
    };
  };

  # Function to generate rule files
  generateRuleFile = name: rule:
    if rule.enabled
    then
      pkgs.writeTextFile {
        name = "${name}.mdc";
        text = rule.content;
        destination = "/.cursor/rules/${
          if rule.path != ""
          then rule.path
          else "${name}.mdc"
        }";
      }
    else null;
in {
  options.cursor.rules = {
    enable = mkEnableOption "cursor rules";

    installDir = mkOption {
      type = types.str;
      default = ".cursor/rules";
      description = "Directory where cursor rules should be installed";
    };

    rules = mkOption {
      type = types.attrsOf ruleType;
      default = {};
      description = "Attribute set of cursor rules";
      example = literalExpression ''
        {
          "code-policy" = {
            name = "code-policy";
            description = "General coding rules for maintaining code quality and consistency";
            content = '''
              ## Code Policy Rules
              - Use consistent formatting
              - Write comprehensive documentation
              - Follow language-specific best practices
              ''';
          };
        }
      '';
    };
  };

  config = mkIf cfg.enable {
    # Generate the cursor rules
    environment.systemPackages =
      mapAttrsToList generateRuleFile cfg.rules;

    # Ensure the .cursor/rules directory exists
    system.activationScripts.cursorRules = ''
      mkdir -p ${cfg.installDir}
    '';
  };
}
