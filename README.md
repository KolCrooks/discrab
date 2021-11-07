
Just a rust discord wrapper.

## Project design philosophy:
- Highly scalable and resource efficient framework
- Easy to use, intuitive, and generally high level
- Highly Customizable
- A framework that is ready for commercial use

## Core features:
- Macros w/ structs to create slash commands
- Auto registration of commands

## Cool ideas we want to implement:
- Unit testing integration
  - This would involve being able to simulate events to the bot so that users can test functionality
- Middlewares


## Planned Usage:

main.rs: 
```rust
use command::Command;
use command_group::CommandGroup;
use command_with_subcommands::ParentCommand;

fn main() {
    Bot::new("BOT_TOKEN")
        .add_commands([CommandGroup, ParentCommand, Command])
        .run();
}
```

command.rs
```rust
#[command]
struct Command;

impl CommandTrait for Command {
    NAME = "my_command";
    DESCRIPTION = "Does some stuff.";
    // OTHER STUFF

    fn run(ctx: CommandContext) {
        // DO SOME THINGS
    }
}
```

command_group.rs
```rust
#[command_group]
struct CommandGroup;

impl CommandGroupTrait for CommandGroup {
    NAME = "command_group";
    DESCRIPTION = "A group of commands.";
    // OTHER STUFF
}

#[command(CommandGroup)]
struct CommandInGroup;

impl CommandTrait for CommandInGroup {
    NAME = "command_in_group";
    DESCRIPTION = "A command in a command group.";
    // OTHER STUFF
    
    fn run(ctx CommandContext) {
        // DO STUFF
    }
}
```

command_with_subcommands.rs
```rust
#[command]
struct ParentCommand;

impl CommandTrait for ParentCommand {
    NAME = "parent_command";
    DESCRIPTION = "A command with a subcommand.";
    // OTHER STUFF

    fn run(ctx CommandContext) {
        // DO STUFF
    }
}

#[command(ParentCommand)]
struct Subcommand;

impl CommandTrait for Subcommand {
    NAME = "subcommand";
    DESCRIPTION = "A subcommand of a parent command.";
    // OTHER STUFF

    fn run(ctx CommandContext) {
        // DO STUFF
    }
}
```



- Unit testing integration