use alloc::sync::Arc;
use core::ffi::{c_char, c_int};

use axerrno::{LinuxError, LinuxResult};
use axfs::fops::OpenOptions;
use axio::{PollState, SeekFrom};
use axsync::Mutex;
use alloc::string::{String, ToString};

use super::fd_ops::{get_file_like, FileLike};
use crate::{ctypes, utils::char_ptr_to_str};

pub struct Dir {
    pub path: String,
}

impl Dir {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_string(),
        }
    }

    pub fn add_to_fd_table(self) -> LinuxResult<c_int> {
        super::fd_ops::add_file_like(Arc::new(self))
    }

    pub fn from_fd(fd: c_int) -> LinuxResult<Arc<Self>> {
        let f = super::fd_ops::get_file_like(fd)?;
        f.into_any()
            .downcast::<Self>()
            .map_err(|_| LinuxError::EINVAL)
    }
}

impl FileLike for Dir {
    fn read(&self, buf: &mut [u8]) -> LinuxResult<usize> {
        Err(LinuxError::EISDIR)
    }

    fn write(&self, buf: &[u8]) -> LinuxResult<usize> {
        Err(LinuxError::EISDIR)
    }

    fn seek(&self, pos: SeekFrom) -> LinuxResult<u64> {
        Err(LinuxError::EISDIR)
    }

    fn path(&self) -> String {
        self.path.to_string().clone()
    }

    fn stat(&self) -> LinuxResult<ctypes::stat> {
        Ok(ctypes::stat {
            st_ino: 0,
            st_nlink: 1,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            st_size: 0,
            st_blocks: 0,
            st_blksize: 0,
            ..Default::default()
        })
    }

    fn into_any(self: Arc<Self>) -> Arc<dyn core::any::Any + Send + Sync> {
        self
    }

    fn poll(&self) -> LinuxResult<PollState> {
        Ok(PollState {
            readable: true,
            writable: true,
        })
    }

    fn set_nonblocking(&self, _nonblocking: bool) -> LinuxResult {
        Ok(())
    }
}
