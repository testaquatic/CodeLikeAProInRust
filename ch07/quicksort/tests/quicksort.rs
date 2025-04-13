use std::fs;

use assert_cmd::Command;
use proptest::{prelude::prop, proptest};
use quicksort::{Quicksort, quicksort};

#[test]
fn test_quicksort() {
    let mut values = vec![12, 1, 5, 0, 6, 2];
    quicksort(&mut values);
    assert_eq!(values, vec![0, 1, 2, 5, 6, 12]);

    let mut values = vec![1, 13, 5, 10, 6, 2, 0];
    quicksort(&mut values);
    assert_eq!(values, vec![0, 1, 2, 5, 6, 10, 13]);
}

#[test]
fn test_quicksort_trait() {
    let mut values = vec![12, 1, 5, 0, 6, 2];
    values.quicksort();
    assert_eq!(values, vec![0, 1, 2, 5, 6, 12]);

    let mut values = vec![1, 13, 5, 10, 6, 2, 0];
    values.quicksort();
    assert_eq!(values, vec![0, 1, 2, 5, 6, 10, 13]);
}

#[test]
fn test_no_args() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("quicksort")?;
    cmd.assert().success().stdout("[]\n");

    Ok(())
}

#[test]
fn test_cli_well_known() -> Result<(), anyhow::Error> {
    let mut cmd = Command::cargo_bin("quicksort")?;
    cmd.args(&["14", "52", "1", "-195", "1582"])
        .assert()
        .success()
        .stdout("[-195, 1, 14, 52, 1582]\n");

    Ok(())
}

#[test]
fn test_cli_fixtures() -> Result<(), anyhow::Error> {
    let paths = fs::read_dir("tests/fixtures")?;
    for fixture in paths {
        let mut path = fixture?.path();
        path.push("args");
        let args = fs::read_to_string(&path)?
            .trim()
            .split(' ')
            .map(str::to_owned)
            .collect::<Vec<_>>();
        path.pop();
        path.push("expected");
        let expected = fs::read_to_string(&path)?;

        let mut cmd = Command::cargo_bin("quicksort")?;
        cmd.args(&args).assert().success().stdout(expected);
    }

    Ok(())
}

proptest! {
    #[test]
    fn test_quicksort_proptest(vec in prop::collection::vec(prop::num::i64::ANY, 0..1000)) {
        let mut vec_sorted = vec.clone();
        vec_sorted.quicksort();

        let mut vec_quicksorted = vec.clone();
        vec_quicksorted.quicksort();

        assert_eq!(vec_quicksorted, vec_sorted);
    }
}
