#![feature(old_io)]
#![feature(old_path)]

extern crate psutil;

use std::old_io::File;
use std::old_io::TempDir;

use psutil::pidfile::{read_pidfile,write_pidfile};

#[test]
fn read_write_pidfile() {
    // This will be removed automatically when dropped
    let tempdir = TempDir::new("psutil-tests").unwrap();
    let pidfile = tempdir.path().join("read_write_pidfile.pid");

    // Write the pidfile to the temporary directory
    write_pidfile(&pidfile).unwrap();

    // Read the pidfile and check it against `getpid()`
    assert_eq!(read_pidfile(&pidfile).unwrap(), psutil::getpid());
}

#[test]
fn read_invalid_pidfile() {
    let tempdir = TempDir::new("psutil-tests").unwrap();
    let pidfile = tempdir.path().join("read_invalid_pidfile.pid");

    write!(&mut File::create(&pidfile).unwrap(), "{}", "beans").unwrap();
    assert!(read_pidfile(&pidfile).is_err());
}
