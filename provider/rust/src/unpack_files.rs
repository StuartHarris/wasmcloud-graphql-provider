use flate2::read::GzDecoder;
use tar::Archive;
use temp_dir::TempDir;

pub fn unpack(temp_dir: &TempDir) -> Result<(), std::io::Error> {
    println!(
        "unpacking node files into {}",
        temp_dir.path().to_string_lossy()
    );
    let tar_gz = include_bytes!("../../node/build/build.tgz");
    let tar = GzDecoder::new(&tar_gz[..]);
    let mut archive = Archive::new(tar);
    archive.unpack(temp_dir.path())?;

    Ok(())
}
