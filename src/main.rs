use std::env;
use std::fs::{copy, File, read_dir};
use std::io::{Read};
use std::path::PathBuf;
use std::process::exit;

use chrono::Local;
use md5::{Digest, Md5};
use structopt::StructOpt;
use zip::ZipArchive;

use crate::osu_collections::OsuCollection;
use crate::osu_collections::OsuCollections;

mod osu_reader;
mod osu_writer;
mod osu_collections;

/// A command-line utility for creating osu! collections from beatmap folders.
#[derive(StructOpt)]
#[structopt(name = "osu-collection-creator", rename_all = "kebab-case")]
struct Cli {
  /// Path where beatmap (.osz) files are stored
  #[structopt(parse(from_os_str))]
  beatmap_path: PathBuf,
  /// The name of the new collection
  #[structopt()]
  collection_name: String,
  /// Path where osu! is installed
  #[structopt(parse(from_os_str))]
  osu_path: Option<PathBuf>,
}

fn read_beatmap_hashes(beatmapsets_path: &PathBuf) -> Vec<String> {
  let beatmapsets = read_dir(beatmapsets_path).unwrap();
  let mut beatmap_hashes = Vec::new();
  for beatmapset in beatmapsets {
    let file = File::open(beatmapset.unwrap().path()).unwrap();
    let mut archive = ZipArchive::new(file).unwrap();
    for i in 0..archive.len() {
      let mut archived_file = archive.by_index(i).unwrap();
      if archived_file.name().ends_with(".osu") {
        let mut contents = String::new();
        archived_file.read_to_string(&mut contents).unwrap();
        let beatmap_hash = format!("{:x}", Md5::digest(contents.as_bytes()));
        beatmap_hashes.push(beatmap_hash);
      }
    }
  }
  return beatmap_hashes;
}

fn get_default_osu_path() -> PathBuf {
  return PathBuf::from(
    env::var("LOCALAPPDATA").expect("LOCALAPPDATA environment variable not found")
  ).join("osu!");
}

fn get_backup_file_name() -> String {
  let now = Local::now();
  let year = now.format("%Y").to_string();
  let month = now.format("%m").to_string();
  let day = now.format("%d").to_string();
  let hour = now.format("%H").to_string();
  let minute = now.format("%M").to_string();
  let second = now.format("%S").to_string();
  return format!("collection_backup_{}-{}-{}-{}-{}-{}.db", year, month, day, hour, minute, second);
}

fn main() {
  if !cfg!(windows) {
    println!("This program is only supported on Windows for now.");
    exit(1);
  }

  let args: Cli = Cli::from_args();
  let osu_path = args.osu_path.unwrap_or(get_default_osu_path());
  if !osu_path.exists() {
    println!("osu! path does not exist: {:?}", osu_path);
    exit(1);
  }

  let collection_path = osu_path.join("collection.db");
  let backup_collection_path = osu_path.join(get_backup_file_name());
  copy(&collection_path, &backup_collection_path).expect("Failed to backup collection file");

  let mut collections = OsuCollections::read(&collection_path);
  let beatmapset_hashes = read_beatmap_hashes(&args.beatmap_path);

  let mut new_collection = OsuCollection::new(args.collection_name);
  for map_hash in beatmapset_hashes {
    new_collection.add_map(&map_hash);
  }
  println!("Collection created, saving to file.");
  collections.add_collection(&new_collection);
  collections.write(&collection_path);
  println!("Collection saved.");
}
