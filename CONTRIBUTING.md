*This file is just a mock at the moment, best way to get up-to-date info is to contact me or submit an issue*

# Contributing to Nexsock

First off, thank you for considering contributing to Nexsock! It's people like you that make Nexsock such a great tool.

## Code of Conduct

By participating in this project, you are expected to uphold our Code of Conduct:

- Use welcoming and inclusive language
- Be respectful of differing viewpoints and experiences
- Gracefully accept constructive criticism
- Focus on what is best for the community
- Show empathy towards other community members

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check our issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

- Your operating system name and version
- Nexsock version and installation method (pre-built binary or built from source)
- Detailed steps to reproduce the issue
- What you expected would happen
- What actually happens
- Any relevant logs or error messages

### Suggesting Enhancements

If you have a suggestion for the project, we'd love to hear it! Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

- A clear and descriptive title
- A detailed description of the proposed feature
- Any possible drawbacks or challenges you can think of
- Mock-ups or examples if applicable

### Pull Requests

Here's the process for submitting code changes:

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code follows the existing style
6. Issue that pull request!

### Development Process

1. Clone your fork and create a new branch:
   ```bash
   git clone https://github.com/<your-username>/nexsock.git
   cd nexsock
   git checkout -b my-feature
   ```

2. Make your changes and commit them:
   ```bash
   git commit -m "Description of changes"
   ```

3. Keep your fork synced:
   ```bash
   git remote add upstream https://github.com/retrokiller543/nexsock.git
   git fetch upstream
   git rebase upstream/main
   ```

### Coding Style

- Use 4 spaces for indentation
- Follow Rust style guidelines
- Use descriptive variable names
- Add comments for complex logic
- Keep functions focused and small
- Write documentation for public APIs

### Testing

- Write unit tests for new functionality
- Ensure all tests pass before submitting PR
- Include integration tests where applicable
- Test on multiple platforms if possible

### Documentation

When contributing documentation:

- Use clear and concise language
- Include code examples where appropriate
- Follow Markdown best practices
- Update relevant README sections
- Document breaking changes

## Git Commit Guidelines

- Use the present tense ("Add feature" not "Added feature")
- Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
- Limit the first line to 72 characters or less
- Reference issues and pull requests liberally after the first line

Example commit message:
```
Add service dependency validation

- Add validation for circular dependencies
- Implement dependency graph visualization
- Update documentation with new features
- Add unit tests for validation logic

Fixes #123
```

## Review Process

The project maintainers will review your contribution. They might suggest changes, improvements, or alternatives. Some things that will increase the chance that your pull request is accepted:

- Write tests
- Write good commit messages
- Follow the style guide
- Write detailed PR descriptions
- Reference relevant issues

## Project Structure

```
nexsock/
├── src/            # Source code
├── tests/          # Test files
├── docs/           # Documentation
├── examples/       # Example configurations
└── scripts/        # Build and maintenance scripts
```

## Getting Help

If you need help, you can:

- Join our Discord server
- Open a GitHub issue with your question
- Contact the maintainers directly
- Check the documentation

## Recognition

Contributors will be recognized in:

- The project's README
- Release notes
- Documentation
- GitHub's contributors page

Thank you for contributing to Nexsock!