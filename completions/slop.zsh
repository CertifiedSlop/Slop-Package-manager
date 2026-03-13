#compdef slop

autoload -U is-at-least

_slop() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_slop_commands" \
"*::: :->slop" \
&& ret=0
    case $state in
    (slop)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:slop-command-$line[1]:"
        case $line[1] in
            (install)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
':package -- Package name to install:_default' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
':package -- Package name to remove:_default' \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
':query -- Search query:_default' \
&& ret=0
;;
(ai)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
':request -- Natural language description of what you want:_default' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
'-a+[Package to add (optional, for preview)]:ADD:_default' \
'--add=[Package to add (optional, for preview)]:ADD:_default' \
'-r+[Package to remove (optional, for preview)]:REMOVE:_default' \
'--remove=[Package to remove (optional, for preview)]:REMOVE:_default' \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
'-i+[Specific input to update (for flake updates)]:INPUT:_default' \
'--input=[Specific input to update (for flake updates)]:INPUT:_default' \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-f[Update flake inputs instead of packages]' \
'--flake[Update flake inputs instead of packages]' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(flake)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
":: :_slop__flake_commands" \
"*::: :->flake" \
&& ret=0

    case $state in
    (flake)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:slop-flake-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" : \
'-u+[Input URL (e.g., github\:nixos/nixpkgs/nixos-unstable)]:URL:_default' \
'--url=[Input URL (e.g., github\:nixos/nixpkgs/nixos-unstable)]:URL:_default' \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Input name:_default' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
':name -- Input name to remove:_default' \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
'::name -- Specific input to update (updates all if not specified):_default' \
&& ret=0
;;
(lock)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_slop__flake__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:slop-flake-help-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(lock)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
;;
(completions)
_arguments "${_arguments_options[@]}" : \
'-s+[Shell to generate completions for (bash, elvish, fish, powershell, zsh)]:SHELL:_default' \
'--shell=[Shell to generate completions for (bash, elvish, fish, powershell, zsh)]:SHELL:_default' \
'-c+[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'--config=[Path to configuration.nix (default\: /etc/nixos/configuration.nix)]:CONFIG:_default' \
'-v[Enable verbose output]' \
'--verbose[Enable verbose output]' \
'-d[Enable dry-run mode (no changes applied)]' \
'--dry-run[Enable dry-run mode (no changes applied)]' \
'-y[Skip confirmation prompts]' \
'--yes[Skip confirmation prompts]' \
'-h[Print help]' \
'--help[Print help]' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
":: :_slop__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:slop-help-command-$line[1]:"
        case $line[1] in
            (install)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(search)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(ai)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(diff)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(flake)
_arguments "${_arguments_options[@]}" : \
":: :_slop__help__flake_commands" \
"*::: :->flake" \
&& ret=0

    case $state in
    (flake)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:slop-help-flake-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(update)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(lock)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
