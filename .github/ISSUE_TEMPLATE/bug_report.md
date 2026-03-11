# GitHub Issue Templates

## Bug Report

```markdown
---
name: Bug Report
about: Create a report to help us improve
title: '[BUG] '
labels: bug
assignees: ''
---

**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Run `slop install <package>`
2. See error

**Expected behavior**
A clear and concise description of what you expected to happen.

**Screenshots/Output**
If applicable, add output or screenshots to help explain your problem.

**Environment:**
- OS: [e.g., NixOS 23.11]
- slop version: [e.g., 0.1.0]
- Rust version: [e.g., 1.75.0]

**Additional context**
Add any other context about the problem here.
```

## Feature Request

```markdown
---
name: Feature Request
about: Suggest an idea for this project
title: '[FEATURE] '
labels: enhancement
assignees: ''
---

**Is your feature request related to a problem? Please describe.**
A clear and concise description of what the problem is.

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
A clear and concise description of any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

## Package Alias Request

```markdown
---
name: Package Alias Request
about: Request a new package alias
title: '[ALIAS] '
labels: enhancement
assignees: ''
---

**Package name**
The actual nixpkgs attribute name (e.g., `neovim`)

**Requested aliases**
Common names users might type (e.g., `nvim`, `editor`, `vim8`)

**Category**
- [ ] Editor
- [ ] Browser
- [ ] Terminal
- [ ] Development
- [ ] System
- [ ] Other

**Justification**
Why should these aliases be added?
```
