use super::HopProgram;
use crate::algebra::hop::DeleteStatus;
use crate::algebra::symlinks::{SymLink, SymLinks};
use crate::algebra::{directories::Directories, std_io::StdIO, user_dirs::UserDirs};
use crate::models::{HomeType, HopEffect, Link, LinkPair};

use std::cell::Cell;
use std::io;
use std::path::{Path, PathBuf};

enum SymLinkDeleteStatus {
    Succeeded,
    Failed,
}

type HomeStatusError = String;

enum GetHopHomeStatus {
    Succeeded(PathBuf),      //Success with path
    Failed(HomeStatusError), //Failure with error
}

struct TestStub<'a> {
    out: &'a Cell<Vec<String>>,
    input: &'a Cell<Vec<String>>,
    get_hop_home: GetHopHomeStatus,
    read_dir_links: Result<Vec<LinkPair>, String>,
    dir_exists: bool,
    link_exists: bool,
    write_link: Option<String>,
    delete_link: SymLinkDeleteStatus,
}

struct Test<'a> {
    stub: TestStub<'a>,
}

impl<'a> TestStub<'a> {
    fn new(output: &'a Cell<Vec<String>>) -> Self {
        Self {
            out: output,
            input: output, //make these equal, because we don't use input usually
            get_hop_home: GetHopHomeStatus::Succeeded(PathBuf::from("/xyz/.your-hop")),
            read_dir_links: Ok(Vec::new()),
            dir_exists: true,
            link_exists: false,
            write_link: None,
            delete_link: SymLinkDeleteStatus::Succeeded,
        }
    }

    fn with_std_in(output: &'a Cell<Vec<String>>, input: &'a Cell<Vec<String>>) -> Self {
        let default = TestStub::new(output);
        TestStub { input, ..default }
    }

    fn with_read_links(output: &'a Cell<Vec<String>>, read_links: Vec<LinkPair>) -> Self {
        let default = TestStub::new(output);
        TestStub {
            read_dir_links: Ok(read_links),
            ..default
        }
    }

    fn with_read_links_and_std_in(
        output: &'a Cell<Vec<String>>,
        read_links: Vec<LinkPair>,
        input: &'a Cell<Vec<String>>,
    ) -> Self {
        let default = TestStub::with_std_in(output, input);
        TestStub {
            read_dir_links: Ok(read_links),
            ..default
        }
    }


    fn program(stub: Self) -> HopProgram<Test<'a>> {
        HopProgram {
            value: Test{ stub },
            hop_home_dir: HomeType::Relative(".xyz".to_string()),
        }
    }
}

impl StdIO for Test<'_> {
    fn println(&self, message: &str) {
        let old_vec = &mut self.stub.out.take();
        old_vec.push(message.to_string());
        self.stub.out.set(old_vec.to_vec())
    }

    fn readln(&self) -> HopEffect<String> {
        let old_vec = &mut self.stub.input.take();
        let result = old_vec.remove(0);
        self.stub.input.set(old_vec.to_vec());
        Ok(result)
    }
}

impl UserDirs for Test<'_> {
    fn get_hop_home(&self, _path: &HomeType) -> HopEffect<PathBuf> {
        match &self.stub.get_hop_home {
            GetHopHomeStatus::Succeeded(path) => Ok(PathBuf::from(path)),
            GetHopHomeStatus::Failed(error) => {
                Err(io::Error::new(io::ErrorKind::Other, error.to_string()))
            }
        }
    }
}

impl SymLinks for Test<'_> {
    fn read_dir_links(&self, _dir_path: &Path) -> HopEffect<Vec<LinkPair>> {
        match &self.stub.read_dir_links {
            Ok(links) => Ok(links.to_vec()),
            Err(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
        }
    }

    fn write_link(&self, _symlink: &SymLink, _target: &Path) -> HopEffect<()> {
        match &self.stub.write_link {
            Some(error) => Err(io::Error::new(io::ErrorKind::Other, error.to_string())),
            None => Ok(()),
        }
    }

    fn link_exists(&self, _file_name: &Path) -> HopEffect<bool> {
        Ok(self.stub.link_exists)
    }

    fn delete_link(&self, _dir_path: &Path, link_pair: &LinkPair) -> HopEffect<()> {
        match &self.stub.delete_link {
            SymLinkDeleteStatus::Succeeded => Ok(()),
            SymLinkDeleteStatus::Failed => Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Failed to delete: {}", &link_pair),
            )),
        }
    }
}

impl Directories for Test<'_> {
    fn dir_exists(&self, _dir_path: &Path) -> HopEffect<bool> {
        Ok(self.stub.dir_exists)
    }
}

