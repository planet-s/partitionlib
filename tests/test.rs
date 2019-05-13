extern crate partitionlib;

use partitionlib::{get_partitions, is_mbr};

#[test]
fn test1() {
    let res = is_mbr(std::path::Path::new("./resources/disk.img"));
    assert!(!res);
}
