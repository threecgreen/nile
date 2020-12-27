use std::error::Error;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

fn write_const(writer: &mut BufWriter<&File>, name: &str, value: &str) -> io::Result<()> {
    writeln!(writer, "export const {} = \"{}\";", name, value)
}

fn get_git_short_sha() -> Result<String, Box<dyn Error>> {
    Ok(String::from_utf8(
        std::process::Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
            .output()
            .expect("git rev-parse result")
            .stdout,
    )?
    .trim()
    .to_owned())
}

fn write_constants() -> Result<(), Box<dyn Error>> {
    assert!(Path::new(env!("CARGO_MANIFEST_DIR")).exists());
    assert!(Path::new(env!("CARGO_MANIFEST_DIR")).join("web").exists());
    assert!(Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("web")
        .join("generated")
        .exists());
    let const_file = File::create(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("web")
            .join("generated")
            .join("constants.ts"),
    )?;
    let mut const_writer = BufWriter::new(&const_file);
    let version = env!("CARGO_PKG_VERSION");
    let git_sha = get_git_short_sha()?;
    write_const(&mut const_writer, "VERSION", &version)?;
    Ok(write_const(&mut const_writer, "GIT_SHA", &git_sha)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    write_constants()?;
    Ok(())
}
