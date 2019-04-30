use super::Result;
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use mbr::partition::read_partitions;

pub struct partition {
    pub start_lba : u64,
    pub size : u64,
    pub flags : Option<u64>,
    pub name : Option<String>,
}


pub fn is_mbr(path: &Path) -> bool {
    let cfg = gpt::GptConfig::new().writable(false);
    let is_mbr = match cfg.open(path) {
        Ok(e) => false,
        Err(x) => true
    };

    is_mbr
    //println!("Disk header: {:#?}", disk.primary_header());
    //println!("Partition layout: {:#?}", disk.partitions());

}

pub fn get_partitions(path: &Path) -> Result<Vec<partition>> {
    let mut res = Vec::new();
    if is_mbr(path) {
        let partitions = read_partitions(path.to_path_buf())?;
        for part in &partitions {
            
        }
    }

    Ok(res)
}
