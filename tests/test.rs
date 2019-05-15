extern crate partitionlib;

use partitionlib::{get_partitions, is_mbr};

#[test]
fn test_gpt() {
    let res = is_mbr(std::path::Path::new("./resources/disk.img"));
    assert!(!res);
}

#[test]
fn test_mbr() {
    let res = is_mbr(std::path::Path::new("./resources/disk_mbr.img"));
    assert!(res);
}
