# Git Workflow

## Branching Strategy
- `main`: The stable production branch.
- `develop`: The active development branch. All feature branches merge here.
- `feature/*`: For new features or improvements.
- `release/*`: Preparing a new release.
- `hotfix/*`: For critical bugs in production.

## Commit Strategy
We use Conventional Commits. Examples:
- `feat:` A new feature
- `fix:` A bug fix
- `docs:` Documentation only changes
- `refactor:` Code change that neither fixes a bug nor adds a feature
- `test:` Adding missing tests
- `perf:` A code change that improves performance
- `chore:` Changes to the build process or auxiliary tools
