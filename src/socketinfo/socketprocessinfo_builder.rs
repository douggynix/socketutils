use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, LinkedList};
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Deref;
use std::os::unix::fs::{MetadataExt};
use crate::socketinfo::linuxsocket::SocketInfo;
use crate::socketinfo::socketprocessinfo::ProcessInfo;
use crate::socketinfo::utils;


pub struct ProcessInfoBuilder{
    inode: usize
}

impl ProcessInfoBuilder {
    pub fn new(inode: usize)->Self {
        ProcessInfoBuilder{
            inode: inode.clone(),
        }
    }

    pub fn build(self) -> std::io::Result<Option<ProcessInfo>>{
        let process_info = get_processinfo_for_inode(self.inode)?;
        Ok(process_info)
    }
}

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


pub fn get_processinfo_for_inode(inode: usize) -> std::io::Result<Option<ProcessInfo>>{
    let processes = get_running_processes()?;
    let mut proc_info: Option<ProcessInfo> = None;
    for process_id in processes.iter(){

        //if inode is found used by this process_id
        let inode_str = format!("{}",inode);
        if has_inode(process_id, & inode_str ){
            let cmdline = get_process_cmdline(& process_id)?;
            proc_info = Some(ProcessInfo{
                pid: process_id.parse::<usize>().unwrap(),
                process_cmdline: cmdline,
                ..ProcessInfo::default()
            });
            break;
        }
    }

    Ok(proc_info)
}


pub fn get_process_cmdline(process_id : &String) -> std::io::Result<String>{
    let  cmdline_file = File::open(format!("/proc/{}/cmdline",process_id))?;
    let buff_reader = BufReader::new(cmdline_file);
    let cmdline = buff_reader.lines()
                            .flat_map(|line| line)
                            .collect::<String>();
    Ok(cmdline)
}


pub fn has_inode(process_id: &String, inode: & String) -> bool {
    if let Ok(files) = get_dir_content(format!("/proc/{}/fd", process_id).as_str() ){
        let inode_exist = files.iter()
            .map( //this one maps the filenames to unix inode same as ls -ailH
                |file| {
                    let filePath = format!("/proc/{}/fd/{}",process_id,file);
                    if let Ok(metadata) = fs::metadata(&filePath){
                       format!("{}",metadata.ino())
                    }
                    else {
                        "".to_string()
                    }
                }
            )
            .filter(|file_inode| file_inode.to_string().eq(inode))
            .count().eq(& 1);

        inode_exist
    }
    else{
        return false;
    }
}

pub fn get_inodes_for_process(process_id: &String) -> std::io::Result<HashSet<String>>{

    let mut processInodes : HashSet<String> = HashSet::new();

    if let Ok(files) = get_dir_content(format!("/proc/{}/fd", process_id).as_str() ){
        processInodes = files.iter()
            .map( //this one maps the filenames to unix inode same as ls -ailH
                  |file| {
                      let filePath = format!("/proc/{}/fd/{}",process_id,file);
                      if let Ok(metadata) = fs::metadata(&filePath){
                          format!("{}",metadata.ino())
                      }
                      else {
                          "".to_string()
                      }
                  }
            )
            .collect::<HashSet<String>>();

    }
    Ok(processInodes)
}

pub fn get_processes_info(socketList: &LinkedList<SocketInfo>) -> std::io::Result<HashMap<String,ProcessInfo>>{
    let mut inodeSet = socketList
        .iter()
        .filter(|sock_info| sock_info.inode > 2)
        .map(|sock_info| sock_info.inode.to_string())
        .collect::<HashSet<String>>();

    let processes = get_running_processes()?;

    let mut processMap: HashMap<String,ProcessInfo> = HashMap::new();

    for process_id in processes{
        //Find the list of inodes for this process from our socket inode set.
        //we will traverse /proc/pid/fd only once
        let this_process_inodes = get_inodes_for_process(& process_id)?;

        //during each loop we reduce the size of the inodeSet to avoid multiple traversals in the directory tree
        //for /proc/pid/fd for each individual inodes in the general inodeSet declared above
        this_process_inodes.iter()
            .filter( | & inode| {
                let has_inode = inodeSet.contains(inode);
                //we remove existing inodes already found our space and runtime complexity for our search
                //upon next iteration
                inodeSet.remove(inode);
                return has_inode;
            })
            .for_each(| inode_filtered| {
                let cmdline = get_process_cmdline(& process_id).unwrap();
                let process_info = ProcessInfo{
                    pid: process_id.parse::<usize>().unwrap(),
                    process_cmdline: cmdline,
                    ..ProcessInfo::default()
                };
                processMap.insert(format!("{}",inode_filtered),process_info);
            });
    }

    Ok(processMap)
}