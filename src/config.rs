use async_std::{fs, path};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, os::unix::fs::PermissionsExt};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Config {
    #[serde(default = "WorkMode::default")]
    work_mode: WorkMode,
    task_list: HashMap<String, ConfigItem>,
}

impl Config {
    pub(crate) async fn link(&self, src: &str, dst: &str) -> async_std::io::Result<()> {
        match self.work_mode {
            WorkMode::HardLink => {
                async_std::fs::hard_link(src, dst).await?;
            }
            WorkMode::SymLink => {
                async_std::os::unix::fs::symlink(src, dst).await?;
            }
        }
        Ok(())
    }

    pub(crate) async fn do_task(&self, item: &ConfigItem) -> async_std::io::Result<()> {
        let permission = item.get_permission();
        let ConfigItem { dst, src, .. } = item;
        create_dir_all(&dst).await?;
        self.link(&src, &dst).await?;
        if let Some(permission) = permission {
            async_std::fs::set_permissions(dst, permission).await?;
        }
        Ok(())
    }

    pub(crate) async fn do_all(&self) -> async_std::io::Result<()> {
        for t in self.task_list.values() {
            self.do_task(t).await?;
        }
        Ok(())
    }
}

pub(crate) async fn create_dir_all<P: AsRef<path::Path>>(p: P) -> async_std::io::Result<()> {
    let mut path_all = path::PathBuf::from(p.as_ref());
    path_all.pop();
    if let Err(e) = fs::create_dir_all(path_all).await {
        if e.kind() == async_std::io::ErrorKind::AlreadyExists {
            return Ok(());
        } else {
            return Err(e);
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub(crate) enum WorkMode {
    HardLink,
    SymLink,
}

impl Default for WorkMode {
    fn default() -> WorkMode {
        WorkMode::HardLink
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct ConfigItem {
    dst: String,
    src: String,
    permission: Option<u32>,
}

impl ConfigItem {
    pub(crate) fn get_permission(&self) -> Option<fs::Permissions> {
        self.permission.map(fs::Permissions::from_mode)
    }
}