(completions)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" : \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_slop_commands] )) ||
_slop_commands() {
    local commands; commands=(
'install:Install a package by name' \
'remove:Remove a package by name' \
'search:Search for packages' \
'ai:Process a natural language request' \
'list:Show current installed packages' \
'diff:Show pending changes as a diff' \
'update:Update packages or flake inputs' \
'flake:Manage flake inputs' \
'completions:Generate shell completions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'slop commands' commands "$@"
}
(( $+functions[_slop__ai_commands] )) ||
_slop__ai_commands() {
    local commands; commands=()
    _describe -t commands 'slop ai commands' commands "$@"
}
(( $+functions[_slop__completions_commands] )) ||
_slop__completions_commands() {
    local commands; commands=()
    _describe -t commands 'slop completions commands' commands "$@"
}
(( $+functions[_slop__diff_commands] )) ||
_slop__diff_commands() {
    local commands; commands=()
    _describe -t commands 'slop diff commands' commands "$@"
}
(( $+functions[_slop__flake_commands] )) ||
_slop__flake_commands() {
    local commands; commands=(
'add:Add a new flake input' \
'remove:Remove a flake input' \
'update:Update flake inputs' \
'lock:Lock flake inputs' \
'list:List flake inputs' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'slop flake commands' commands "$@"
}
(( $+functions[_slop__flake__add_commands] )) ||
_slop__flake__add_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake add commands' commands "$@"
}
(( $+functions[_slop__flake__help_commands] )) ||
_slop__flake__help_commands() {
    local commands; commands=(
'add:Add a new flake input' \
'remove:Remove a flake input' \
'update:Update flake inputs' \
'lock:Lock flake inputs' \
'list:List flake inputs' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'slop flake help commands' commands "$@"
}
(( $+functions[_slop__flake__help__add_commands] )) ||
_slop__flake__help__add_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake help add commands' commands "$@"
}
(( $+functions[_slop__flake__help__help_commands] )) ||
_slop__flake__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake help help commands' commands "$@"
}
(( $+functions[_slop__flake__help__list_commands] )) ||
_slop__flake__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake help list commands' commands "$@"
}
(( $+functions[_slop__flake__help__lock_commands] )) ||
_slop__flake__help__lock_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake help lock commands' commands "$@"
}
(( $+functions[_slop__flake__help__remove_commands] )) ||
_slop__flake__help__remove_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake help remove commands' commands "$@"
}
(( $+functions[_slop__flake__help__update_commands] )) ||
_slop__flake__help__update_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake help update commands' commands "$@"
}
(( $+functions[_slop__flake__list_commands] )) ||
_slop__flake__list_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake list commands' commands "$@"
}
(( $+functions[_slop__flake__lock_commands] )) ||
_slop__flake__lock_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake lock commands' commands "$@"
}
(( $+functions[_slop__flake__remove_commands] )) ||
_slop__flake__remove_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake remove commands' commands "$@"
}
(( $+functions[_slop__flake__update_commands] )) ||
_slop__flake__update_commands() {
    local commands; commands=()
    _describe -t commands 'slop flake update commands' commands "$@"
}
(( $+functions[_slop__help_commands] )) ||
_slop__help_commands() {
    local commands; commands=(
'install:Install a package by name' \
'remove:Remove a package by name' \
'search:Search for packages' \
'ai:Process a natural language request' \
'list:Show current installed packages' \
'diff:Show pending changes as a diff' \
'update:Update packages or flake inputs' \
'flake:Manage flake inputs' \
'completions:Generate shell completions' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'slop help commands' commands "$@"
}
(( $+functions[_slop__help__ai_commands] )) ||
_slop__help__ai_commands() {
    local commands; commands=()
    _describe -t commands 'slop help ai commands' commands "$@"
}
(( $+functions[_slop__help__completions_commands] )) ||
_slop__help__completions_commands() {
    local commands; commands=()
    _describe -t commands 'slop help completions commands' commands "$@"
}
(( $+functions[_slop__help__diff_commands] )) ||
_slop__help__diff_commands() {
    local commands; commands=()
    _describe -t commands 'slop help diff commands' commands "$@"
}
(( $+functions[_slop__help__flake_commands] )) ||
_slop__help__flake_commands() {
    local commands; commands=(
'add:Add a new flake input' \
'remove:Remove a flake input' \
'update:Update flake inputs' \
'lock:Lock flake inputs' \
'list:List flake inputs' \
    )
    _describe -t commands 'slop help flake commands' commands "$@"
}
(( $+functions[_slop__help__flake__add_commands] )) ||
_slop__help__flake__add_commands() {
    local commands; commands=()
    _describe -t commands 'slop help flake add commands' commands "$@"
}
(( $+functions[_slop__help__flake__list_commands] )) ||
_slop__help__flake__list_commands() {
    local commands; commands=()
    _describe -t commands 'slop help flake list commands' commands "$@"
}
(( $+functions[_slop__help__flake__lock_commands] )) ||
_slop__help__flake__lock_commands() {
    local commands; commands=()
    _describe -t commands 'slop help flake lock commands' commands "$@"
}
(( $+functions[_slop__help__flake__remove_commands] )) ||
_slop__help__flake__remove_commands() {
    local commands; commands=()
    _describe -t commands 'slop help flake remove commands' commands "$@"
}
(( $+functions[_slop__help__flake__update_commands] )) ||
_slop__help__flake__update_commands() {
    local commands; commands=()
    _describe -t commands 'slop help flake update commands' commands "$@"
}
(( $+functions[_slop__help__help_commands] )) ||
_slop__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'slop help help commands' commands "$@"
}
(( $+functions[_slop__help__install_commands] )) ||
_slop__help__install_commands() {
    local commands; commands=()
    _describe -t commands 'slop help install commands' commands "$@"
}
(( $+functions[_slop__help__list_commands] )) ||
_slop__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'slop help list commands' commands "$@"
}
(( $+functions[_slop__help__remove_commands] )) ||
_slop__help__remove_commands() {
    local commands; commands=()
    _describe -t commands 'slop help remove commands' commands "$@"
}
(( $+functions[_slop__help__search_commands] )) ||
_slop__help__search_commands() {
    local commands; commands=()
    _describe -t commands 'slop help search commands' commands "$@"
}
(( $+functions[_slop__help__update_commands] )) ||
_slop__help__update_commands() {
    local commands; commands=()
    _describe -t commands 'slop help update commands' commands "$@"
}
(( $+functions[_slop__install_commands] )) ||
_slop__install_commands() {
    local commands; commands=()
    _describe -t commands 'slop install commands' commands "$@"
}
(( $+functions[_slop__list_commands] )) ||
_slop__list_commands() {
    local commands; commands=()
    _describe -t commands 'slop list commands' commands "$@"
}
(( $+functions[_slop__remove_commands] )) ||
_slop__remove_commands() {
    local commands; commands=()
    _describe -t commands 'slop remove commands' commands "$@"
}
(( $+functions[_slop__search_commands] )) ||
_slop__search_commands() {
    local commands; commands=()
    _describe -t commands 'slop search commands' commands "$@"
}
(( $+functions[_slop__update_commands] )) ||
_slop__update_commands() {
    local commands; commands=()
    _describe -t commands 'slop update commands' commands "$@"
}

if [ "$funcstack[1]" = "_slop" ]; then
    _slop "$@"
else
    compdef _slop slop
fi
