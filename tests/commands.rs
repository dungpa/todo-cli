use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs

#[test]
fn add_todo_item_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("todo-cli")?;

    cmd.arg("add").arg("First one");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TODO item \'First one\' added."));

    Ok(())
}