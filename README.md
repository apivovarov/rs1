# Example project VSCode Rust

## Required VSCode Extensions
- rust-analyzer
- CodeLLDB

## Create project

```
cargo new rs1

cd rs1

cargo build

cargo run
```


## VSCode

Open the project folder is VSCode

Select the "Run and Debug" icon in the Activity Bar on the side of VS Code.

Click on blie link - create launch.json file

Select LLDB in Command Palette

Cargo.toml has been detected in this workspace.
Would you like to generate launch configurations for its targets? Yes

This should generate launsh.json file with two configurations:
- Debug executable 'rs1'
- Debug unit tests in executable 'rs1'

Select one of the configurations and hit ▶.
