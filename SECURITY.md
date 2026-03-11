# Security Policy

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take the security of slop seriously. If you discover a security vulnerability, please follow these steps:

### How to Report

1. **DO NOT** create a public GitHub issue
2. Email your findings to: security@example.com
3. Include:
   - Description of the vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

### What to Expect

- **Initial Response**: Within 48 hours
- **Status Update**: Within 1 week
- **Resolution Timeline**: Depends on severity

### Security Best Practices for Users

1. **API Keys**: Never commit API keys to version control
   ```bash
   # Good
   export SLOP_AI_API_KEY="your-key"
   
   # Bad - don't do this
   # Hardcoding in configuration files
   ```

2. **Sudo Privileges**: slop requires sudo for nixos-rebuild
   - Review changes before confirming
   - Use `--dry-run` to preview

3. **Backups**: Always keep backups of your configuration
   - slop creates automatic backups
   - Store backups securely

4. **Dependencies**: We regularly audit dependencies
   ```bash
   # Check for vulnerabilities
   cargo audit
   ```

## Security Features

- ✅ Automatic configuration backups
- ✅ Syntax validation before changes
- ✅ Dry-run mode for testing
- ✅ Interactive confirmation prompts
- ✅ No sensitive data logging

## Known Limitations

- Requires sudo privileges for system modifications
- Configuration file must be writable
- AI API keys transmitted to third-party services

## Responsible Disclosure

We follow responsible disclosure practices and will:
- Credit researchers (with permission)
- Publish security advisories for significant issues
- Update documentation as needed

Thank you for helping keep slop secure!
