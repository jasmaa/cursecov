# cursecov

Analyzes and enforces a threshold percentage of swear word comments in JS projects.

## Why

JavaScript has historically been [one of the most popular languages to build software in](https://survey.stackoverflow.co/2024/technology#most-popular-technologies). At the same time, code quality is a common and often difficult problem to solve that exists within many high contributor codebases today. One may reason thus that improving the quality of code written in JS projects could have an immense impact on the software community at large, just in terms of sheer quantity of code improved.

Today, several tools exist in the JS space for promoting and maintaining high quality code, such as [ESlint](https://eslint.org/) and [Prettier](https://prettier.io/). While these tools are good at establishing a set of ground rules, they are often insufficient at measuring the more subjective sides of "good code".

Cursecov attempts to bridge this gap. Based on scientific research which has shown a [positive correlation between the use of swear words in code and the quality of code written](https://lobste.rs/s/wxlql2/is_there_correlation_between_use), Cursecov provides a way for JS developers to profile the amount of swear words in projects they own and enforce a minimum amount of swearing which can be evaluated at build time. This will theoretically either increase the quality of code in those projects or otherwise filter primarily for developers who write "high quality code".

## Usage

Install and run the command in your project:

```
cursecov -h
```

## Development

```
cargo build
cargo test
cargo run -- -h
```