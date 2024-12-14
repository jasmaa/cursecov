use assert_cmd::Command;
use assert_fs::prelude::{FileTouch, FileWriteStr, PathChild};
use predicates::prelude::{predicate, PredicateBooleanExt};

#[test]
fn it_fails_when_no_files() {
    let mut cmd = Command::cargo_bin("cursecov").unwrap();
    cmd.assert().failure();
}

#[test]
fn it_fails_when_one_file_exists_without_curse_comments() {
    let temp = assert_fs::TempDir::new().unwrap();

    let hello_file = temp.child("hello.js");
    hello_file.touch().unwrap();
    hello_file.write_str("console.log('hello world')").unwrap();

    let mut cmd = Command::cargo_bin("cursecov").unwrap();
    cmd.current_dir(temp.path()).assert().failure();
}

#[test]
fn it_succeeds_when_one_file_with_curse_comments() {
    let temp = assert_fs::TempDir::new().unwrap();

    let hello_file = temp.child("hello.js");
    hello_file.touch().unwrap();
    hello_file
        .write_str(
            "// this is a fucking console log
console.log('hello world')",
        )
        .unwrap();

    let mut cmd = Command::cargo_bin("cursecov").unwrap();
    cmd.current_dir(temp.path()).assert().success();
}

#[test]
fn it_fails_when_multiple_files_without_curse_comments() {
    let temp = assert_fs::TempDir::new().unwrap();

    let hello_file = temp.child("hello.js");
    hello_file.touch().unwrap();
    hello_file.write_str("console.log('hello world')").unwrap();

    let hello_file_2 = temp.child("foo/hello2.js");
    hello_file_2.touch().unwrap();
    hello_file_2
        .write_str(
            "const foo = 12;

console.log(foo);",
        )
        .unwrap();

    let hello_file_3 = temp.child("hello3.txt");
    hello_file_3.touch().unwrap();
    hello_file_3
        .write_str("This fucking text file has no impact on the cursecov")
        .unwrap();

    let mut cmd = Command::cargo_bin("cursecov").unwrap();
    cmd.current_dir(temp.path()).assert().failure();
}

#[test]
fn it_succeeds_when_multiple_files_with_curse_comments() {
    let temp = assert_fs::TempDir::new().unwrap();

    let hello_file = temp.child("hello.js");
    hello_file.touch().unwrap();
    hello_file
        .write_str(
            "// this is a fucking console log
console.log('hello world')",
        )
        .unwrap();

    let hello_file_2 = temp.child("foo/hello2.js");
    hello_file_2.touch().unwrap();
    hello_file_2
        .write_str(
            "// this is the dumbass declaration
const foo = 12;

// then it prints the motherfucking value
console.log(foo);",
        )
        .unwrap();

    let hello_file_3 = temp.child("hello3.txt");
    hello_file_3.touch().unwrap();
    hello_file_3
        .write_str("This fucking text file has no impact on the cursecov")
        .unwrap();

    let mut cmd = Command::cargo_bin("cursecov").unwrap();
    cmd.current_dir(temp.path()).assert().success();
}

#[test]
fn it_fails_when_not_enough_curse_comments() {
    let temp = assert_fs::TempDir::new().unwrap();

    let hello_file = temp.child("hello.js");
    hello_file.touch().unwrap();
    hello_file
        .write_str(
            "// this is a test program

// it prints hello world in javascript.

// this is the place where the fucking print statement happens
console.log('hello world')",
        )
        .unwrap();

    let mut cmd = Command::cargo_bin("cursecov").unwrap();
    cmd.current_dir(temp.path())
        .arg("--min-coverage")
        .arg("100")
        .assert()
        .failure();
}

#[test]
fn it_succeeds_when_in_verbose_mode() {
    let temp = assert_fs::TempDir::new().unwrap();

    let hello_file = temp.child("hello.js");
    hello_file.touch().unwrap();
    hello_file
        .write_str(
            "// this is a fucking console log
console.log('hello world')",
        )
        .unwrap();

    let hello_file_2 = temp.child("foo/hello2.js");
    hello_file_2.touch().unwrap();
    hello_file_2
        .write_str(
            "// this is the dumbass declaration
const foo = 12;

// then it prints the motherfucking value
console.log(foo);",
        )
        .unwrap();

    let mut cmd = Command::cargo_bin("cursecov").unwrap();
    cmd.current_dir(temp.path()).arg("-v").assert().stdout(
        predicate::str::contains("hello.js").and(predicate::str::contains("foo/hello2.js")),
    );
}
