extern crate mbr;
extern crate gpt;
extern crate uuid;

pub type Result<T> = std::io::Result<T>;
pub mod partition;
#[cfg(test)]
mod test;
