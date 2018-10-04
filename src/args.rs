use clap::{App, Arg, ArgMatches, AppSettings};
use directories::ProjectDirs;
use std::path::PathBuf;
use std::process;

fn is_int(s: String) -> Result<(), String> {
    match s.parse::<i64>() {
        Ok(_) => Ok(()),
        Err(_e) => Err(format!("invalid integer {}", s)),
    }
}

pub fn get_app() -> App<'static,'static> {

    App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(Arg::with_name("store")
                .long("store")
                .value_name("FILE")
                .conflicts_with_all(&["store_name"])
                .help("Use a non-default store file")
                .takes_value(true))
        .arg(Arg::with_name("store_name")
                .long("store_name")
                .value_name("FILE")
                .conflicts_with_all(&["store"])
                .help("Use a non-default filename for the store file in the default store directory")
                .takes_value(true))
        .arg(Arg::with_name("purge")
                .short("P")
                .long("purge")
                .help("Purge directories that no longer exist from the database"))
        .arg(Arg::with_name("increase")
                .short("i")
                .long("increase")
                .help("Increase the weight of a directory by WEIGHT")
                .conflicts_with_all(&["add", "decrease"])
                .requires("directory")
                .value_name("WEIGHT")
                .takes_value(true))
        .arg(Arg::with_name("add")
                .short("a")
                .long("add")
                .conflicts_with_all(&["increase", "decrease"])
                .requires("directory")
                .help("Add a visit to a DIRECTORY to the store"))
        .arg(Arg::with_name("decrease")
                .short("d")
                .long("decrease")
                .conflicts_with_all(&["increase", "add"])
                .requires("directory")
                .help("Decrease the weight of a directory by WEIGHT")
                .value_name("WEIGHT")
                .takes_value(true))
        .arg(Arg::with_name("truncate")
                .short("T")
                .long("truncate")
                .help("Truncate the stored directories to only the top N")
                .value_name("N")
                .validator(is_int)
                .takes_value(true))
        .arg(Arg::with_name("sorted")
                .long("sorted")
                .group("lists")
                .help("Print the stored directories in order of highest to lowest score"))
        .arg(Arg::with_name("sort_method")
                .long("sort_method")
                .help("The method to sort by most used")
                .takes_value(true)
                .possible_values(&["frecent", "frequent", "recent"])
                .default_value("frecent"))
        .arg(Arg::with_name("limit")
                .long("limit")
                .short("l")
                .takes_value(true)
                .requires("lists")
                .help("Limit the number of results printed --sorted"))
        .arg(Arg::with_name("stat")
                .short("s")
                .group("lists")
                .long("stat")
                .help("Print statistics about the stored directories"))
        .arg(Arg::with_name("directory")
                .index(1)
                .help("The directory to jump to"))
}

pub fn get_store_path(matches: &ArgMatches) -> PathBuf {
  match (matches.value_of("store"), matches.value_of("store_name")) {
    (Some(dir), None) => PathBuf::from(dir),
    (None, file) => default_store(file), 
    _ => unreachable!(),
  }
}

pub fn default_store(filename: Option<&str>) -> PathBuf {
  let store_dir = match ProjectDirs::from("", "", env!("CARGO_PKG_NAME")) {
    Some(dir) => dir.data_dir().to_path_buf(),
    None => {
      eprintln!("Failed to detect default data directory");
      process::exit(1);
    }
  };

  let mut store_file = store_dir.clone();
  let default = format!("{}.json", env!("CARGO_PKG_NAME"));
  let filename = filename.unwrap_or(&default);
  store_file.push(filename);

  return store_file.to_path_buf(); 
}

#[cfg(test)]
mod tests {
  use super::*;
  use spectral::prelude::*;

  #[test]
  fn get_store_path_full() {
    let arg_vec = vec!["topd", "--store", "/test/path"];
    let matches = get_app().get_matches_from_safe(arg_vec).unwrap();

    let store_path = get_store_path(&matches);

    assert_that!(store_path).is_equal_to(PathBuf::from("/test/path"));
  }

  #[test]
  fn get_store_path_file() {
    let arg_vec = vec!["topd", "--store_name", "test.path"];
    let matches = get_app().get_matches_from_safe(arg_vec).unwrap();

    let store_path = get_store_path(&matches);

    assert_that!(store_path.to_str().unwrap()).ends_with("test.path");
  }
}
