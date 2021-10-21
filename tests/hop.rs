use assert_cmd::Command;

use std::fs;
use std::io;
use tempfile::tempdir;
use std::os::unix::fs as nixfs;
use ansi_term::Color::Yellow;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hop")?;

    const VERSION: &str = env!("CARGO_PKG_VERSION");

    cmd.arg("-V");
    cmd.assert()
        .success()
        .stdout(format!("Hop {}\n", VERSION));

    Ok(())
}

 #[test]
fn creates_hop_home() -> Result<(), Box<dyn std::error::Error>> {

    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let hop_home = hop_home_temp.as_path();

    //ensure hop home does not exist before running the exec
    fs::metadata(hop_home).expect_err(&format!("Found hop_home: {}", hop_home.to_string_lossy()));

    let mut cmd = Command::cargo_bin("hop")?;
    cmd
    .arg("-c")
    .arg(hop_home.as_os_str())
    .arg("-l")
    .assert()
    .success();

    fs::metadata(hop_home).expect(&format!("Could not find hop_home: {}", hop_home.to_string_lossy()));

    working_dir.close()?;

    Ok(())
}

#[test]
fn create_links() -> Result<(), Box<dyn std::error::Error>> {

    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let target_dir_temp = working_dir.path().join("somedir");
    let hop_home = hop_home_temp.as_path();
    let target_dir = target_dir_temp.as_path();

    fs::create_dir_all(target_dir)?;

    let mut cmd = Command::cargo_bin("hop")?;
    cmd
    .arg("-c")
    .arg(hop_home.as_os_str())
    .arg("-m")
    .arg("blee")
    .arg(target_dir.as_os_str())
    .assert()
    .success();

    fs::metadata(target_dir).expect(&format!("Could not find target dir: {}", target_dir.to_string_lossy()));

    let entries = fs::read_dir(hop_home)?.map(|res| res.map(|d| d.file_name())).collect::<Result<Vec<_>, io::Error>>()?;
    assert_eq!(entries, vec!["blee"]);

    working_dir.close()?;

    Ok(())
}

#[test]
fn read_links() -> Result<(), Box<dyn std::error::Error>> {
    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let target_dir_temp1 = working_dir.path().join("somedir1");
    let target_dir_temp2 = working_dir.path().join("somedir2");
    let target_dir_temp3 = working_dir.path().join("somedir3");
    let target_dir_temp4 = working_dir.path().join("somedir4");
    let target_dir_temp5 = working_dir.path().join("somedir5");

    let hop_home = hop_home_temp.as_path();
    let target_dir1 = target_dir_temp1.as_path();
    let target_dir2 = target_dir_temp2.as_path();
    let target_dir3 = target_dir_temp3.as_path();
    let target_dir4 = target_dir_temp4.as_path();
    let target_dir5 = target_dir_temp5.as_path();


    fs::create_dir_all(hop_home)?;
    fs::create_dir_all(target_dir1)?;
    fs::create_dir_all(target_dir2)?;
    fs::create_dir_all(target_dir3)?;

    //not symlinked later on.
    fs::create_dir_all(target_dir4)?;
    fs::create_dir_all(target_dir5)?;

    fs::metadata(target_dir1).expect(&format!("Could not find target dir1: {}", target_dir1.to_string_lossy()));
    fs::metadata(target_dir2).expect(&format!("Could not find target dir2: {}", target_dir2.to_string_lossy()));
    fs::metadata(target_dir3).expect(&format!("Could not find target dir3: {}", target_dir3.to_string_lossy()));
    fs::metadata(target_dir4).expect(&format!("Could not find target dir4: {}", target_dir4.to_string_lossy()));
    fs::metadata(target_dir5).expect(&format!("Could not find target dir5: {}", target_dir5.to_string_lossy()));


    //Symlinks created only for target_dir 1,2 and 3.
    nixfs::symlink(target_dir1, hop_home.join("tag1"))?;
    nixfs::symlink(target_dir2, hop_home.join("tag2"))?;
    nixfs::symlink(target_dir3, hop_home.join("tag3"))?;

    let mut cmd = Command::cargo_bin("hop")?;

    let output =
        cmd
        .arg("-c")
        .arg(hop_home.as_os_str())
        .arg("-l")
        .output()?;

    let output_str = String::from_utf8(output.stdout)?;
    let mut output_lines: Vec<&str> = output_str.lines().collect();
    output_lines.sort();

    assert_eq!(&output_lines, &vec!["tag1", "tag2", "tag3"]);

    working_dir.close()?;

    Ok(())
}

