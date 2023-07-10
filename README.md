# Workspace Assessment

In this assessment you will be setting up a Cargo workspace that will allow you to have multiple crates together in the same project. This is the recommended setup for using SeaORM in your project. It can also be helpful in breaking out logic into separate independent projects.

You will notice that you cannot see auto completions from Rust Analyzer in the sub crates until you set up the workspace.

## Skills

- Rust workspaces

## Grading Rubric

To pass the assessment you will need to do the following

- Convert the root project into a Cargo workspace

With the server running, the tests can be run with the command cargo test. The tests ensure that the above requirements are passing. However they are some other requirements that are necessary to pass this assessment.

    Code is formatted
    Cargo linting doesn't find any problems
    Clippy is happy

You can format your code with the command cargo fmt. It is possible to set auto-formatting on your code editor of choice so that this happens automatically when you save.

You can lint your code with the command cargo check. Cargo check can be configured to run in your editor of choice to tell you if there are linting problems.

Clippy is an optional linter that goes above and beyond the normal linting that Cargo check provides. It's a helpful tool as it can provide alternate (usually better) methods of writing the same code. You can run clippy with the command cargo clippy. It is possible to set clippy to run instead of the normal Cargo check linter so that you get it's lintings instead.

To run all of these checks run the shell script ./check.sh. If there is output it will put it in a file named check.out. It will validate that the tests are passing, the code is formatted, and both the linters don't have any errors or warnings.