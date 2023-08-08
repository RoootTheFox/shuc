# shuc - don't ever accidentally run a command again¹

**shuc is a simple wrapper command that takes a command as input
and prompts you to confirm before running it.**


## usage
`shuc <command>`

## installing
- you need the rust toolchain installed on your system (as well as having .cargo/bin in your path)
- then simply run `cargo install --git https://github.com/RoootTheFox/shuc`

## extras
you can use aliases to always run common commands (such as shutdown or rm) through shuc:<br>
- `alias rm="shuc rm"`
- `alias shutdown="shuc shutdown"`

## why is it named `shuc`?
the original name intention was "**shu**tdown **c**onfirmation" but i decided it would be a better idea to make it universal

i kept the name because it was short and sounded nice idk

<br><br>
---
¹ i am not responsible for any possible fuck ups that occur while using this software