#[test]
fn read_tabulated_links() -> Result<(), Box<dyn std::error::Error>> {
    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let target_dir_temp1 = working_dir.path().join("somedir1");
    let target_dir_temp2 = working_dir.path().join("somedir2");
    let target_dir_temp3 = working_dir.path().join("somedir3");
    let target_dir_temp4 = working_dir.path().join("somedir4");
    let target_dir_temp5 = working_dir.path().join("somedir5");

    let hop_home = hop_home_temp.as_path();
    let target_dir1 = target_dir_temp1.as_path();
    let target_dir2 = target_dir_temp2.as_path();
    let target_dir3 = target_dir_temp3.as_path();
    let target_dir4 = target_dir_temp4.as_path();
    let target_dir5 = target_dir_temp5.as_path();

    fs::create_dir_all(hop_home)?;
    fs::create_dir_all(target_dir1)?;
    fs::create_dir_all(target_dir2)?;
    fs::create_dir_all(target_dir3)?;
    fs::create_dir_all(target_dir4)?;
    fs::create_dir_all(target_dir5)?;

    fs::metadata(target_dir1).expect(&format!("Could not find target dir1: {}", target_dir1.to_string_lossy()));
    fs::metadata(target_dir2).expect(&format!("Could not find target dir2: {}", target_dir2.to_string_lossy()));
    fs::metadata(target_dir3).expect(&format!("Could not find target dir3: {}", target_dir3.to_string_lossy()));
    fs::metadata(target_dir4).expect(&format!("Could not find target dir4: {}", target_dir4.to_string_lossy()));
    fs::metadata(target_dir5).expect(&format!("Could not find target dir5: {}", target_dir5.to_string_lossy()));

    //create symlinks to only target_dir 1,3 and 5.
    nixfs::symlink(target_dir1, hop_home.join("tag1"))?;
    nixfs::symlink(target_dir3, hop_home.join("tag3"))?;
    nixfs::symlink(target_dir5, hop_home.join("tag5"))?;

    let mut cmd = Command::cargo_bin("hop")?;

    let output =
        cmd
        .arg("-c")
        .arg(hop_home.as_os_str())
        .arg("-t")
        .output()?;

    let output_str = String::from_utf8(output.stdout)?;
    let mut output_lines: Vec<&str> = output_str.lines().collect();
    output_lines.sort();

    let expected_output =
        vec![
            format!("{} {} {}", "tag1", Yellow.paint("->"), target_dir1.to_string_lossy()),
            format!("{} {} {}", "tag3", Yellow.paint("->"), target_dir3.to_string_lossy()),
            format!("{} {} {}", "tag5", Yellow.paint("->"), target_dir5.to_string_lossy())
        ];
    assert_eq!(&output_lines, &expected_output);

    working_dir.close()?;

    Ok(())
}

#[test]
fn retrieve_link() -> Result<(), Box<dyn std::error::Error>> {

    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let target_dir_temp1 = working_dir.path().join("somedir1");
    let target_dir_temp2 = working_dir.path().join("somedir2");
    let target_dir_temp3 = working_dir.path().join("somedir3");

    let hop_home = hop_home_temp.as_path();
    let target_dir1 = target_dir_temp1.as_path();
    let target_dir2 = target_dir_temp2.as_path();
    let target_dir3 = target_dir_temp3.as_path();

    fs::create_dir_all(hop_home)?;
    fs::create_dir_all(target_dir1)?;
    fs::create_dir_all(target_dir2)?;
    fs::create_dir_all(target_dir3)?;

    fs::metadata(target_dir1).expect(&format!("Could not find target dir1: {}", target_dir1.to_string_lossy()));
    fs::metadata(target_dir2).expect(&format!("Could not find target dir2: {}", target_dir2.to_string_lossy()));
    fs::metadata(target_dir3).expect(&format!("Could not find target dir3: {}", target_dir3.to_string_lossy()));

    nixfs::symlink(target_dir1, hop_home.join("tag1"))?;
    nixfs::symlink(target_dir2, hop_home.join("tag2"))?;
    nixfs::symlink(target_dir3, hop_home.join("tag3"))?;

    let mut cmd = Command::cargo_bin("hop")?;

    let output =
        cmd
        .arg("-c")
        .arg(hop_home.as_os_str())
        .arg("-j")
        .arg("tag2")
        .output()?;

    let output_str = String::from_utf8(output.stdout)?;
    let mut output_lines: Vec<&str> = output_str.lines().collect();
    output_lines.sort();

    assert_eq!(&output_lines, &vec![target_dir2.to_string_lossy()]);

    working_dir.close()?;

    Ok(())
}

#[test]
fn delete_link() -> Result<(), Box<dyn std::error::Error>> {

    let working_dir = tempdir()?;
    let hop_home_temp = working_dir.path().join("mine").join("hophome");
    let target_dir_temp1 = working_dir.path().join("somedir1");
    let target_dir_temp2 = working_dir.path().join("somedir2");
    let target_dir_temp3 = working_dir.path().join("somedir3");

    let hop_home = hop_home_temp.as_path();
    let target_dir1 = target_dir_temp1.as_path();
    let target_dir2 = target_dir_temp2.as_path();
    let target_dir3 = target_dir_temp3.as_path();

    fs::create_dir_all(hop_home)?;
    fs::create_dir_all(target_dir1)?;
    fs::create_dir_all(target_dir2)?;
    fs::create_dir_all(target_dir3)?;

    fs::metadata(target_dir1).expect(&format!("Could not find target dir1: {}", target_dir1.to_string_lossy()));
    fs::metadata(target_dir2).expect(&format!("Could not find target dir2: {}", target_dir2.to_string_lossy()));
    fs::metadata(target_dir3).expect(&format!("Could not find target dir3: {}", target_dir3.to_string_lossy()));

    let tag1_temp = hop_home.join("tag1");
    let tag2_temp = hop_home.join("tag2");
    let tag3_temp = hop_home.join("tag3");

    let tag1 = tag1_temp.as_path();
    let tag2 = tag2_temp.as_path();
    let tag3 = tag3_temp.as_path();

    nixfs::symlink(target_dir1, tag1)?;
    nixfs::symlink(target_dir2, tag2)?;
    nixfs::symlink(target_dir3, tag3)?;

    fs::metadata(tag1).expect(&format!("Could not find target tag1: {}", tag1.to_string_lossy()));
    fs::metadata(tag2).expect(&format!("Could not find target tag2: {}", tag2.to_string_lossy()));
    fs::metadata(tag3).expect(&format!("Could not find target tag3: {}", tag3.to_string_lossy()));

    let mut command = Command::cargo_bin("hop")?;

    command
        .arg("-c")
        .arg(hop_home.as_os_str())
        .arg("-d")
        .arg("tag2")
        .write_stdin("Y".as_bytes())
        .assert()
        .success();


    fs::metadata(tag2).expect_err("Find target tag2. Expected it to be deleted");

    working_dir.close()?;

    Ok(())
}
