mod socketinfo;

use std::collections::{BTreeMap};
use std::fs::File;
use std::io::{BufRead, BufReader};
use socketinfo::linuxsocket::SocketInfo;
use crate::socketinfo::linuxsocket::Protocol;

fn main() -> std::io::Result<()> {
   // let row ="8: 1400000A:C4B2 5A131268:01BB 06 00000000:00000000 03:000001F6 00000000     0        0 0 3 0000000052f0bc2a";
     //println!("{}",row);
    let socket_files :BTreeMap<Protocol,&str>  = BTreeMap::from([
        (Protocol::TCP, "/proc/net/tcp"),
        (Protocol::TCP6, "/proc/net/tcp6"),
        (Protocol::UDP, "/proc/net/udp"),
        (Protocol::UDP6, "/proc/net/udp6"),
    ]);

    for (net_protocol,net_file) in socket_files.iter() {
        let  proc_file = File::open(net_file)?;
        let buff_reader = BufReader::new(proc_file);
        for line in buff_reader.lines().skip(1).flatten(){
            let socket_info =
                SocketInfo::builder(line,  net_protocol.clone() )
                    .build();

            println!("{:?}",socket_info.unwrap());
        }
    }

    Ok(())
}
