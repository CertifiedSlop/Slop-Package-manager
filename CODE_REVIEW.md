# Code Review Guidelines

## For Reviewers

### Checklist

- [ ] **Code Quality**
  - Code follows Rust style guidelines
  - No clippy warnings (`cargo clippy -- -D warnings`)
  - Code is formatted (`cargo fmt`)
  - Functions are appropriately sized

- [ ] **Functionality**
  - Feature works as described
  - Edge cases are handled
  - Error handling is appropriate
  - No breaking changes (or properly documented)

- [ ] **Testing**
  - Tests are included for new functionality
  - Existing tests still pass
  - Test coverage is adequate

- [ ] **Documentation**
  - Public functions have doc comments
  - README is updated (if needed)
  - CHANGELOG is updated
  - Examples are provided (if needed)

- [ ] **Security**
  - No sensitive data exposure
  - Input validation is present
  - No new security vulnerabilities

### Review Response Time

- Aim to review within 48 hours
- Be constructive and helpful
- Explain the "why" behind suggestions

## For Authors

### Before Requesting Review

1. Run all checks:
   ```bash
   # Use nix develop for consistent environment
   nix develop --command cargo fmt
   nix develop --command cargo clippy -- -D warnings
   nix develop --command cargo test
   ```

2. Update documentation:
   - Code comments
   - README (if user-facing change)
   - CHANGELOG

3. Write clear commit messages

### CI/CD Checklist

Before merging, ensure:
- [ ] All CI checks pass (rust-matrix, nix-flake-build, security-audit, docs)
- [ ] No clippy warnings
- [ ] Tests pass on all Rust versions in matrix
- [ ] Documentation builds without warnings
- [ ] Security audit passes

### During Review

- Respond to all comments
- Make requested changes promptly
- Ask for clarification if needed
- Be open to feedback

## Merging

- All CI checks must pass
- At least one approval required
- Author should not merge their own PR (if possible)
- Squash commits for clean history

---

Thank you for maintaining code quality! 🦀
