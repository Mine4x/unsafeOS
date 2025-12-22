extern crate alloc;
use alloc::{
    collections::BTreeMap, format, string::{String, ToString}, vec::Vec
};
use spin::Mutex;

#[derive(Debug)]
enum Node {
    File(Vec<u8>),
    Directory(BTreeMap<String, Node>),
}

pub(crate) struct RamFs {
    root: Mutex<Node>,
}

impl RamFs {
    pub(crate) fn new() -> Self {
        Self {
            root: Mutex::new(Node::Directory(BTreeMap::new())),
        }
    }

    fn split_path(path: &str) -> Vec<&str> {
        path.trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect()
    }

    fn traverse_mut<'a>(
        mut __node__: &'a mut Node,
        path: &[&str],
    ) -> Result<&'a mut Node, String> {
        for part in path {
            match __node__ {
                Node::Directory(__children__) => {
                    __node__ = __children__
                        .get_mut(*part)
                        .ok_or_else(|| format!("Path not found: {}", part))?;
                }
                _ => return Err("Not a directory".to_string()),
            }
        }
        Ok(__node__)
    }

    pub(crate) fn read(&self, path: &str) -> Result<Vec<u8>, String> {
        let parts = Self::split_path(path);
        let mut __guard__ = self.root.lock();
        let mut __node__: &mut Node = &mut *__guard__;

        for part in parts {
            __node__ = match __node__ {
                Node::Directory(__children__) => __children__
                    .get_mut(part)
                    .ok_or_else(|| format!("Path not found: {}", part))?,
                _ => return Err("Not a directory".to_string()),
            };
        }

        match __node__ {
            Node::File(__data__) => Ok(__data__.clone()),
            _ => Err("Path is not a file".to_string()),
        }
    }

    pub(crate) fn write(&self, path: &str, data: &[u8]) -> Result<(), String> {
        let parts = Self::split_path(path);
        let (dirs, file) = parts.split_at(parts.len() - 1);
        let mut __guard__ = self.root.lock();
        let __parent__ = Self::traverse_mut(&mut __guard__, dirs)?;

        match __parent__ {
            Node::Directory(__children__) => {
                __children__.insert(file[0].to_string(), Node::File(data.to_vec()));
                Ok(())
            }
            _ => Err("Parent is not a directory".to_string()),
        }
    }

    pub(crate) fn create_dir(&self, path: &str) -> Result<(), String> {
        let parts = Self::split_path(path);
        let (dirs, new) = parts.split_at(parts.len() - 1);
        let mut __guard__ = self.root.lock();
        let __parent__ = Self::traverse_mut(&mut __guard__, dirs)?;

        match __parent__ {
            Node::Directory(__children__) => {
                __children__.insert(
                    new[0].to_string(),
                    Node::Directory(BTreeMap::new()),
                );
                Ok(())
            }
            _ => Err("Parent is not a directory".to_string()),
        }
    }

    pub(crate) fn list_dir(&self, path: &str) -> Result<Vec<String>, String> {
        let parts = Self::split_path(path);
        let mut __guard__ = self.root.lock();
        let mut __node__: &mut Node = &mut *__guard__;

        for part in parts {
            __node__ = match __node__ {
                Node::Directory(__children__) => __children__
                    .get_mut(part)
                    .ok_or_else(|| format!("Path not found: {}", part))?,
                _ => return Err("Not a directory".to_string()),
            };
        }

        match __node__ {
            Node::Directory(__children__) => Ok(__children__.keys().cloned().collect()),
            _ => Err("Path is not a directory".to_string()),
        }
    }
}