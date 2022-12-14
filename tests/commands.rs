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

#[test]
fn remove_todo_item_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("todo-cli")?;
    cmd1.arg("reset").spawn()?.wait()?;

    let mut cmd2 = Command::cargo_bin("todo-cli")?;
    cmd2.arg("add").arg("First one").spawn()?.wait()?;

    let mut cmd = Command::cargo_bin("todo-cli")?;
    cmd.arg("remove").arg("1");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TODO item #1 removed."));

    Ok(())
}

#[test]
fn complete_todo_item_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("todo-cli")?;
    cmd1.arg("reset").spawn()?.wait()?;

    let mut cmd2 = Command::cargo_bin("todo-cli")?;
    cmd2.arg("add").arg("First one").spawn()?.wait()?;

    let mut cmd = Command::cargo_bin("todo-cli")?;
    cmd.arg("complete").arg("1");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("TODO item #1 completed."));

    Ok(())
}

#[test]
fn list_todo_items_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("todo-cli")?;
    cmd1.arg("add").arg("First one").spawn()?.wait()?;

    let mut cmd = Command::cargo_bin("todo-cli")?;
    cmd.arg("list");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Pending TODO items displayed."));

    Ok(())
}

#[test]
fn audit_todo_items_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("todo-cli")?;
    cmd1.arg("add").arg("First one").spawn()?.wait()?;

    let mut cmd = Command::cargo_bin("todo-cli")?;
    cmd.arg("audit");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All TODO items displayed."));

    Ok(())
}

#[test]
fn stats_todo_item_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("todo-cli")?;
    cmd1.arg("reset").spawn()?.wait()?;

    let mut cmd2 = Command::cargo_bin("todo-cli")?;
    cmd2.arg("add").arg("First one").spawn()?.wait()?;

    let mut cmd3 = Command::cargo_bin("todo-cli")?;
    cmd3.arg("complete").arg("1").spawn()?.wait()?;

    let mut cmd4 = Command::cargo_bin("todo-cli")?;
    cmd4.arg("add").arg("Second one").spawn()?.wait()?;

    let mut cmd = Command::cargo_bin("todo-cli")?;
    cmd.arg("stats");
    cmd.assert().success().stdout(predicate::str::contains(
        "Pending: 1 TODO item(s), completed: 1 TODO item(s).",
    ));

    Ok(())
}

#[test]
fn reset_todo_items_should_succeed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd1 = Command::cargo_bin("todo-cli")?;
    cmd1.arg("add").arg("First one").spawn()?.wait()?;

    let mut cmd = Command::cargo_bin("todo-cli")?;
    cmd.arg("reset");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("All TODO items deleted."));

    Ok(())
}
