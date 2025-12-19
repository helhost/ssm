# Contributing Guidelines

## Branch Protection

- Direct pushes to `main` are not allowed.
- All changes must be merged via pull request.
- Required checks (tests, lint, etc.) must pass before merging.
- Force pushes and branch deletions on `main` are disabled.


## Merging Policy

- Pull requests must target `main`.
- All required status checks must pass before merge.
- Reviews are required before merging.


## Branch Naming Convention

All branches must follow this format:

`<prefix>/<name>`

### Allowed prefixes
- `feature/`
- `bugfix/`
- `release/`
- `docs/`

### Naming rules
- Lowercase letters and numbers only
- Words separated by single hyphens
- No consecutive hyphens
- No trailing hyphens

### Examples

Valid:
- `feature/new-login`
- `bugfix/api-timeout2`
- `docs/readme-update`

Invalid:
- `feature/NewLogin`
- `feature/new_login`
- `feature/new--login`
- `feature/new-login-`


## Commit Message Guidelines

Commit messages should be clear, concise, and describe intent rather than implementation details.

### Rules

1. Use a short subject line (50 characters max)
   - Use the imperative mood
   - Do not end with a period

2. Prefix the subject with one of the following:
   - `feat:` new capability or feature
   - `fix:` bug fix
   - `docs:` documentation changes only
   - `refactor:` code change without behavior change
   - `test:` tests only
   - `chore:` tooling, configuration, or dependency updates

3. Use a commit body for non-trivial changes
   - Separate subject and body with a blank line
   - Wrap lines at approximately 72 characters
   - Explain motivation, constraints, or tradeoffs

4. Keep commits focused
   - One logical change per commit
   - If it cannot be summarized in one sentence, split it

### Examples

Valid:
- `feat: overlay blueprint on camera feed`
- `fix: correct world scale conversion`
- `refactor: isolate pose estimation pipeline`
- `docs: add branch naming rules`

Invalid:
- `updates`
- `stuff`
- `fixed bug`
- `Feat: Add new feature.`
