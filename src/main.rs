mod socketinfo;

use std::borrow::Borrow;
use std::collections::{BTreeMap, HashSet, LinkedList};
use std::error;
use std::fmt::format;
use std::fs::File;
use std::io::{BufRead, BufReader};
use socketinfo::linuxsocket::SocketInfo;
use crate::socketinfo::linuxsocket::Protocol;
use crate::socketinfo::socketprocessinfo::ProcessInfo;
use crate::socketinfo::socketprocessinfo_builder as processinfo_builder;


fn main() -> std::result::Result<(),Box<dyn error::Error>> {
    let socket_files :BTreeMap<Protocol,&str>  = BTreeMap::from([
        (Protocol::TCP, "/proc/net/tcp"),
        (Protocol::TCP6, "/proc/net/tcp6"),
        (Protocol::UDP, "/proc/net/udp"),
        (Protocol::UDP6, "/proc/net/udp6"),
        (Protocol::RAW, "/proc/net/raw"),
    ]);

    let mut socketList: LinkedList<SocketInfo> = LinkedList::new();


    for (net_protocol,net_file) in socket_files.iter() {
        let  proc_file = File::open(net_file)?;
        let buff_reader = BufReader::new(proc_file);
        for line in buff_reader.lines().skip(1).flatten(){
            if let Ok(socket_info) = SocketInfo::builder(line,  net_protocol.clone() )
                .build() {
                socketList.push_back(socket_info);
                //let process_info = ProcessInfo::builder(socketList.back().unwrap().inode).build()?;
                //println!("{:?}",socketList.back());
            }
        }
    }

    let processInfo = processinfo_builder::get_processes_info(& socketList)?;
    socketList.iter().for_each( |socketinfo| {
        let inode = format!("{}", socketinfo.inode);

        if let Some(proc_info) = processInfo.get( & inode) {
            let mut program = format!("{}", proc_info.process_cmdline);
            program.truncate(64);
            println!("{:?} Pid={} Program={}",socketinfo , proc_info.pid,  program );
        }
        else{
            println!("{:?} Pid=-- Program=--",socketinfo);
        }
        //println!("{:?} Pid=-- Program=--",socketinfo);
    });

    Ok(())
}
