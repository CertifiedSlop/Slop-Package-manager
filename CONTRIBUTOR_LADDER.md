# Contributor Ladder

This document outlines the path for contributing to slop and progressing through different roles in the project.

## Table of Contents

- [Overview](#overview)
- [Contributor Levels](#contributor-levels)
  - [User](#user)
  - [Contributor](#contributor)
  - [Active Contributor](#active-contributor)
  - [Triage](#triage)
  - [Maintainer](#maintainer)
- [How to Progress](#how-to-progress)
- [Responsibilities by Level](#responsibilities-by-level)

---

## Overview

We welcome contributions at all levels! Whether you're fixing a typo in documentation or architecting major features, every contribution is valued.

```
User → Contributor → Active Contributor → Triage → Maintainer
```

---

## Contributor Levels

### User

**What you do:**
- Use slop in your NixOS configuration
- Report bugs and issues
- Suggest features
- Share feedback

**How to get started:**
1. Install slop on your system
2. Use it for package management
3. Report issues when you encounter problems
4. Suggest improvements via GitHub Issues

**Recognition:**
- Listed as a user in community stats
- Welcome in GitHub Discussions

---

### Contributor

**What you do:**
- Submit pull requests (PRs)
- Fix bugs
- Add small features
- Improve documentation
- Add package aliases

**Requirements:**
- Have at least 1 merged PR
- Follow the [Contributing Guide](CONTRIBUTING.md)
- Write tests for code changes
- Use conventional commit messages

**Permissions:**
- Can be assigned to issues
- Can be mentioned in PRs for review

**How to become a contributor:**
1. Read [CONTRIBUTING.md](CONTRIBUTING.md)
2. Find an issue labeled `good first issue` or `help wanted`
3. Fork the repository
4. Create a branch: `git checkout -b feature/your-feature`
5. Make your changes
6. Run tests: `cargo test`
7. Run linter: `cargo clippy -- -D warnings`
8. Format code: `cargo fmt`
9. Submit a PR

**Recognition:**
- Listed in CONTRIBUTORS.md (if created)
- Contributions visible on GitHub profile

---

### Active Contributor

**What you do:**
- Regularly submit PRs
- Help review other contributors' PRs
- Participate in discussions
- Mentor new contributors

**Requirements:**
- Have at least 5 merged PRs
- Consistent contributions over 2+ months
- Help review at least 3 PRs from others
- Participate in GitHub Discussions

**Permissions:**
- Can be assigned to more complex issues
- Can label issues (with guidance)
- Invited to contributor discussions

**How to become an active contributor:**
1. Continue contributing regularly
2. Start reviewing other PRs
3. Help answer questions in Discussions
4. Take on more complex issues

**Recognition:**
- Public acknowledgment in release notes
- Invitation to contributor chat/channel

---

### Triage

**What you do:**
- Review and label incoming issues
- Review PRs for quality and correctness
- Help prioritize work
- Close duplicate or invalid issues

**Requirements:**
- Have at least 10 merged PRs
- Consistent contributions over 3+ months
- Demonstrate understanding of project goals
- Good communication skills
- Regular participation in issue discussions

**Permissions:**
- Can add/remove labels on issues and PRs
- Can close duplicate/invalid issues
- Can assign issues to contributors
- Can mark issues as stale
- Voting rights on roadmap priorities

**How to become triage:**
1. Express interest to maintainers
2. Demonstrate consistent helpfulness
3. Show good judgment in issue discussions
4. Be nominated by a maintainer
5. Accept the role and responsibilities

**Responsibilities:**
- Review new issues weekly
- Label and categorize issues
- Welcome new contributors
- Help maintain issue hygiene

**Recognition:**
- Triage badge on GitHub profile
- Listed in README.md as triage member
- Access to triage-only discussions

---

### Maintainer

**What you do:**
- Set project direction and roadmap
- Review and merge PRs
- Make releases
- Represent the project publicly
- Make final decisions on contentious issues

**Requirements:**
- Have at least 20 merged PRs
- Consistent contributions over 6+ months
- Deep understanding of codebase
- Excellent communication skills
- Commitment to project success
- Nominated by existing maintainers

**Permissions:**
- Merge PRs
- Create releases
- Push to main branch (with review)
- Manage GitHub repository settings
- Access to project resources
- Final say on technical decisions

**How to become a maintainer:**
1. Express interest to existing maintainers
2. Demonstrate leadership and expertise
3. Show commitment to project vision
4. Be nominated by existing maintainer
5. Consensus approval from maintainers
6. Accept the role and responsibilities

**Responsibilities:**
- Weekly PR reviews
- Monthly release cycle
- Roadmap planning
- Community engagement
- Mentoring triage and contributors
- Security response

**Recognition:**
- Maintainer badge on GitHub profile
- Listed in README.md as maintainer
- Decision-making authority
- Project representation rights

---

## How to Progress

### General Guidelines

1. **Be consistent** - Regular contributions matter more than bursts
2. **Be helpful** - Help others succeed
3. **Be patient** - Progress takes time
4. **Be communicative** - Engage in discussions
5. **Be reliable** - Follow through on commitments

### Timeline Expectations

| Level | Minimum Time | Minimum PRs | Key Activity |
|-------|-------------|-------------|--------------|
| User | - | 0 | Use and report |
| Contributor | 1 month | 1 | Submit PRs |
| Active Contributor | 2 months | 5 | Regular contributions |
| Triage | 3 months | 10 | Review and label |
| Maintainer | 6 months | 20 | Lead and decide |

*Note: These are guidelines, not strict rules. Exceptional contributors may progress faster.*

---

## Responsibilities by Level

### Code Quality

| Level | Expected Quality |
|-------|-----------------|
| Contributor | Tests pass, follows style guide |
| Active Contributor | Well-tested, documented |
| Triage | Reviews others' code quality |
| Maintainer | Sets quality standards |

### Community Engagement

| Level | Expected Engagement |
|-------|---------------------|
| Contributor | Responds to PR feedback |
| Active Contributor | Helps in discussions |
| Triage | Welcomes new contributors |
| Maintainer | Leads community initiatives |

### Time Commitment

| Level | Expected Time |
|-------|---------------|
| Contributor | As available |
| Active Contributor | Few hours/week |
| Triage | 2-4 hours/week |
| Maintainer | 5-10 hours/week |

---

## Stepping Down

If you need to step down from a role:

1. Notify other maintainers/triage
2. Update GitHub permissions
3. Optionally, take a break instead of leaving
4. You're always welcome back!

---

## Questions?

- Open a GitHub Discussion
- Ask in existing contributor channels
- Talk to a maintainer directly

---

**Thank you for contributing to slop!** 🦀

Every contribution, from documentation to code to community help, makes the project better.
