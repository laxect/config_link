use async_std::fs::Permissions;
use serde::{Deserialize, Serialize};
use std::os::unix::fs::PermissionsExt;

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Config {
    work_mode: WorkMode,
    task_list: Vec<ConfigItem>,
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

    pub(crate) async fn do_task(&self, item: ConfigItem) -> async_std::io::Result<()> {
        let permission = item.get_permission();
        let ConfigItem { dst, src, .. } = item;
        self.link(&src, &dst).await?;
        if let Some(permission) = permission {
            async_std::fs::set_permissions(dst, permission).await?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Deserialize, Serialize)]
pub(crate) enum WorkMode {
    HardLink,
    SymLink,
}

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct ConfigItem {
    dst: String,
    src: String,
    permission: Option<u32>,
}

impl ConfigItem {
    pub(crate) fn new<T: Into<String>>(dst: T, src: T, permission: Option<u32>) -> ConfigItem {
        ConfigItem {
            dst: dst.into(),
            src: src.into(),
            permission,
        }
    }

    pub(crate) fn get_permission(&self) -> Option<Permissions> {
        self.permission.map(Permissions::from_mode)
    }
}
