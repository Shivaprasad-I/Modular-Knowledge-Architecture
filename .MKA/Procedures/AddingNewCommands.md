# Procedure: Adding New Commands to MKA-CLI

Follow this procedure to extend the `mka` utility with new functionality.

## 1. Define the Command
Add your new command to the `Commands` enum in `mka-cli/src/models/enums/cli_commands.rs`.
- Use `clap` attributes for documentation and arguments.
- Example:
  ```rust
  NewCommand {
      #[arg(short, long)]
      force: bool,
  },
  ```

## 2. Create the Handler
Create a new file `mka-cli/src/commands/<command_name>.rs`.
- Implement a `pub fn handle(...) -> Result<()>` function.
- Utilize `crate::analyzer`, `crate::models`, and `crate::utils` as needed.

## 3. Register the Module
Add the new module to `mka-cli/src/commands/mod.rs`:
```rust
pub mod <command_name>;
```

## 4. Wire the Command
Update the `match` statement in `mka-cli/src/main.rs` to call your new handler.

## 5. Build and Run
Build the utility in release mode for best performance:
```bash
cd mka-cli
cargo build --release
```

Run the binary:
```bash
./target/release/mka <your-command>
```

## 6. Document in MKA
After adding the command, create a new workflow file in `.MKA/Workflows/` and add it to `.MKA/index.mka.yaml` so AI agents can utilize it.
