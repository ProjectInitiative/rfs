use chrono::DateTime;
use fuse_mt::{FileAttr, FileType};
use std::path::Path;
use std::{
    error::Error,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::pb::filer_pb::Entry;

#[derive(Clone)]
pub struct SwfsFile<'a> {
    pub path: &'a Path,
    pub attr: FileAttr,
    pub entry: Entry,
}

pub fn convert_entry_to_swfsfile(path: &Path, entry: Entry) -> Option<SwfsFile> {
    let mut file_type = FileType::Directory;
    let nlink = entry.hard_link_counter;
    if !entry.is_directory {
        file_type = FileType::RegularFile;
    }

    let attributes = match &entry.attributes {
        Some(attr) => attr,
        None => return None,
    };

    let attr = FileAttr {
        size: attributes.file_size as u64,
        blocks: 0,
        atime: convert_unix_sec_to_system_time(attributes.mtime as u64),
        mtime: convert_unix_sec_to_system_time(attributes.mtime as u64),
        ctime: convert_unix_sec_to_system_time(attributes.crtime as u64),
        crtime: convert_unix_sec_to_system_time(attributes.crtime as u64),
        kind: file_type,
        perm: attributes.file_mode as u16,
        nlink: nlink as u32,
        uid: attributes.uid,
        gid: attributes.gid,
        rdev: attributes.rdev,
        flags: 0,
    };

    Some(SwfsFile { path, attr, entry })
}

pub fn extract_name_parent_from_path(path: &Path) -> Option<(&str, &str)> {
    match path.to_str() {
        Some(path_str) => {
            if path_str == "/" {
                return Some(("/", "/"));
            }
        }
        None => {}
    }

    let name = match path.file_name() {
        Some(opt_str) => match opt_str.to_str() {
            Some(name) => name,
            None => return None,
        },
        None => return None,
    };
    let parent = match path.parent() {
        Some(opt_str) => match opt_str.to_str() {
            Some(parent) => parent,
            None => return None,
        },
        None => return None,
    };
    Some((name, parent))
}

pub fn convert_unix_sec_to_system_time(seconds: u64) -> SystemTime {
    let duration = Duration::from_secs(seconds);
    UNIX_EPOCH + duration
}

fn convert_rc3339_to_system_time(time: String) -> SystemTime {
    let rfc3339 = DateTime::parse_from_rfc3339(&time).unwrap();
    let duration = Duration::from_millis(rfc3339.timestamp_millis() as u64);
    UNIX_EPOCH + duration
}

fn calc_file_name_from_path(full_path: String) -> String {
    let mut path = full_path.clone();
    // remove trailing slash
    if path.len() > 1 && full_path.ends_with('/') {
        _ = &path.pop();
    }
    path.split('/').last().unwrap_or("").into()
}

fn calc_parent_from_path(full_path: String) -> String {
    let mut path = full_path.clone();
    // remove trailing slash
    if path.len() > 1 && full_path.ends_with('/') {
        _ = &path.pop();
    }
    let mut path_vec: Vec<&str> = path.split('/').collect();
    // if there are only two elements ["","test1"] ensure second item is ""
    // so join will properly add the root parent /
    if path_vec.len() == 2 {
        path_vec[1] = "";
    }
    // if path is longer than / or /parent1, remove last item from vec
    if path_vec.len() > 2 && !path_vec.last().unwrap().eq(&"") {
        _ = path_vec.pop();
    }
    path_vec.join("/")
}

fn calc_parent_num_from_path(full_path: String) -> u64 {
    let mut path = full_path.clone();
    // remove trailing slash
    if path.len() > 1 && full_path.ends_with('/') {
        _ = &path.pop();
    }
    path.split('/').count() as u64 - 1
}

// pub fn convert_json_swfsfile_vec(json: serde_json::Value) -> Vec<&'a SwfsFile>
// {
//     let mut swfs_files = Vec::new();
//     for entry in json["Entries"].as_array().unwrap() {
//         let full_path = entry["FullPath"].as_str().unwrap().to_string();
//         let inode = entry["Inode"].as_u64().unwrap();
//
//         let mut file_type = FileType::RegularFile;
//         if inode == 0 { file_type = FileType::Directory }
//         let inode = InodeTable::generate_inode();
//
//         let file_size = entry["FileSize"].as_u64().unwrap();
//         let block_size: u32 = 512;
//         let blocks = ((file_size as f64)/block_size as f64).ceil() as u64;
//
//         let crtime = convert_rc3339_to_system_time(entry["Crtime"].as_str().unwrap().to_string());
//         let mtime = convert_rc3339_to_system_time(entry["Mtime"].as_str().unwrap().to_string());
//         let mut mode = entry["Mode"].as_u64().unwrap();
//         if mode > u16::MAX as u64 { mode = 0; }
//         let mode = mode as u16;
//
//         let mut hard_link_counter = entry["HardLinkCounter"].as_u64().unwrap() as u32;
//         if hard_link_counter == 0 { hard_link_counter = 1 }
//
//         let uid = entry["Uid"].as_u64().unwrap() as u32;
//         let gid = entry["Gid"].as_u64().unwrap() as u32;
//
//
//         let file_attr = FileAttr {
//             // ino:  inode,
//             size: file_size,
//             blocks: blocks,
//             atime: mtime,
//             mtime: mtime,
//             ctime: mtime,
//             crtime: crtime,
//             kind: file_type,
//             perm: mode,
//             nlink: hard_link_counter,
//             uid: uid,
//             gid: gid,
//             rdev: 0,
//             // blksize: block_size,
//             flags: 0
//         };
//         let swfs_file = SwfsFile {
//             full_path: full_path.clone(),
//             name: calc_file_name_from_path(full_path.clone()),
//             // parent: calc_parent_from_path(full_path.clone()),
//             parent: calc_parent_from_path(full_path.clone()),
//             file_attr: file_attr
//         };
//         swfs_files.push(swfs_file);
//     }
//     swfs_files
// }

