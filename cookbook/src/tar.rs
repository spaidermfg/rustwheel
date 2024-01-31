use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs::File;
use std::io::Error;
use tar::{Archive, Builder};

pub fn tar_handler() {
    let a = unpack_tar_gz();

    match a {
        Ok(res) => println!("unpack ok: {:?}", res),
        Err(error) => println!("unpack err: {}", error),
    }

    let pack = pack_tar_gz();
    match pack {
        Ok(()) => println!("pack success"),
        Err(error) => println!("pack err: {}", error),
    }
}

fn unpack_tar_gz() -> Result<&'static str, Error> {
    let tar_path = "archive.tar.gz";
    let file = File::open(tar_path)?;

    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive.unpack(".")?;

    Ok("解压成功")
}

fn pack_tar_gz() -> Result<(), Error> {
    let tar_gz = File::create("archive.tar.gz")?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut ar = Builder::new(enc);
    ar.append_dir_all(".", "/var/logs")?;

    Ok(())
}
