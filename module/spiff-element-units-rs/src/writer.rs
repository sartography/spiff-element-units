use serde;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::PathBuf;

//
// write `contents` to file at `path`.
//
pub fn write_string(path: &PathBuf, contents: &str) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

//
// serialize `value` and write the results to a `String`.
//
pub fn write_to_string<T>(value: &T) -> serde_json::Result<String>
where
    T: ?Sized + serde::Serialize,
{
    serde_json::to_string(value)
}

//
// serialize `value` and write the results to a file at `path`.
//
pub fn write<T>(path: &PathBuf, value: &T) -> Result<(), Box<dyn Error>>
where
    T: ?Sized + serde::Serialize,
{
    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer_pretty(&mut writer, value)?;
    writer.flush()?;

    Ok(())
}
