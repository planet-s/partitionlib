extern crate partitionlib;

use partitionlib::{get_partitions_from_file, LogicalBlockSize};

#[test]
fn test_gpt() {
    assert!(dbg!(
        get_partitions_from_file("./resources/disk.img", LogicalBlockSize::Lb512)
            .unwrap()
            .unwrap()
    )
    .kind
    .is_gpt());
}

#[test]
fn test_mbr() {
    assert!(
        get_partitions_from_file("./resources/disk_mbr.img", LogicalBlockSize::Lb512)
            .unwrap()
            .unwrap()
            .kind
            .is_mbr()
    );
}
