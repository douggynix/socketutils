use std::collections::{HashMap, HashSet, LinkedList};
use std::fs;
use std::os::unix::fs::{MetadataExt};
use crate::socketinfo::linuxsocket::SocketInfo;
use crate::socketinfo::socketprocessinfo::ProcessInfo;
use crate::socketinfo::utils;


pub fn get_running_processes() -> std::io::Result<Vec<String>> {
    let dir = get_dir_content("/proc")?;
    let processes = dir.iter().filter(|file| utils::isdigit(& file.as_str()))
        .map(|file| file.to_string())
        .collect::<Vec<String>>();
    Ok(processes)
}


fn get_dir_content(path : &str) -> std::io::Result<Vec<String>> {
    let dir = fs::read_dir(path)?;
    let mut files: Vec<String> = Vec::new();
    for dir_entry in dir{
        if let Ok(entry) = dir_entry {
            if let Some( file) = entry.file_name().to_str() {
                files.push(file.to_string())
            }
        }
    }

    Ok(files)
}




pub fn get_process_cmdline(process_id : &String) -> std::io::Result<String>{
    let cmdline = fs::read_to_string(format!("/proc/{}/cmdline",process_id))?;
    Ok(cmdline)
}


pub fn get_inodes_for_process(process_id: &String) -> std::io::Result<HashSet<String>>{

    let mut process_inodes: HashSet<String> = HashSet::new();

    if let Ok(files) = get_dir_content(format!("/proc/{}/fd", process_id).as_str() ){
        process_inodes = files.iter()
            .map( //this one maps the filenames to unix inode same as ls -ailH
                  |file| {
                      let file_path = format!("/proc/{}/fd/{}", process_id, file);
                      if let Ok(metadata) = fs::metadata(&file_path){
                          format!("{}",metadata.ino())
                      }
                      else {
                          "".to_string()
                      }
                  }
            )
            .collect::<HashSet<String>>();

    }
    Ok(process_inodes)
}

pub fn get_processes_info(socket_list: &LinkedList<SocketInfo>) -> std::io::Result<HashMap<String,ProcessInfo>>{
    let mut inode_set = socket_list
        .iter()
        .filter(|sock_info| sock_info.inode > 2)
        .map(|sock_info| sock_info.inode.to_string())
        .collect::<HashSet<String>>();

    let processes = get_running_processes()?;

    let mut process_map: HashMap<String,ProcessInfo> = HashMap::new();

    for process_id in processes{
        //Find the list of inodes for this process from our socket inode set.
        //we will traverse /proc/pid/fd only once
        let this_process_inodes = get_inodes_for_process(& process_id)?;

        //during each loop we reduce the size of the inode_set to avoid multiple traversals in the directory tree
        //for /proc/pid/fd for each individual inodes in the general inode_set declared above
        this_process_inodes.iter()
            .filter( | & inode| {
                let has_inode = inode_set.contains(inode);
                //we remove existing inodes already found our space and runtime complexity for our search
                //upon next iteration
                inode_set.remove(inode);
                return has_inode;
            })
            .for_each(| inode_filtered| {
                let cmdline = get_process_cmdline(& process_id).unwrap();
                let process_info = ProcessInfo{
                    pid: process_id.parse::<usize>().unwrap(),
                    process_cmdline: cmdline,
                    ..ProcessInfo::default()
                };
                process_map.insert(format!("{}", inode_filtered), process_info);
            });
    }

    Ok(process_map)
}