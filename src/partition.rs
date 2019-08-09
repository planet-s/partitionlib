use super::Result;
//use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;
use mbr::partition::read_partitions;
use uuid::Uuid;
use gpt;

/// A union of the MBR and GPT partition entry
#[derive(Clone, Debug)]
pub struct Partition {
    /// The starting logical block number
    pub start_lba : u64,
    /// The size of the partition in sectors
    pub size : u64,
    pub flags : Option<u64>,
    pub name : Option<String>,
    pub uuid : Option<Uuid>
}


pub fn is_mbr(path: &Path) -> bool {
    let cfg = gpt::GptConfig::new().writable(false);
    let is_mbr = match cfg.open(path) {
        Ok(_e) => false,
        Err(x) => {
                    println!("[is_mbr] Error {:?} ", x);
                    true
        }
    };

    is_mbr
    //println!("Disk header: {:#?}", disk.primary_header());
    //println!("Partition layout: {:#?}", disk.partitions());

}

pub fn get_partitions(path: &Path) -> Result<Vec<Partition>> {
    let mut res = Vec::new();
    if is_mbr(path) {
        let partitions = read_partitions(path.to_path_buf())?;
        for part in &partitions {
            let t = Partition {
                start_lba : part.p_lba as u64,
                size : part.p_size as u64,
                flags : None,
                name : None,
                uuid : None
            };

            res.push(t);
        }
    }
    else {
        let cfg = gpt::GptConfig::new().writable(false);
        let disk = cfg.open(path).expect("failed to open disk");
        for (_id, part) in disk.partitions().iter() {
            let t = Partition {
                start_lba : part.first_lba,
                size : part.last_lba - part.first_lba + 1,
                flags : Some(part.flags),
                name : Some(part.name.clone()),
                uuid : Some(part.part_guid)
            };
            res.push(t);
        }

    }

    Ok(res)
}

impl Partition {
    pub fn to_offset(&self) -> u64 {
        self.start_lba * 512
    }
}
