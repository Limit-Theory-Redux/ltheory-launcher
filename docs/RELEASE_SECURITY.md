# Release Security

This document defines the minimum controls for an official launcher release.

## GitHub configuration

1. Protect `main`; require pull requests, review and successful CI before merge.
2. Enable GitHub private vulnerability reporting, secret scanning and Dependabot alerts.
3. Create a `release-signing` Environment with required reviewers and no deployment branches other than `main`.
4. Store `TAURI_PRIVATE_KEY` as an Environment secret, not a repository or organization-wide secret available to ordinary workflows.
5. Create a separate `game-release-signing` Environment in the game repository and store the dedicated Minisign private key as `GAME_SIGNING_PRIVATE_KEY`.
6. Require GitHub Actions to be pinned to full commit SHAs.
7. Require two-factor authentication or passkeys for organization members with write or release access.

Pull-request workflows must not receive updater keys, code-signing credentials or write-capable tokens.

## Launcher artifacts

- Build from the frozen Bun and Cargo lockfiles.
- Run type checks, tests and dependency audits before release.
- Sign Tauri updater artifacts with the protected updater key.
- Publish checksums and a software bill of materials with the release.
- Retain the source commit and workflow run that produced each artifact.

## Game archives

The launcher must install only archives bearing a valid detached signature from the dedicated game-release key. Generate that key outside the repository and keep the private key in a protected release environment. The public verification key may be embedded in the launcher.

Do not reuse the launcher updater key for game archives. If either private key may have been exposed, rotate it, publish the new public key through a trusted release, and revoke affected artifacts.
