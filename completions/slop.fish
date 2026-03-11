# slop fish completion

complete -c slop -s h -l help -d 'Print help'
complete -c slop -s V -l version -d 'Print version'
complete -c slop -s v -l verbose -d 'Enable verbose output'
complete -c slop -s c -l config -d 'Path to configuration.nix' -r -F
complete -c slop -s d -l dry-run -d 'Enable dry-run mode'
complete -c slop -s y -l yes -d 'Skip confirmation prompts'

# Install command
complete -c slop -n "__fish_use_subcommand" -a install -d 'Install a package by name'
complete -c slop -n "__fish_seen_subcommand_from install" -s h -l help -d 'Print help'
complete -c slop -n "__fish_seen_subcommand_from install" -s v -l verbose -d 'Enable verbose output'

# Remove command
complete -c slop -n "__fish_use_subcommand" -a remove -d 'Remove a package by name'
complete -c slop -n "__fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c slop -n "__fish_seen_subcommand_from remove" -s v -l verbose -d 'Enable verbose output'

# Search command
complete -c slop -n "__fish_use_subcommand" -a search -d 'Search for packages'
complete -c slop -n "__fish_seen_subcommand_from search" -s h -l help -d 'Print help'
complete -c slop -n "__fish_seen_subcommand_from search" -s v -l verbose -d 'Enable verbose output'

# AI command
complete -c slop -n "__fish_use_subcommand" -a ai -d 'Process a natural language request'
complete -c slop -n "__fish_seen_subcommand_from ai" -s h -l help -d 'Print help'
complete -c slop -n "__fish_seen_subcommand_from ai" -s v -l verbose -d 'Enable verbose output'

# List command
complete -c slop -n "__fish_use_subcommand" -a list -d 'Show current installed packages'
complete -c slop -n "__fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c slop -n "__fish_seen_subcommand_from list" -s v -l verbose -d 'Enable verbose output'

# Diff command
complete -c slop -n "__fish_use_subcommand" -a diff -d 'Show pending changes as a diff'
complete -c slop -n "__fish_seen_subcommand_from diff" -s h -l help -d 'Print help'
complete -c slop -n "__fish_seen_subcommand_from diff" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_seen_subcommand_from diff" -s a -l add -d 'Package to add'
complete -c slop -n "__fish_seen_subcommand_from diff" -s r -l remove -d 'Package to remove'
