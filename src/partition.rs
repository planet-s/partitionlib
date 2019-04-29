use super::Result;
use std::io::{Read, Write, Seek, SeekFrom};

pub struct partition {
    pub start_lba : u64,
    pub size : u64
}


pub fn is_mbr<D: Read + Write + Seek>(disk : &mut D) -> Result<bool> {
    disk.seek(SeekFrom::Start(0))?;
    Ok(true)
}
