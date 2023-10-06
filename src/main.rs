mod socketinfo;

use std::fs::File;
use std::io::{BufRead, BufReader};
use socketinfo::linuxsocket::SocketInfo;

fn main() -> std::io::Result<()> {
   // let row ="8: 1400000A:C4B2 5A131268:01BB 06 00000000:00000000 03:000001F6 00000000     0        0 0 3 0000000052f0bc2a";
     //println!("{}",row);

    let  tcp_proc_file = File::open("/proc/net/tcp")?;
    let buff_reader = BufReader::new(tcp_proc_file);
    for line in buff_reader.lines().skip(1){
        if let Ok(socket_row) = line {
            //println!("{:?}",socket_row.as_str());
            let socket_info = SocketInfo::new(socket_row.as_str());
            println!("{:?}",socket_info);
        }
    }
    //


    Ok(())
}
