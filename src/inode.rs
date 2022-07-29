use std::{collections::HashMap, hash::Hash, sync::Arc};

use uuid::Uuid;

use crate::filer_utils::SwfsFile;


pub struct InodeTable {
    /// hashmap <inode, file attrs>
    inode_table: HashMap<u64,SwfsFile>,
    /// hashmap <parent, hashmap < file name, inode >>
    parent_table: HashMap<u64, HashMap<String, u64>>
}

impl InodeTable {
    pub fn new() -> InodeTable {
        InodeTable { 
            inode_table: HashMap::new(),
            parent_table: HashMap::new()
         }
    }

    pub fn generate_inode() -> u64 
    {
        // Generate a new UUID for inode
        let uuid = Uuid::new_v4();
        let inode = uuid.as_u64_pair().0;
        return inode;
    }

    // pub fn add_or_update_file_attr(&mut self, f: SwfsFile) 
    // {
    //     self.parent_table
    //         .entry(f.parent)
    //         .or_default()
    //         .insert(f.name.clone(), f.file_attr.ino);
    //     self.inode_table
    //         .insert(f.file_attr.ino, f.clone());
    // }
    //
    // pub fn delete_file_attr(&mut self, f: SwfsFile)
    // {
    //     self.inode_table.remove(&f.file_attr.ino);
    //     self.parent_table
    //         .entry(f.parent)
    //         .or_default()
    //         .remove(&f.name);
    // }

    pub fn file_attr_from_inode(&mut self, inode: u64) -> Option<&SwfsFile>
    {
        return self.inode_table.get(&inode);
    }

    pub fn file_attr_from_name_and_parent(&mut self, parent: u64, name: String) -> Option<&SwfsFile>
    {
        match self.parent_table.get(&parent)
        {
            Some(file_names) => match file_names.get(&name) {
                Some(&inode) => return self.inode_table.get(&inode),
                None => return None
            },
            None => return None
        }
    }

    pub fn clear_tables(&mut self)
    {
        self.inode_table.clear();
        self.parent_table.clear();
    }
}


