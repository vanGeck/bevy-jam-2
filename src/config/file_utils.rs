use ron::Options;
use serde::Serialize;
use std::fs;
use std::path::PathBuf;

/// Creates and returns a PathBuf to the assets/config/default/ directory.
/// If the directory does not exist yet, creates it.
///
/// This is where default configuration files are stored.
pub fn get_config_default_dir() -> PathBuf {
    create_if_missing(PathBuf::new().join("assets/config/default/"))
}

/// Creates and returns a PathBuf to the assets/config/default/ directory.
/// If the directory does not exist yet, creates it.
///
/// This is where override configuration files for individual devs are stored.
pub fn get_config_dev_dir() -> PathBuf {
    create_if_missing(PathBuf::new().join("assets/config/dev/"))
}

/// Creates the given directory if it does not exist yet.
fn create_if_missing(path: PathBuf) -> PathBuf {
    fs::create_dir_all(&path).unwrap_or_else(|err| {
        panic!(
            "Failed to create directory {:?} because error {:?}",
            &path, err
        )
    });
    path
}

#[allow(dead_code)]
pub fn serialise_ron<S>(serialize: S) -> Result<String, ron::Error>
where
    S: Serialize,
{
    let pretty_config = ron::ser::PrettyConfig::default()
        // .indentor("\t".to_string())
        .new_line("\n".to_string());
    let mut buf = Vec::new();
    let mut ron_serializer =
        ron::ser::Serializer::with_options(&mut buf, Some(pretty_config), Options::default())?;
    serialize.serialize(&mut ron_serializer)?;
    Ok(String::from_utf8(buf).unwrap())
}
