use std::time::{UNIX_EPOCH, SystemTime, Duration};

use fuser::{FileAttr, FileType};
use chrono::{DateTime};
use uuid::Uuid;

use crate::inode::InodeTable;

#[derive(Clone)]
pub struct SwfsFile {
    pub full_path: String,
    pub name: String,
    pub parent: u64,
    // "FullPath": "/topics",
    //   "Mtime": "2022-07-08T16:54:46Z",
    //   "Crtime": "2022-07-08T16:54:46Z",
    //   "Mode": 2147484141,
    //   "Uid": 998,
    //   "Gid": 998,
    //   "Mime": "",
    //   "TtlSec": 0,
    //   "UserName": "",
    //   "GroupNames": null,
    //   "SymlinkTarget": "",
    //   "Md5": null,
    //   "FileSize": 0,
    //   "Rdev": 0,
    //   "Inode": 0,
    //   "Extended": null,
    //   "HardLinkId": null,
    //   "HardLinkCounter": 0,
    //   "Content": null,
    //   "Remote": null,
    //   "Quota": 0
    pub file_attr: FileAttr
}



pub fn convert_json_swfsfile_vec(json: serde_json::Value) -> Vec<SwfsFile>
{
    let mut swfs_files = Vec::new();
    for entry in json["Entries"].as_array().unwrap() {
        let full_path = entry["FullPath"].as_str().unwrap().to_string();
        let inode = entry["Inode"].as_u64().unwrap();

        let mut file_type = FileType::RegularFile;
        if inode == 0 { file_type = FileType::Directory }
        let inode = InodeTable::generate_inode();

        let file_size = entry["FileSize"].as_u64().unwrap();
        let block_size: u32 = 512;
        let blocks = ((file_size as f64)/block_size as f64).ceil() as u64;

        let crtime = convert_rc3339_to_system_time(entry["Crtime"].as_str().unwrap().to_string());
        let mtime = convert_rc3339_to_system_time(entry["Mtime"].as_str().unwrap().to_string());
        let mut mode = entry["Mode"].as_u64().unwrap();
        if mode > u16::MAX as u64 { mode = 0; }
        let mode = mode as u16;

        let mut hard_link_counter = entry["HardLinkCounter"].as_u64().unwrap() as u32;
        if hard_link_counter == 0 { hard_link_counter = 1 }
        
        let uid = entry["Uid"].as_u64().unwrap() as u32;
        let gid = entry["Gid"].as_u64().unwrap() as u32;


        let file_attr = FileAttr {
            ino:  inode,
            size: file_size,
            blocks: blocks,
            atime: mtime,
            mtime: mtime,
            ctime: mtime,
            crtime: crtime,
            kind: file_type,
            perm: mode,
            nlink: hard_link_counter,
            uid: uid,
            gid: gid,
            rdev: 0,
            blksize: block_size,
            flags: 0
        };
        let swfs_file = SwfsFile {
            full_path: full_path.clone(),
            name: calc_file_name_from_path(full_path.clone()),
            parent: calc_parent_from_path(full_path.clone()), 
            file_attr: file_attr
        };
        swfs_files.push(swfs_file);
    }
    return swfs_files;
}

fn convert_rc3339_to_system_time(time: String) -> SystemTime {
    let rfc3339 = DateTime::parse_from_rfc3339(&time).unwrap();
    let millis = Duration::from_millis(rfc3339.timestamp_millis() as u64);
    return UNIX_EPOCH + millis;
}

fn calc_file_name_from_path(full_path: String) -> String {
    let mut path = full_path.clone();
    if path.len() > 1 && full_path.ends_with('/')
    {
        _ = &path.pop();
    }
    return path.split('/').last().unwrap_or("").into();
}

fn calc_parent_from_path(full_path: String) -> u64 {
    let mut path = full_path.clone();
    if path.len() > 1 && full_path.ends_with('/')
    {
        _ = &path.pop();
    }
    return path.split('/').count() as u64 - 1;
}

#[cfg(test)]
mod tests {
    use crate::filer_utils::{calc_parent_from_path, calc_file_name_from_path};

    /// calc_parent_from_path tests
    #[test]
    fn calc_parent_from_path_multiple_parents() {
        let full_path = "/test1/test2".into();
        assert_eq!(calc_parent_from_path(full_path), 2);
    }
    #[test]
    fn calc_parent_from_path_extra_slash() {
        let full_path = "/test1/test2/".into();
        assert_eq!(calc_parent_from_path(full_path), 2);
    }
    #[test]
    fn calc_parent_from_path_single() {
        let full_path = "/test1".into();
        assert_eq!(calc_parent_from_path(full_path), 1);
    }
    #[test]
    fn calc_parent_from_path_root() {
        let full_path = "/".into();
        assert_eq!(calc_parent_from_path(full_path), 1);
    }
    #[test]
    fn calc_parent_from_path_empty() {
        let full_path = "".into();
        assert_eq!(calc_parent_from_path(full_path), 0);
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