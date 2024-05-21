# Contributing to the Oxide Convergence project

Thank you for your interest in contributing to the Oxide Convergence project!

**Jump To:**

- [Bug Reports](#bug-reports)
- [Feature Requests](#feature-requests)
- [Code Contributions](#code-contributions)

## Bug Reports

Bug reports are accepted through the [Issues](https://github.com/vivdiwakar/oxide-convergence/issues) page. Please label the issue as a **bug** to categorize and track it.

### Before Submitting a Bug Report

Before submitting a bug report, please do a search through the existing issues to make sure it has not already been reported. If you find that the bug has already been raised, please give it a `+1` to help us to decide which issues we prioritise.

### Submitting a Bug Report

Please ensure that your bug report contains the following:

- A short, descriptive title. Other community members should be able to understand the nature of the issue by reading this title.
- A succinct, detailed description of the problem you're experiencing. This should include:
  - Expected vs. actual behaviour exhibited
  - Any details of the execution environment that may be relevant, including Rust version numbers, etc.
  - If applicable, the exception backtrace (using `RUST_BACKTRACE=1`)
  - If you are able to create one, include a minimal working example that reproduces the issue
  - Use Markdown formatting as appropriate to make the bug report easier to read; for example use code blocks when pasting a code snippet or exception stack-trace

## Feature Requests

Feature requests are also submitted through the [Issues](https://github.com/vivdiwakar/oxide-convergence/issues) page. Please label the issue as a **feature** or **enhancement** to categorize and track it.

As with Bug Reports, please do a search of the open issues first before submitting a new one to avoid duplicates. If you do find a a feature request that represents your suggestion, please give it a +1.

**NOTE:** If you intend to implement this feature, please submit the feature request _before_ working on any code changes; this will allow members on the team to assess the idea, discuss the design with you and ensure that it makes sense to include such a feature.

### Submitting a Feature Request

Please ensure that your feature request contains the following:

- A short, descriptive title. Other community members should be able to understand the nature of the issue by reading this title.
- A detailed description of the the proposed feature, including why you believe it should be added and the benefit
- Illustrative example (pseudo-)code may also be provided to help explain how the feature should work
- Use Markdown formatting as appropriate to make the feature request easier to read
- If you plan to implement this feature yourself, please indicate as much and that you'd like to the issue to be assigned to you

## Code Contributions

This software is released under the [MIT License](https://github.com/vivdiwakar/oxide-convergence/blob/main/LICENSE); any contributed code will be released under this license.

Code contributions are handled using branching and pull requests, and should include new or modified tests that verify that the code implements the intended feature or fixes the bug, and works as intended and expected.

Before submitting a pull request, please ensure:

- Tests to exercise the new or modified behaviour are present
- Local `cargo run` succeeds
- Git commit message is detailed and includes context behind the change and links to the relevant issue number(s), if any

### Getting Your Pull Request Merged

All Pull Requests must be reviewed by the team before merging. This can usually take a couple of days (as this project is best effort and the team have day jobs and other commitments), but do feel free to nudge in the PR after a couple of days!
