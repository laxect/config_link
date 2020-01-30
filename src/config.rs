use crate::error;
use async_std::{fs, path};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, os::unix::fs::PermissionsExt};

#[derive(Clone, Deserialize, Serialize)]
pub(crate) struct Config {
    #[serde(default = "WorkMode::default")]
    work_mode: WorkMode,
    task_list: HashMap<String, ConfigItem>,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            work_mode: WorkMode::default(),
            task_list: HashMap::new(),
        }
    }

    pub(crate) async fn do_task(&self, item: &ConfigItem) -> error::Result<()> {
        let permission = item.get_permission();
        let ConfigItem { dst, .. } = item;
        create_dir_all(&dst).await?;
        item.link(self.work_mode).await?;
        if let Some(permission) = permission {
            async_std::fs::set_permissions(dst, permission).await?;
        }
        Ok(())
    }

    pub(crate) fn fix_home_dir(&mut self) -> error::Result<()> {
        for t in self.task_list.values_mut() {
            t.fix_home_dir()?;
        }
        Ok(())
    }

    pub(crate) async fn do_all(&mut self) -> error::Result<()> {
        self.fix_home_dir()?;
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

fn fix_home_dir(p: &mut String) -> Result<(), env::VarError> {
    let home_dir = env::var("HOME")?;
    if p.starts_with('~') {
        p.replace_range(..1, &home_dir);
    }
    Ok(())
}

impl ConfigItem {
    pub(crate) fn get_permission(&self) -> Option<fs::Permissions> {
        self.permission.map(fs::Permissions::from_mode)
    }

    pub(crate) fn fix_home_dir(&mut self) -> Result<(), env::VarError> {
        fix_home_dir(&mut self.src)?;
        fix_home_dir(&mut self.dst)?;
        Ok(())
    }

    pub(crate) async fn link(&self, work_mode: WorkMode) -> async_std::io::Result<()> {
        match work_mode {
            WorkMode::HardLink => {
                async_std::fs::hard_link(&self.src, &self.dst).await?;
            }
            WorkMode::SymLink => {
                async_std::os::unix::fs::symlink(&self.src, &self.dst).await?;
            }
        }
        Ok(())
    }
}
