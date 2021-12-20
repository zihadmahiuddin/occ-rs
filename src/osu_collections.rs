use std::fmt::{Display, Formatter};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::io::Read;
use std::path::PathBuf;

use crate::osu_reader::OsuReader;
use crate::osu_writer::OsuWriter;

// static VERSION: i32 = 20211122;

pub struct OsuCollections {
  pub version: i32,
  pub entries: Vec<OsuCollection>,
}

impl OsuCollections {
  // pub fn new() -> OsuCollections {
  //   return OsuCollections {
  //     version: VERSION,
  //     entries: vec![]
  //   }
  // }
  //
  // pub fn print_all(&self) {
  //   for i in 0..self.entries.len() {
  //     let collection = self.entries.get(i).unwrap();
  //     println!("{}", collection.to_string());
  //   }
  //   println!();
  // }

  pub fn read(path: &PathBuf) -> OsuCollections {
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    let mut osu_reader = OsuReader::new(buffer);

    let version = osu_reader.read_i32();
    let collection_count = osu_reader.read_i32();

    let mut collections = Vec::new();

    for _i in 0..collection_count {
      let name = osu_reader.read_string();
      let map_count = osu_reader.read_i32();

      let mut map_hashes = Vec::new();

      for _i in 0..map_count {
        let map_hash = osu_reader.read_string();
        map_hashes.push(map_hash);
      }

      collections.push(
        OsuCollection {
          name,
          map_count,
          map_hashes,
        }
      )
    }

    return OsuCollections {
      version,
      entries: collections,
    };
  }

  pub fn add_collection(&mut self, collection: &OsuCollection) {
    self.entries.push(OsuCollection {
      name: collection.name.clone(),
      map_count: collection.map_count,
      map_hashes: collection.map_hashes.clone(),
    });
  }

  pub fn write(&self, path: &PathBuf) {
    let mut osu_writer = OsuWriter::new();

    osu_writer.write_i32(self.version);
    osu_writer.write_i32(self.entries.len() as i32);

    for collection in self.entries.iter() {
      osu_writer.write_string(&collection.name);
      osu_writer.write_i32(collection.map_count);

      for map_hash in collection.map_hashes.iter() {
        osu_writer.write_string(&map_hash);
      }
    }

    let file = OpenOptions::new()
      .create(true)
      .write(true)
      .open(path)
      .unwrap();
    let mut writer = BufWriter::new(file);

    writer.write(&*osu_writer.buffer).unwrap();
  }
}

pub struct OsuCollection {
  pub name: String,
  pub map_count: i32,
  pub map_hashes: Vec<String>,
}

impl Display for OsuCollection {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    return write!(f, "Name: {}, Map Count: {}, Map Hashes: {:?}", self.name, self.map_count, self.map_hashes);
  }
}

impl OsuCollection {
  pub fn new(
    name: String,
  ) -> OsuCollection {
    return OsuCollection {
      name,
      map_count: 0,
      map_hashes: vec![],
    };
  }

  pub fn add_map(&mut self, map_hash: &String) {
    self.map_hashes.push(map_hash.clone());
    self.map_count += 1;
  }
}
