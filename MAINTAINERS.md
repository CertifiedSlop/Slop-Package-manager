# Maintainers Guide

This document provides guidelines and responsibilities for maintainers of the slop project.

## Table of Contents

- [Maintainer Responsibilities](#maintainer-responsibilities)
- [Decision Making](#decision-making)
- [Release Process](#release-process)
- [Code Review Guidelines](#code-review-guidelines)
- [Issue Triage](#issue-triage)
- [Security Response](#security-response)
- [Communication](#communication)
- [Onboarding New Maintainers](#onboarding-new-maintainers)

---

## Maintainer Responsibilities

### Weekly Tasks

- [ ] Review pending PRs (aim for < 1 week response time)
- [ ] Review new issues and ensure proper labeling
- [ ] Check CI/CD pipeline status
- [ ] Respond to discussions and questions
- [ ] Monitor dependency updates and security advisories

### Monthly Tasks

- [ ] Plan and execute releases
- [ ] Review roadmap progress
- [ ] Update documentation as needed
- [ ] Review and update issue/PR templates
- [ ] Check test coverage trends
- [ ] Review and update dependencies

### Quarterly Tasks

- [ ] Evaluate contributor progression
- [ ] Review and update roadmap
- [ ] Assess project health metrics
- [ ] Plan major features and improvements
- [ ] Community outreach and engagement

---

## Decision Making

### Consensus Model

We use a consensus-based decision-making model:

1. **Proposal** - Any maintainer can propose a change
2. **Discussion** - Allow 1 week for discussion (2 weeks for major changes)
3. **Decision** - Consensus among active maintainers
4. **Implementation** - Assign and implement the decision

### Voting

When consensus cannot be reached:

- Each maintainer gets 1 vote
- Simple majority (>50%) for routine decisions
- Supermajority (>66%) for breaking changes
- Ties are broken by the project lead (if designated)

### Areas Requiring Maintainer Approval

- Merging breaking changes
- Releasing new versions
- Adding new maintainers
- Changing project license
- Major architectural changes
- Security patches

---

## Release Process

### Release Schedule

- **Patch releases** (0.1.x) - As needed for bug fixes
- **Minor releases** (0.x.0) - Monthly or when features accumulate
- **Major releases** (x.0.0) - When breaking changes are ready

### Pre-Release Checklist

```markdown
## Release Preparation

- [ ] Update CHANGELOG.md with all changes
- [ ] Bump version in Cargo.toml
- [ ] Run all tests: `cargo test`
- [ ] Run clippy: `cargo clippy -- -D warnings`
- [ ] Format code: `cargo fmt`
- [ ] Update documentation if needed
- [ ] Create release branch: `release/v0.1.0`
- [ ] Test release candidate
- [ ] Get maintainer approval
```

### Release Steps

1. **Create release commit**
   ```bash
   git checkout -b release/v0.1.0
   git commit -m "chore: prepare release v0.1.0"
   ```

2. **Tag the release**
   ```bash
   git tag -a v0.1.0 -m "Release v0.1.0"
   ```

3. **Push to GitHub**
   ```bash
   git push origin release/v0.1.0
   git push origin v0.1.0
   ```

4. **Create GitHub Release**
   - Go to Releases page
   - Create release from tag
   - Copy CHANGELOG.md content
   - Mark as latest release

5. **Publish to crates.io** (if applicable)
   ```bash
   cargo publish
   ```

### Post-Release

- [ ] Announce release in Discussions
- [ ] Update wiki if needed
- [ ] Monitor for issues
- [ ] Thank contributors

---

## Code Review Guidelines

### Review Expectations

- **Response time**: Within 48 hours
- **Thoroughness**: Check functionality, tests, and documentation
- **Tone**: Constructive and helpful

### Review Checklist

```markdown
## Code Review

### Functionality
- [ ] Feature works as described
- [ ] Edge cases handled
- [ ] Error handling appropriate
- [ ] No breaking changes (or documented)

### Code Quality
- [ ] Follows Rust style guidelines
- [ ] No clippy warnings
- [ ] Code is formatted
- [ ] Functions are appropriately sized

### Testing
- [ ] Tests included for new functionality
- [ ] Existing tests pass
- [ ] Test coverage adequate

### Documentation
- [ ] Public functions documented
- [ ] README updated (if needed)
- [ ] CHANGELOG updated
- [ ] Examples provided (if needed)

### Security
- [ ] No sensitive data exposure
- [ ] Input validation present
- [ ] No new vulnerabilities
```

### Merging PRs

- Require at least 1 maintainer approval
- All CI checks must pass
- Author should not merge their own PR (if possible)
- Squash commits for clean history

---

## Issue Triage

### Label Guide

| Label | Description |
|-------|-------------|
| `bug` | Confirmed bug |
| `enhancement` | Feature request |
| `documentation` | Documentation improvement |
| `good first issue` | Beginner-friendly |
| `help wanted` | Need community help |
| `roadmap` | Planned feature |
| `high priority` | Urgent attention needed |
| `blocked` | Waiting on something |
| `duplicate` | Already reported |
| `invalid` | Not a valid issue |
| `wontfix` | Will not be fixed |

### Triage Process

1. **Acknowledge** - Respond within 48 hours
2. **Categorize** - Add appropriate labels
3. **Prioritize** - Set priority level
4. **Assign** - Assign to maintainer or leave open
5. **Close** - Close if duplicate/invalid (with explanation)

### Response Templates

**Bug Report:**
```
Thanks for reporting this! We'll investigate. Can you provide:
- Your NixOS version
- slop version
- Steps to reproduce
```

**Feature Request:**
```
Great idea! This aligns with our roadmap. We'll discuss priority.
```

**Duplicate:**
```
This is a duplicate of #123. Please follow that issue for updates.
```

---

## Security Response

### Vulnerability Report Process

1. **Acknowledge** - Within 48 hours
2. **Assess** - Determine severity
3. **Fix** - Develop and test patch
4. **Release** - Publish security release
5. **Disclose** - Public advisory after fix available

### Security Release

For critical vulnerabilities:

1. Create fix in private branch
2. Test thoroughly
3. Coordinate with affected parties
4. Release and publish advisory
5. Update SECURITY.md if needed

---

## Communication

### Channels

- **GitHub Issues** - Bug reports and feature requests
- **GitHub Discussions** - Questions and community
- **GitHub PRs** - Code contributions
- **Email** - Security issues (see SECURITY.md)

### Response Time Expectations

| Channel | Expected Response |
|---------|------------------|
| Security Report | 48 hours |
| Bug Report | 1 week |
| Feature Request | 2 weeks |
| PR Review | 48 hours |
| Discussion | 1 week |

### Public Representation

When representing slop publicly:

- Be respectful and inclusive
- Acknowledge it's experimental software
- Don't make promises about timelines
- Direct questions to GitHub Discussions
- Report security issues privately

---

## Onboarding New Maintainers

### Nomination

1. Existing maintainer nominates
2. Discuss in private maintainer channel
3. Vote (requires >66% approval)
4. Extend invitation

### Onboarding Checklist

```markdown
## New Maintainer Onboarding

- [ ] Add to GitHub maintainers team
- [ ] Grant repository access
- [ ] Add to CI/CD systems
- [ ] Share credentials (if applicable)
- [ ] Schedule onboarding call
- [ ] Review this document together
- [ ] Assign first maintainer tasks
- [ ] Announce to community
```

### First Tasks

- Review and merge a few PRs
- Triage some issues
- Plan next release
- Update roadmap

---

## Stepping Down

If a maintainer needs to step down:

1. Notify other maintainers
2. Transfer responsibilities
3. Remove from teams/permissions
4. Optionally, become emeritus maintainer

**Emeritus Maintainers:**
- Listed in README.md
- No merge/access permissions
- Always welcome to contribute

---

## Current Maintainers

| Name | GitHub | Role |
|------|--------|------|
| (Add maintainers here) | @username | Project Lead |
| | @username | Maintainer |

---

## Thank You

Being a maintainer is a commitment. Thank you for dedicating your time to making slop better for everyone! 🦀

**Remember:**
- It's okay to say "I don't know"
- Delegate when possible
- Take breaks to avoid burnout
- Celebrate wins and contributors
