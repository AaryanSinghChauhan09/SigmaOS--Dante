# SigmaOS--Dante Component Wiki

## Overview
SigmaOS--Dante is a SigmaOS component inspired by Linux Mint, Omarchy, and Arch Linux. It provides a CLI-style tool with subcommand dispatch and fail-fast unknown-command handling.

## Principles
- Single Responsibility
- Fail-Fast
- Defensive Programming
- Component Isolation
- Stable API
- Deterministic Tests

## Architecture
Dante follows a thin CLI entry point pattern:
- **Library crate** (`src/lib.rs`): Contains the core tool logic, errors, and component handle
- **Binary entry point** (`src/main.rs`): Minimal binary that reports the component is present

## API
The component provides the following public APIs:
- `Tool` struct - CLI tool with subcommand dispatch
  - `new(name)` - Creates tool, fails fast on empty name
  - `run(args)` - Dispatches subcommand: `status`, `version`, `help`
- `Component` struct - Backwards-compatible component handle
  - `new()` - Creates component handle

## Test Structure (Arrange-Act-ASSERT)
Tests follow the AAA pattern:
1. **Arrange**: Set up test fixtures and expected states
2. **Act**: Execute the code under test
3. **Assert**: Verify the expected outcomes

Example test structure:
```rust
#[test]
fn status_and_version() {
    // Arrange
    let t = Tool::new("demo").unwrap();
    
    // Act
    let status = t.run(&["status"]).unwrap();
    let version = t.run(&["version"]).unwrap();
    
    // Assert
    assert_eq!(status, "demo: ready");
    assert!(version.contains("0.1.0"));
}
```

## Development
- All code follows defensive programming principles
- Unit tests provided for all public functionality
- Fail-fast on invalid input (empty name, unknown commands)
- No unsafe code allowed
- Component handle kept for backwards compatibility

## TODO
- Add integration tests
- Expand subcommand capabilities
- Improve error messages