#[test]
fn list_links_success() {
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let output = Cell::new(vec![]);
    let stub = TestStub::with_read_links(&output, read_links);
    let program = TestStub::program(stub);

    match program.list_links() {
        Ok(entries) => {
            assert_eq!(&Vec::<String>::new(), &output.into_inner());
            assert_eq!(
                &vec![
                    LinkPair::new("myLink", "/my/path/to/link"),
                    LinkPair::new("myOtherLink", "/my/path/to/Otherlink")
                ],
                &entries
            )
        }
        Err(e) => panic!("{}: Expected an Ok but got err", e),
    }
}

#[test]
fn list_links_home_dir_failure() {
    let output = Cell::new(vec![]);
    let default = TestStub::new(&output);
    let stub = TestStub {
        get_hop_home: GetHopHomeStatus::Failed("Failed to get home dir".to_string()),
        ..default
    };
    let program = TestStub::program(stub);

    match program.list_links() {
        Err(e) => assert_eq!(e.to_string(), "Failed to get home dir"),
        Ok(_) => panic!("Expected an Err but got Ok"),
    }
}

#[test]
fn list_links_read_links_failure() {
    let output = Cell::new(vec![]);

    let default = TestStub::new(&output);
    let stub = TestStub {
        read_dir_links: Err("Failed to read links".to_string()),
        ..default
    };
    let program = TestStub::program(stub);

    match program.list_links() {
        Err(e) => assert_eq!(e.to_string(), "Failed to read links"),
        Ok(_) => panic!("Expected an Err but got Ok"),
    }
}

#[test]
fn list_links_read_links_no_result() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);
    let stub = TestStub::new(&output);
    let program = TestStub::program(stub);

    match program.list_links() {
        Ok(_) => {
            let output_vec = &output.into_inner();
            assert!(
                output_vec.is_empty(),
                "Expected output to be empty but got: {:?}",
                output_vec
            )
        }
        Err(e) => panic!("{}: Expected an Ok but got err", e),
    }
}

#[test]
fn tabulate_links_success() {
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let output = Cell::new(vec![]);
    let stub = TestStub::with_read_links(&output, read_links);
    let program = TestStub::program(stub);

    match program.tabulate_links() {
        Ok(entries) => {
            assert_eq!(
                &vec![
                    LinkPair::new("myLink", "/my/path/to/link"),
                    LinkPair::new("myOtherLink", "/my/path/to/Otherlink")
                ],
                &entries
            );
            assert_eq!(&Vec::<String>::new(), &output.into_inner())
        }
        Err(e) => panic!("{}: Expected an Ok but got err", e),
    }
}

#[test]
fn jump_target_success() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let stub = TestStub::with_read_links(&output, read_links);
    let program = TestStub::program(stub);

    match program.jump_target(Link::new("myOtherLink")) {
        Ok(link) => {
            assert_eq!(link, "/my/path/to/Otherlink".to_string());
            assert_eq!(&Vec::<String>::new(), &output.into_inner())
        }
        Err(e) => panic!("{}: Expected an Ok but got err", e),
    }
}

#[test]
fn jump_target_not_found() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let stub = TestStub::with_read_links(&output, read_links);
    let program = TestStub::program(stub);

    match program.jump_target(Link::new("bizarre")) {
        Ok(_) => panic!("Expected an Err but got Ok"),
        Err(e) => assert_eq!(e.to_string(), "Could not find link: bizarre".to_string()),
    }
}

#[test]
fn jump_target_without_links() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);

    let stub = TestStub::new(&output);
    let program = TestStub::program(stub);

    match program.jump_target(Link::new("myLink")) {
        Ok(_) => panic!("Expected Err but got Ok"),
        Err(e) => assert_eq!(e.to_string(), "Could not find link: myLink".to_string()),
    }
}

#[test]
fn mark_dir_success() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);

    let stub = TestStub::new(&output);
    let program = TestStub::program(stub);

    match program.mark_dir(&LinkPair::new("myLink", "/my/path/to/link")) {
        Ok(_) => assert_eq!(&Vec::<String>::new(), &output.into_inner()),
        Err(e) => panic!("{}: Expected an Ok but got err", e),
    }
}

#[test]
fn mark_dir_dir_does_not_exist() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);

    let default = TestStub::new(&output);
    let stub = TestStub {
        dir_exists: false,
        ..default
    };
    let program = TestStub::program(stub);

    match program.mark_dir(&LinkPair::new("myLink", "/my/path/to/link")) {
    Ok(_) => panic!("Expected an Err but got Ok"),
    Err(e) => assert_eq!("A directory named `/my/path/to/link` does not exist or you do not have permission to it.", e.to_string()),
  }
}

