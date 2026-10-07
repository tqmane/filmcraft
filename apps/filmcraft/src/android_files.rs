//! Keep the existing random-access media services and publish completed saves through SAF.
use filmcraft_engine::{FsServices, Services};
pub struct AndroidFiles;
impl Services for AndroidFiles {
    fn finish_export(&self, path: &str) -> std::io::Result<()> {
        rfd::publish_file(std::path::Path::new(path)).map_err(std::io::Error::other)
    }
    fn read_file(&self, path: &str) -> std::io::Result<Vec<u8>> {
        FsServices.read_file(path)
    }
    fn write_file(&self, path: &str, data: &[u8]) -> std::io::Result<()> {
        FsServices.write_file(path, data)?;
        rfd::publish_file(std::path::Path::new(path)).map_err(std::io::Error::other)
    }
    fn file_size(&self, path: &str) -> std::io::Result<u64> {
        FsServices.file_size(path)
    }
    fn read_range(&self, path: &str, offset: u64, len: usize) -> std::io::Result<Vec<u8>> {
        FsServices.read_range(path, offset, len)
    }
    fn reader(&self, path: &str) -> Option<std::io::Result<filmcraft_media::SharedReader>> {
        FsServices.reader(path)
    }
    fn list_dir(&self, path: &str) -> Option<std::io::Result<Vec<String>>> {
        FsServices.list_dir(path)
    }
    fn file_loader(&self) -> Option<filmcraft_media::sequence::FrameLoader> {
        FsServices.file_loader()
    }
    fn list_entries(&self, path: &str) -> Option<std::io::Result<Vec<filmcraft_engine::media_browser::DirEntry>>> {
        FsServices.list_entries(path)
    }
    fn volumes(&self) -> Vec<filmcraft_engine::media_browser::Volume> {
        FsServices.volumes()
    }
    fn home_dir(&self) -> Option<String> {
        FsServices.home_dir()
    }
}
