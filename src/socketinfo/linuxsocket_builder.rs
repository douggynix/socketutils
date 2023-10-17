use std::collections::HashMap;
use sscanf::sscanf;
use crate::socketinfo::linuxsocket::{EndPoint, Protocol, SocketInfo, AddressType};
use super::utils;


const LOCAL_SOCKET: usize = 1;
const REMOTE_SOCKET: usize = 2;
const SOCKET_STATE: usize = 3;
const UID: usize = 7;
const INODE: usize = 9;

pub struct SocketInfoBuilder{
    socket_data: String,
    protocol: Protocol,
}

impl SocketInfoBuilder {
    pub fn new(data : String, proto: Protocol) -> Self {
        SocketInfoBuilder {
            socket_data: data,
            protocol: proto,
        }
    }

    pub fn build(self) -> Result<SocketInfo, sscanf::Error> {
        let base_socket_info : SocketInfo = build_common_entry(self.socket_data.as_str())? ;

        let socket_meta_vector = utils::split_text_by_words(self.socket_data.as_str());

        let local_endpoint = match self.protocol {
            Protocol::TCP | Protocol::UDP | Protocol::RAW => parse_socket_endpoint(socket_meta_vector[LOCAL_SOCKET])?,
            Protocol::TCP6 | Protocol::UDP6 => parse_socket_endpoint6(socket_meta_vector[LOCAL_SOCKET])?
        };

        let remote_endpoint = match self.protocol {
            Protocol::TCP | Protocol::UDP  | Protocol::RAW => parse_socket_endpoint(socket_meta_vector[REMOTE_SOCKET])?,
            Protocol::TCP6 | Protocol::UDP6 => parse_socket_endpoint6(socket_meta_vector[REMOTE_SOCKET])?
        };

        Ok(SocketInfo{
            protocol: self.protocol,
            local_endpoint,
            remote_endpoint,
            ..base_socket_info
        })
    }
}

fn get_socket_inode(socket_vec_entry: &Vec<&str>) -> Result<usize, sscanf::Error> {
    get_entry_usize(socket_vec_entry[INODE])
}

fn get_entry_usize(entry: &str) -> Result<usize, sscanf::Error> {
    let value: usize = sscanf!(entry,"{usize}")?;
    Ok(value)
}


fn parse_socket_endpoint(endpoint_entry: &str) -> Result<EndPoint, sscanf::Error> {
    //ip adress are stored in little endian format : shifted from right to left
    // for example 127.0.0.1 would be stored like 1 0 0 127 in little indian
    let little_endian : (u8,u8,u8,u8, u16) = sscanf!(endpoint_entry,"{u8:x}{u8:x}{u8:x}{u8:x}:{u16:x}")? ;

    let endpoint = EndPoint::new( vec![little_endian.3 as u16, little_endian.2 as u16 ,
                                       little_endian.1 as u16, little_endian.0 as u16],
                                  little_endian.4 , AddressType::IPV4  );
    Ok(endpoint)
}

fn parse_socket_endpoint6(endpoint_entry: &str) -> Result<EndPoint, sscanf::Error> {
    let little_endian6: (u16, u16, u16, u16, u16, u16, u16, u16, u16) = sscanf!(endpoint_entry,"{u16:x}{u16:x}{u16:x}{u16:x}{u16:x}{u16:x}{u16:x}{u16:x}:{u16:x}")? ;

    let endpoint6 = EndPoint::new(vec![little_endian6.0.to_be(), little_endian6.1.to_be(),
                                       little_endian6.2.to_be(), little_endian6.3.to_be() ,
                                       little_endian6.4.to_be(), little_endian6.5.to_be(),
                                       little_endian6.6.to_be(), little_endian6.7.to_be()],
                                  little_endian6.8, AddressType::IPV6 );

    Ok(endpoint6)
}


fn get_socket_uid(socket_record: &Vec<&str>) -> Result<usize, sscanf::Error>{
    get_entry_usize(socket_record[UID])
}

fn get_socket_state(socket_record: &Vec<&str>) -> Result<String, sscanf::Error> {
    //https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/include/net/tcp_states.h
    let tcp_state: HashMap<u8,&str> = HashMap::from(
        [
            (0x01,"ESTABLISHED"),
            (0x02, "SYN_SENT"),
            (0x03, "SYN_RECV"),
            (0x04, "FIN_WAIT1"),
            (0x05, "FIN_WAIT2"),
            (0x06, "TIME_WAIT"),
            (0x07, "CLOSE"),
            (0x08, "CLOSE_WAIT"),
            (0x09, "LAST_ACK"),
            (0x0A, "LISTENING"),
            (0x0B, "CLOSING"),
            (0x0C, "NEW_SYN_RECV"),
        ]
    );

    let state_index : u8 = sscanf!(socket_record[SOCKET_STATE],"{u8:x}")? ;

    let state = tcp_state.get(& state_index).unwrap_or(& "UNKNOWN");
    Ok( state.to_string())
}



pub fn build_common_entry(socket_data: &str) -> Result<SocketInfo, sscanf::Error>{

    let socket_info: SocketInfo = Default::default();
    let socket_vec_entry = utils::split_text_by_words(socket_data);

    let s_state = get_socket_state(&socket_vec_entry)?;

    let inode_num = get_socket_inode(& socket_vec_entry)?;

    let user_id = get_socket_uid(& socket_vec_entry)?;

    //println!("State : {}", tcp_state.get(& state));
    Ok(SocketInfo{
        state : s_state,
        inode: inode_num,
        uid: user_id,
        ..socket_info
    })
}
