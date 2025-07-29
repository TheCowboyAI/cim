# Commit Signing Configuration

This project uses SSH commit signing with YubiKey for enhanced security.

## Configuration

All commits should be signed using SSH keys. The project is configured to:
- Use SSH signing format
- Require YubiKey for signing operations
- Automatically sign all commits when properly configured

## Setup

```bash
git config commit.gpgsign true
git config gpg.format ssh
git config user.signingkey ~/.ssh/yubikey_signing_key.pub
```

## Verification

Signed commits will show as 'Verified' on GitHub.