#[cfg(test)]
mod tests {
    use crate::filer_utils::{
        calc_file_name_from_path, calc_parent_from_path, calc_parent_num_from_path,
        extract_name_parent_from_path,
    };
    use std::path::Path;

    /// extract_name_parent_from_path tests
    #[test]
    fn extract_name_parent_from_path_root() {
        let path = Path::new("/");
        assert_eq!(extract_name_parent_from_path(path).unwrap(), ("", "/"));
    }
    #[test]
    fn extract_name_parent_from_path_single() {
        let path = Path::new("/test");
        assert_eq!(extract_name_parent_from_path(path).unwrap(), ("test", "/"));
    }
    #[test]
    fn extract_name_parent_from_path_multi() {
        let path = Path::new("/test1/test2/test3");
        assert_eq!(
            extract_name_parent_from_path(path).unwrap(),
            ("test3", "/test1/test2")
        );
    }
    #[test]
    fn extract_name_parent_from_path_empty() {
        let path = Path::new("");
        assert_eq!(extract_name_parent_from_path(path), None);
    }

    /// calc_parent_from_path tests
    #[test]
    fn calc_parent_num_from_path_multiple_parents() {
        let full_path = "/test1/test2".into();
        assert_eq!(calc_parent_num_from_path(full_path), 2);
    }
    #[test]
    fn calc_parent_num_from_path_extra_slash() {
        let full_path = "/test1/test2/".into();
        assert_eq!(calc_parent_num_from_path(full_path), 2);
    }
    #[test]
    fn calc_parent_num_from_path_single() {
        let full_path = "/test1".into();
        assert_eq!(calc_parent_num_from_path(full_path), 1);
    }
    #[test]
    fn calc_parent_num_from_path_root() {
        let full_path = "/".into();
        assert_eq!(calc_parent_num_from_path(full_path), 1);
    }
    #[test]
    fn calc_parent_num_from_path_empty() {
        let full_path = "".into();
        assert_eq!(calc_parent_num_from_path(full_path), 0);
    }

    // test string representation of paths
    #[test]
    fn calc_parent_from_path_multiple_parents() {
        let full_path = "/test1/test2".into();
        assert_eq!(calc_parent_from_path(full_path), "/test1");
    }
    #[test]
    fn calc_parent_from_path_extra_slash() {
        let full_path = "/test1/test2/".into();
        assert_eq!(calc_parent_from_path(full_path), "/test1");
    }
    #[test]
    fn calc_parent_from_path_single() {
        let full_path = "/test1".into();
        assert_eq!(calc_parent_from_path(full_path), "/");
    }
    #[test]
    fn calc_parent_from_path_root() {
        let full_path = "/".into();
        assert_eq!(calc_parent_from_path(full_path), "/");
    }
    #[test]
    fn calc_parent_from_path_empty() {
        let full_path = "".into();
        assert_eq!(calc_parent_from_path(full_path), "");
    }

    ///calc_file_name_from_path tests
    #[test]
    fn calc_file_name_from_path_multiple_parents() {
        let full_path = "/test1/test2".into();
        assert_eq!(calc_file_name_from_path(full_path), "test2");
    }
    #[test]
    fn calc_file_name_from_path_extra_slash() {
        let full_path = "/test1/test2/".into();
        assert_eq!(calc_file_name_from_path(full_path), "test2");
    }
    #[test]
    fn calc_file_name_from_path_single() {
        let full_path = "/test1".into();
        assert_eq!(calc_file_name_from_path(full_path), "test1");
    }
    #[test]
    fn calc_file_name_from_path_root() {
        let full_path = "/".into();
        assert_eq!(calc_file_name_from_path(full_path), "");
    }
    #[test]
    fn calc_file_name_from_path_empty() {
        let full_path = "".into();
        assert_eq!(calc_file_name_from_path(full_path), "");
    }
}
