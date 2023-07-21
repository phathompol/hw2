use assert_cmd::Command;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn list_players_if_else_loop_1() {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd
    .assert()
    .success()
    .stdout("Player 1: N/A\nPlayer 2: N/A\n");
}

#[test]
fn list_players_if_else_loop_2() {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd
    .args(vec!["Mike"])
    .assert()
    .success()
    .stdout("Player 1: Mike\nPlayer 2: N/A\n");
}

#[test]
fn list_players_if_else_loop_last_loop() {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd
    .args(vec!["Mike", "Leo"])
    .assert()
    .success()
    .stdout("Player 1: Mike\nPlayer 2: Leo\n");
}

#[test]
fn list_players_if_else_loop_fourth_player() {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd
    .args(vec!["Mike", "Leo", "ralph"])
    .assert()
    .success()
    .stdout("Player 1: Mike\nPlayer 2: Leo\n");
}