use std::path::{PathBuf, Path};
use std::{io, ffi, fs};
use std::io::Read;
use image::{ImageBuffer, GenericImage, ImageError, DynamicImage};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error")]
    Io(#[cause] io::Error),
    #[fail(display = "Image error")]
    FailedToLoadImage(#[cause] ImageError),
    #[fail(display = "Failed to read CString from file that contains 0")]
    FileContainsNil,
    #[fail(display = "Failed to get executable path")]
    FailedToGetExePath,
}

impl From<io::Error> for Error {
    fn from(other: io::Error) -> Self {
        Error::Io(other)
    }
}

impl From<ImageError> for Error {
    fn from(other: ImageError) -> Self {
        Error::FailedToLoadImage(other)
    }
}

pub struct Resources {
    root_path: PathBuf,
}

impl Resources {
    pub fn from_relative_exe_path(rel_path: &Path) -> Result<Self, Error> {
        let exe_file_name = std::env::current_exe()
            .map_err(|_| Error::FailedToGetExePath)?;
        let exe_path = exe_file_name.parent()
            .ok_or(Error::FailedToGetExePath)?;
        Ok(Resources {
            root_path: exe_path.join(rel_path)
        })
    }

    pub fn load_cstring(&self, resource_name: &str) -> Result<ffi::CString, Error> {
        let buffer = self.load_bytes_from_file(resource_name)?;
        if buffer.iter().any(|i| *i == 0) {
            return Err(Error::FileContainsNil);
        }
        Ok(unsafe {ffi::CString::from_vec_unchecked(buffer)})
    }

    pub fn load_image(&self, resource_name: &str) -> Result<DynamicImage, Error> {
        Ok(image::open(
            resource_name_to_path(&self.root_path, resource_name))?)
    }

    pub fn load_bytes_from_file(&self, resource_name: &str) -> Result<Vec<u8>, Error> {
        let mut file = fs::File::open(
            resource_name_to_path(&self.root_path, resource_name)
        )?;
        let mut buffer: Vec<u8> = Vec::with_capacity(
            file.metadata()?.len() as usize + 1
        );
        file.read_to_end(&mut buffer)?;
        Ok(buffer)
    }
}

fn resource_name_to_path(root_dir: &Path, location: &str) -> PathBuf {
    let mut path: PathBuf = root_dir.into();
    for part in location.split('/') {
        path = path.join(part);
    }
    path
}