#[test]
fn mark_dir_link_exists() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);

    let default = TestStub::new(&output);
    let stub = TestStub {
        link_exists: true,
        ..default
    };
    let program = TestStub::program(stub);

    match program.mark_dir(&LinkPair::new("myLink", "/my/path/to/link")) {
        Ok(_) => panic!("Expected an Err but got Ok"),
        Err(e) => assert_eq!(
            "A link named `myLink` already exists. Aborting mark creation.",
            e.to_string()
        ),
    }
}

#[test]
fn mark_dir_write_link_failed() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);

    //Is there a more succinct, yet readable way to do this?
    let default = TestStub::new(&output);
    let stub = TestStub {
        write_link: Some("Could not create link because this is a test".to_string()),
        ..default
    };
    let program = TestStub::program(stub);

    match program.mark_dir(&LinkPair::new("myLink", "/my/path/to/link")) {
        Ok(_) => panic!("Expected an Err but got Ok"),
        Err(e) => assert_eq!(
            "Could not create link because this is a test",
            e.to_string()
        ),
    }
}

#[test]
fn delete_link_success() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);
    let input: Cell<Vec<String>> = Cell::new(vec!["Y".to_string()]);
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let stub = TestStub::with_read_links_and_std_in(&output, read_links, &input);
    let program = TestStub::program(stub);

    match program.delete_link(&Link::new("myLink")) {
        Ok(result) => {
            let expected = vec![
                "Are you sure you want to delete myLink which links to /my/path/to/link ?"
                    .to_string(),
            ];

            assert_eq!(&expected, &output.into_inner());
            assert_eq!(
                result,
                DeleteStatus::DeleteSucceeded(LinkPair::new("myLink", "/my/path/to/link"))
            );
            assert_eq!(&Vec::<String>::new(), &input.into_inner());
        }
        Err(e) => panic!("Expected an Ok but got Err: {}", e),
    }
}

#[test]
fn delete_link_aborted() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);
    let input: Cell<Vec<String>> = Cell::new(vec!["N".to_string()]);
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let stub = TestStub::with_read_links_and_std_in(&output, read_links, &input);
    let program = TestStub::program(stub);

    match program.delete_link(&Link::new("myLink")) {
        Ok(result) => {
            let expected = vec![
                "Are you sure you want to delete myLink which links to /my/path/to/link ?"
                    .to_string(),
            ];

            assert_eq!(&expected, &output.into_inner());
            assert_eq!(result, DeleteStatus::DeleteAborted);
            assert_eq!(&Vec::<String>::new(), &input.into_inner());
        }
        Err(e) => panic!("Expected an Ok but got Err: {}", e),
    }
}

#[test]
fn delete_link_link_not_found() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);
    let input: Cell<Vec<String>> = Cell::new(vec!["N".to_string()]);
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let stub = TestStub::with_read_links_and_std_in(&output, read_links, &input);
    let program = TestStub::program(stub);

    match program.delete_link(&Link::new("notALink")) {
        Ok(_) => panic!("Expected an Err but got Ok"),
        Err(e) => {
            assert_eq!(&Vec::<String>::new(), &output.into_inner());
            assert_eq!(
                "Could not find link named:`notALink` for deletion".to_string(),
                e.to_string()
            );
            assert_eq!(&vec!["N"], &input.into_inner());
        }
    }
}

#[test]
fn delete_link_failed() {
    let output: Cell<Vec<String>> = Cell::new(vec![]);
    let input: Cell<Vec<String>> = Cell::new(vec!["Y".to_string()]);
    let read_links = vec![
        LinkPair::new("myLink", "/my/path/to/link"),
        LinkPair::new("myOtherLink", "/my/path/to/Otherlink"),
    ];

    let default = TestStub::with_read_links_and_std_in(&output, read_links, &input);
    let stub = TestStub {
        delete_link: SymLinkDeleteStatus::Failed,
        ..default
    };

    let program = TestStub::program(stub);
    match program.delete_link(&Link::new("myLink")) {
        Ok(_) => panic!("Expected Err but got Ok"),
        Err(e) => {
            let expected = vec![
                "Are you sure you want to delete myLink which links to /my/path/to/link ?"
                    .to_string(),
            ];

            assert_eq!(
                "Failed to delete: myLink -> /my/path/to/link",
                e.to_string()
            );
            assert_eq!(&expected, &output.into_inner());
            assert_eq!(&Vec::<String>::new(), &input.into_inner())
        }
    }
}
