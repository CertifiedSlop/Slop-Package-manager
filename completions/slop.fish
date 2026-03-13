# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_slop_global_optspecs
	string join \n v/verbose c/config= d/dry-run y/yes h/help V/version
end

function __fish_slop_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_slop_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_slop_using_subcommand
	set -l cmd (__fish_slop_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c slop -n "__fish_slop_needs_command" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_needs_command" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_needs_command" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_needs_command" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_needs_command" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_needs_command" -s V -l version -d 'Print version'
complete -c slop -n "__fish_slop_needs_command" -f -a "install" -d 'Install a package by name'
complete -c slop -n "__fish_slop_needs_command" -f -a "remove" -d 'Remove a package by name'
complete -c slop -n "__fish_slop_needs_command" -f -a "search" -d 'Search for packages'
complete -c slop -n "__fish_slop_needs_command" -f -a "ai" -d 'Process a natural language request'
complete -c slop -n "__fish_slop_needs_command" -f -a "list" -d 'Show current installed packages'
complete -c slop -n "__fish_slop_needs_command" -f -a "diff" -d 'Show pending changes as a diff'
complete -c slop -n "__fish_slop_needs_command" -f -a "update" -d 'Update packages or flake inputs'
complete -c slop -n "__fish_slop_needs_command" -f -a "flake" -d 'Manage flake inputs'
complete -c slop -n "__fish_slop_needs_command" -f -a "completions" -d 'Generate shell completions'
complete -c slop -n "__fish_slop_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c slop -n "__fish_slop_using_subcommand install" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand install" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand install" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand install" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand install" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand remove" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand remove" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand remove" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand remove" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand remove" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand search" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand search" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand search" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand search" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand search" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand ai" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand ai" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand ai" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand ai" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand ai" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand list" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand list" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand list" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand list" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand list" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand diff" -s a -l add -d 'Package to add (optional, for preview)' -r
complete -c slop -n "__fish_slop_using_subcommand diff" -s r -l remove -d 'Package to remove (optional, for preview)' -r
complete -c slop -n "__fish_slop_using_subcommand diff" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand diff" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand diff" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand diff" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand diff" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand update" -s i -l input -d 'Specific input to update (for flake updates)' -r
complete -c slop -n "__fish_slop_using_subcommand update" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand update" -s f -l flake -d 'Update flake inputs instead of packages'
complete -c slop -n "__fish_slop_using_subcommand update" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand update" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand update" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand update" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -f -a "add" -d 'Add a new flake input'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -f -a "remove" -d 'Remove a flake input'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -f -a "update" -d 'Update flake inputs'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -f -a "lock" -d 'Lock flake inputs'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -f -a "list" -d 'List flake inputs'
complete -c slop -n "__fish_slop_using_subcommand flake; and not __fish_seen_subcommand_from add remove update lock list help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from add" -s u -l url -d 'Input URL (e.g., github:nixos/nixpkgs/nixos-unstable)' -r
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from add" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from add" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from add" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from add" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from add" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from remove" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from remove" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from remove" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from remove" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from remove" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from update" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from update" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from update" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from update" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from update" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from lock" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from lock" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from lock" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from lock" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from lock" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from list" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from list" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from list" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from list" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from list" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from help" -f -a "add" -d 'Add a new flake input'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from help" -f -a "remove" -d 'Remove a flake input'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from help" -f -a "update" -d 'Update flake inputs'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from help" -f -a "lock" -d 'Lock flake inputs'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from help" -f -a "list" -d 'List flake inputs'
complete -c slop -n "__fish_slop_using_subcommand flake; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c slop -n "__fish_slop_using_subcommand completions" -s s -l shell -d 'Shell to generate completions for (bash, elvish, fish, powershell, zsh)' -r
complete -c slop -n "__fish_slop_using_subcommand completions" -s c -l config -d 'Path to configuration.nix (default: /etc/nixos/configuration.nix)' -r
complete -c slop -n "__fish_slop_using_subcommand completions" -s v -l verbose -d 'Enable verbose output'
complete -c slop -n "__fish_slop_using_subcommand completions" -s d -l dry-run -d 'Enable dry-run mode (no changes applied)'
complete -c slop -n "__fish_slop_using_subcommand completions" -s y -l yes -d 'Skip confirmation prompts'
complete -c slop -n "__fish_slop_using_subcommand completions" -s h -l help -d 'Print help'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "install" -d 'Install a package by name'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "remove" -d 'Remove a package by name'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "search" -d 'Search for packages'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "ai" -d 'Process a natural language request'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "list" -d 'Show current installed packages'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "diff" -d 'Show pending changes as a diff'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "update" -d 'Update packages or flake inputs'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "flake" -d 'Manage flake inputs'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "completions" -d 'Generate shell completions'
complete -c slop -n "__fish_slop_using_subcommand help; and not __fish_seen_subcommand_from install remove search ai list diff update flake completions help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c slop -n "__fish_slop_using_subcommand help; and __fish_seen_subcommand_from flake" -f -a "add" -d 'Add a new flake input'
complete -c slop -n "__fish_slop_using_subcommand help; and __fish_seen_subcommand_from flake" -f -a "remove" -d 'Remove a flake input'
complete -c slop -n "__fish_slop_using_subcommand help; and __fish_seen_subcommand_from flake" -f -a "update" -d 'Update flake inputs'
complete -c slop -n "__fish_slop_using_subcommand help; and __fish_seen_subcommand_from flake" -f -a "lock" -d 'Lock flake inputs'
complete -c slop -n "__fish_slop_using_subcommand help; and __fish_seen_subcommand_from flake" -f -a "list" -d 'List flake inputs'
