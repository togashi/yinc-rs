<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

# Copilot Instructions for yinc.rs

This is a Rust implementation of the YAML include processor (`yinc`), originally written in Go.

## Project Context

- **Purpose**: Process YAML files with `!include` directives to enable modular YAML composition
- **Language**: Rust (migrated from Go)
- **CLI Tool**: Uses `clap` for command-line argument parsing
- **Async**: Uses `tokio` for async operations

## Key Features

1. **Include Directives**: Process `!include <path>` tags in YAML files
2. **Multiple Formats**: Support YAML, JSON files, and HTTP/HTTPS resources
3. **Special Directives**: 
   - `!include $(shell <command>)` - Execute shell commands
   - `!include $(json <path>)` - Include JSON files converted to YAML
4. **Glob Patterns**: Support wildcard patterns for including multiple files
5. **Recursive Processing**: Handle nested includes within included files

## Architecture

- `main.rs`: CLI entry point using clap
- `lib.rs`: Library interface and configuration
- `include/`: Core include processing logic
  - `processor.rs`: Main processing engine
  - `directive.rs`: Directive parsing
  - `resolver.rs`: Path resolution utilities
- `parser/yaml.rs`: YAML/JSON parsing utilities
- `http/client.rs`: HTTP client for remote resources
- `shell/executor.rs`: Shell command execution
- `error/mod.rs`: Error handling

## Dependencies

- `clap`: CLI argument parsing
- `serde_yaml`: YAML processing
- `serde_json`: JSON processing
- `tokio`: Async runtime
- `reqwest`: HTTP client
- `glob`: Pattern matching
- `anyhow`: Error handling

## Coding Guidelines

- Use async/await for I/O operations
- Handle errors properly with custom `YincError` type
- Follow Rust naming conventions
- Use proper error propagation with `?` operator
- Maintain compatibility with the original Go implementation's behavior
