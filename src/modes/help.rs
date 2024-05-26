use colored::Colorize;

pub fn help() {
    println!(
        "\
{usage}: {clargo} <{title_subcommand}> <{title_args}>

{clargo} -> 
a minimal, incremental, and fast C build system.
created as a test of knowledge, not out of necessity
for just another C build system.

Clargo.toml configuration is overwritten by args.

Clargo is NOT A COMPILER! you MUST have gcc installed.

{subcommands}:
{help} -> show this menu
{init} -> initalize project 
    {name} -> sets project name
{check} -> check project (builds object files, does not link)
    {release} -> sets checking to release mode
{run} -> runs project (builds project  then runs)
    {release} -> sets building to release mode
{build} -> builds project
    {release} -> sets building to release mode
{clean} -> cleans output directory (deletes it)
",
        usage = "USAGE".green().bold(),
        clargo = "clargo".purple().bold(),
        title_subcommand = "SUBCOMMAND".green().italic(),
        title_args = "ARGS".green().italic(),
        subcommands = "SUBCOMMANDS".green().bold(),
        help = "help".cyan(),
        init = "init".cyan(),
        name = "name".blue(),
        release = "release".blue(),
        check = "check".cyan(),
        run = "run".cyan(),
        build = "build".cyan(),
        clean = "clean".cyan(),
    );
}
