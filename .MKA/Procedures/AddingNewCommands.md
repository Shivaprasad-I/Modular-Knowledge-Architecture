# Procedure: Adding New Commands to MKA CLI

The `mka` utility is built in Rust using `clap`. Follow these steps to extend the CLI.

## 1. Define the Command
Add the new command variant to `mka-cli/src/models/enums.rs`:
```rust
pub enum Commands {
    // ...
    NewCommand {
        #[arg(short, long)]
        option: bool,
    },
}
```

## 2. Implement the Handler
Create a new file `mka-cli/src/commands/new_command.rs`:
```rust
use anyhow::Result;

pub fn handle(option: bool) -> Result<()> {
    // Your logic here
    Ok(())
}
```
Register the module in `mka-cli/src/commands/mod.rs`.

## 3. Wire the Main Loop
Update the `match` statement in `mka-cli/src/main.rs`:
```rust
match &cli.command {
    Commands::NewCommand { option } => commands::new_command::handle(*option)?,
}
```

## 4. Documentation
After adding the command, create a new **Workflow** file in `.MKA/Workflows/` and add it to `.MKA/index.mka.yaml` so AI agents can utilize it.
