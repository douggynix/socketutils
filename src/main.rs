mod socketinfo;

use std::collections::{BTreeMap,LinkedList};
use std::error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use socketinfo::linuxsocket::SocketInfo;
use crate::socketinfo::linuxsocket::Protocol;
use crate::socketinfo::{socketprocessinfo_builder as processinfo_builder, utils};


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
            if let Ok(socket_info) = SocketInfo::builder(line,  net_protocol.clone() )
                .build() {
                socket_list.push_back(socket_info);
                //let process_info = ProcessInfo::builder(socket_list.back().unwrap().inode).build()?;
                //println!("{:?}",socket_list.back());
            }
        }
    }

    //build a HashMap with Key=inode_number and Value=ProcessInfo in order to find program name later
    // through SocketInfo.inode
    let process_info = processinfo_builder::get_processes_info(&socket_list)?;

    socket_list.iter().for_each( |socketinfo| {
        let inode = format!("{}", socketinfo.inode);

        if let Some(proc_info) = process_info.get( & inode) {
            let mut program = utils::truncate(& proc_info.process_cmdline, 64);

            program = utils::remove_non_printable_chars(&program);

            //println!("{:?} Pid={} Program={}",socketinfo , proc_info.pid,  program );
            println!("protocol={:?}, pid={}, local={:?}, remote={:?}, state={}, program={}",socketinfo.protocol , proc_info.pid,
                     socketinfo.local_endpoint,socketinfo.remote_endpoint, socketinfo.state, program);
        }
        else{
            //println!("{:?} Pid=-- Program=--",socketinfo);
            println!("protocol={:?}, pid=--, local={:?}, remote={:?}, state={}, program=--",socketinfo.protocol ,
                     socketinfo.local_endpoint,socketinfo.remote_endpoint, socketinfo.state);
        }
        //println!("{:?} Pid=-- Program=--",socketinfo);
    });

    Ok(())
}
