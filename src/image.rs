pub mod guest_image;
pub mod host_image;

use crate::mount::{FileType, Mount, MountFlags};
use anyhow::{Context, Result};
use guest_image::GuestImage;
use host_image::HostImage;
use std::path::{Path, PathBuf};

pub trait ContainerImage {
    fn mount(&self) -> Result<()>;
    fn name(&self) -> String;
    fn root_path(&self) -> PathBuf;
    fn need_userns(&self) -> bool;
}

impl ContainerImage for GuestImage {
    fn mount(&self) -> Result<()> {
        Mount::<_, &str>::new(&self.container_path(), FileType::Dir)
            .src(&self.container_path())
            .flags(MountFlags::MS_BIND)
            .mount()
            .with_context(|| {
                format!("Failed to bind mount '{}'", self.container_path().display())
            })?;

        Ok(())
    }

    fn name(&self) -> String {
        self.container_name()
    }

    fn root_path(&self) -> PathBuf {
        self.image_root()
    }

    fn need_userns(&self) -> bool {
        true
    }
}

impl ContainerImage for HostImage {
    fn mount(&self) -> Result<()> {
        Mount::new(&self.container_path(), FileType::Dir)
            .flags(MountFlags::MS_PRIVATE)
            .flags(MountFlags::MS_REC)
            .data(format!(
                "lowerdir=/,upperdir={},workdir={}",
                &self.ovr_upperdir.display(),
                &self.ovr_workdir.display()
            ))
            .mount()
    }

    fn name(&self) -> String {
        self.container_name()
    }

    fn root_path(&self) -> PathBuf {
        self.container_path()
    }

    fn need_userns(&self) -> bool {
        false
    }
}
