# slop zsh completion

#zsh completion

_slop() {
  local -a cmd
  _arguments -C \
    '(- :)'"{--help,-h}":'Print help' \
    '(- :)'"{--version,-V}":'Print version' \
    '(- *)'"{--verbose,-v}":'Enable verbose output' \
    '(- *)'"{--config,-c}":'Path to configuration.nix' \
    '(- *)'"{--dry-run,-d}":'Enable dry-run mode' \
    '(- *)'"{--yes,-y}":'Skip confirmation prompts' \
    "1: :->cmnds" \
    "*::arg:->args"

  case $state in
  cmnds)
    cmd=(
      "install:Install a package by name"
      "remove:Remove a package by name"
      "search:Search for packages"
      "ai:Process a natural language request"
      "list:Show current installed packages"
      "diff:Show pending changes as a diff"
    )
    _describe "command" cmd
    ;;
  esac

  case "$words[1]" in
  install)
    _arguments \
      "{--help,-h}":'Print help' \
      "{--verbose,-v}":'Enable verbose output' \
      "1:package:Package name to install"
    ;;
  remove)
    _arguments \
      "{--help,-h}":'Print help' \
      "{--verbose,-v}":'Enable verbose output' \
      "1:package:Package name to remove"
    ;;
  search)
    _arguments \
      "{--help,-h}":'Print help' \
      "{--verbose,-v}":'Enable verbose output' \
      "1:query:Search query"
    ;;
  ai)
    _arguments \
      "{--help,-h}":'Print help' \
      "{--verbose,-v}":'Enable verbose output' \
      "1:request:Natural language description"
    ;;
  list)
    _arguments \
      "{--help,-h}":'Print help' \
      "{--verbose,-v}":'Enable verbose output'
    ;;
  diff)
    _arguments \
      "{--help,-h}":'Print help' \
      "{--verbose,-v}":'Enable verbose output' \
      "{--add,-a}":'Package to add' \
      "{--remove,-r}":'Package to remove'
    ;;
  esac
}

if [ "$funcstack[1]" = "_slop" ]; then
  _slop "$@"
else
  compdef _slop slop
fi
