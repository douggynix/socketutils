use std::fmt;
use std::fmt::Formatter;
use crate::socketinfo::linuxsocket_builder::SocketInfoBuilder;

#[derive(Debug,PartialEq,Eq, Hash,Clone,Ord,PartialOrd)]
pub enum Protocol{
    TCP=0x01, UDP=0x02, TCP6=0x03,UDP6=0x04,
}


impl fmt::Debug for EndPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",self.to_string())
    }
}

impl ToString for EndPoint{
    fn to_string(&self) -> String {
        let ip_address = self.address.iter()
                .map(|item| {
                    match self.address.len()  {
                        4 => format!("{}",item),
                        _ => { //IPV6
                            if item == & 0_u16 {
                                format!("")
                            }
                            else {
                                //display ipv6 adress in hexadecimal format
                                format!("{:x}", item)
                            }
                        }
                    }
                })
                .collect::<Vec<String>>();

        if self.address.len() == 4 {
            format!("{} port={}",ip_address.join("."), self.port)
        }
        else{
            let mut addr = ip_address.join(":");
            //remove the last ':' ::::::1:
            addr.remove(addr.len()-1);
            format!("{} port={}",addr, self.port)
        }

    }
}



#[derive(PartialEq,Default)]
pub struct EndPoint {
    port : u16,
    address: Vec<u16>,
}

impl EndPoint {
    pub fn new(address : Vec<u16>, port : u16) -> EndPoint {
        EndPoint{
            port,
            address,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SocketInfo {
    pub protocol: Protocol,
    pub local_endpoint: EndPoint,
    pub remote_endpoint:  EndPoint,
    pub state : String,
    pub inode: usize,
    pub uid: usize,
}

impl Default for SocketInfo {
    fn default() -> Self {
        SocketInfo{
            protocol: Protocol::TCP,
            local_endpoint: Default::default(),
            remote_endpoint: Default::default(),
            state: "".to_string(),
            inode: 0,
            uid: 0,
        }
    }
}

impl SocketInfo {
    pub fn builder(procfs_record: String, proto : Protocol) -> SocketInfoBuilder{
         SocketInfoBuilder::new(procfs_record,proto)
    }
}

