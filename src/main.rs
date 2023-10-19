mod socketinfo;

use std::collections::{BTreeMap,LinkedList};
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use socketinfo::linuxsocket::SocketInfo;
use crate::socketinfo::linuxsocket::Protocol;
use crate::socketinfo::{socketprocessinfo_builder as processinfo_builder};


fn main() -> std::result::Result<(),Box<dyn error::Error>> {
    let socket_files :BTreeMap<Protocol,&str>  = BTreeMap::from([
        (Protocol::TCP, "/proc/net/tcp"),
        (Protocol::TCP6, "/proc/net/tcp6"),
        (Protocol::UDP, "/proc/net/udp"),
        (Protocol::UDP6, "/proc/net/udp6"),
        (Protocol::RAW, "/proc/net/raw"),
    ]);

    let mut socket_list: LinkedList<SocketInfo> = LinkedList::new();


    for (net_protocol,net_file) in socket_files.iter() {
        let  proc_file = File::open(net_file)?;
        let buff_reader = BufReader::new(proc_file);
        for line in buff_reader.lines().skip(1).flatten(){
            if let Ok(socket_info) = SocketInfo::builder(line,  Box::new(*net_protocol) )
                .build() {
                socket_list.push_back(socket_info);
            }
        }
    }

    //build a HashMap with Key=inode_number and Value=ProcessInfo in order to find program name later
    // through SocketInfo.inode
    let process_map = processinfo_builder::get_processes_info(&socket_list)?;

    //Look for processnames for each socket using their inode number as key from process_map
    socket_list.iter().for_each( |socketinfo| {
        let inode = format!("{}", socketinfo.inode);

        if let Some(proc_info) = process_map.get( & inode) {
            let program = & proc_info.process_cmdline;

            println!("protocol={:?}, pid={}, local={:?}, remote={:?}, inode={}, state={}, program={}",socketinfo.protocol , proc_info.pid,
                     socketinfo.local_endpoint,socketinfo.remote_endpoint, socketinfo.inode, socketinfo.state, program);
        }
        else{
            println!("protocol={:?}, pid=--, local={:?}, remote={:?}, inode={}, state={}, program=--",socketinfo.protocol ,
                     socketinfo.local_endpoint,socketinfo.remote_endpoint, socketinfo.inode, socketinfo.state);
        }
    });

    Ok(())
}
