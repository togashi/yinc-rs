# yinc.rs

YAML include processor - Rust implementation

A Rust port of the original Go implementation of `yinc`, providing powerful YAML file composition through include directives.

> **Note**: This project is a Rust port of the original Go implementation available at [https://github.com/togashi/yinc-go](https://github.com/togashi/yinc-go)

## Features

- **Include Directives**: Process `!include <path>` tags in YAML files
- **Multiple Formats**: Support YAML, JSON files, and HTTP/HTTPS resources
- **Special Directives**: 
  - `!include $(shell <command>)` - Execute shell commands and include output
  - `!include $(json <path>)` - Include JSON files converted to YAML
- **Glob Patterns**: Support wildcard patterns for including multiple files
- **Recursive Processing**: Handle nested includes within included files
- **Async Operations**: Built with Tokio for efficient I/O operations

## Installation

```bash
cargo install --path .
```

## Usage

```bash
yinc input.yml
```

### Command Line Options

- `-w, --indent-width <WIDTH>`: Set indentation width (default: 2)
- `-m, --multi-documents`: Output multiple documents
- `--include-tag <TAG>`: Specify include tag (default: "!include")
- `--replace-tag <TAG>`: Specify replace tag (default: "!replace")
- `-V, --version`: Show version information

## Directive Examples

### Basic File Inclusion

```yaml
# main.yml
config:
  !include config/database.yml
servers:
  !include config/servers.yml
```

### JSON File Inclusion

```yaml
# Including JSON files (auto-converted to YAML)
data: !include data.json
explicit_json: !include $(json data.json)
```

### Shell Command Execution

```yaml
# Execute shell commands and include output
git_info: !include $(shell git log -1 --format="%H %s")
current_date: !include $(shell date)
```

### HTTP Resources

```yaml
# Include remote resources
remote_config: !include https://example.com/config.yml
```

### Glob Patterns

```yaml
# Include multiple files using patterns
all_configs:
  - !include config/*.yml
```

## Supported Directive Types

| Expression | Description |
|------------|-------------|
| `!include <path>` | Include local YAML or JSON file. Path accepts glob expressions. JSON files must have '.json' extension |
| `!include <url>` | Include remote resources. Supports HTTP and HTTPS |
| `!include $(json <path>)` | Include local JSON file, converted to YAML before inclusion |
| `!include $(shell <command>)` | Include command output as YAML |

## Development

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running with Example

```bash
cargo run -- tests/fixtures/parent.yml
```

## Architecture

The project is structured as follows:

- `src/main.rs`: CLI entry point
- `src/lib.rs`: Library interface and configuration
- `src/include/`: Core include processing logic
- `src/parser/`: YAML/JSON parsing utilities
- `src/http/`: HTTP client for remote resources
- `src/shell/`: Shell command execution
- `src/error/`: Error handling

## License

MIT License - see LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
