# Security Policy

## Supported versions

Security fixes are provided for the latest published launcher release. Older releases may stop receiving updates once a replacement is available.

The game is in early development. Security support covers the launcher, its installer and updater, and the official game packages it installs. It does not cover unofficial builds or modified packages.

## Reporting a vulnerability

Please report vulnerabilities privately through GitHub's private vulnerability reporting for this repository. Do not open a public issue for an unpatched vulnerability.

Include, where possible:

- the affected version and operating system;
- steps to reproduce the issue;
- the security impact;
- relevant logs with personal information and secrets removed; and
- whether the issue is already public or actively exploited.

We will acknowledge receipt, investigate, coordinate a fix and publish an advisory when users can update safely. Please allow time for a fix before public disclosure.

## Release security

Official launcher updates must be signed with the Tauri updater key. Pull-request builds must never receive release-signing credentials or publish release artifacts. Game archives must be cryptographically verified before installation.

Operational requirements for maintainers are documented in [docs/RELEASE_SECURITY.md](./docs/RELEASE_SECURITY.md).
