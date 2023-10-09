use std::fs::File;
use std::io::Error;
use flate2::read::GzDecoder;
use tar::Archive;

pub fn tar_handler() {
    let a = unpack_tar_gz();

    match a {
        Ok(res) => println!("unpack ok: {:?}", res),
        Err(error) => println!("unpack err: {}", error)
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