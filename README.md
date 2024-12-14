# cursecov

Analyzes the percentage of swear word comments in JS projects.

## Why

Research has shown a [positive correlation between the use of swear words in code and the quality of code written](https://lobste.rs/s/wxlql2/is_there_correlation_between_use). At the same time, JavaScript has historically been [one of the most popular languages to build software in](https://survey.stackoverflow.co/2024/technology#most-popular-technologies). Naturally, improving the quality of code written in JS projects would have an immense impact on the software community at large, just in terms of the quantity of code improved. However, while several tools do exist in the JS space today for promoting code quality, such as [ESlint](https://eslint.org/) and [Prettier](https://prettier.io/), tooling is often insufficient at measuring the subjective nature of "good code".

Cursecov attempts to bridge this gap by providing a way for JS developers to profile the amount of swear words in projects they own and as a way to implement a forcing function that encourages swearing within the comments of a codebase (which will presumably either increase the quality of code in those projects or otherwise filter for developers that write high quality code).

## Usage

Install and run the command in your project:

```
cursecov
```

## Development

```
cargo build
cargo run
```