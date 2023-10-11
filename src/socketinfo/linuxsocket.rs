use std::collections::HashMap;
use sscanf::sscanf;
use super::utils;


const LOCAL_SOCKET: usize = 1;
const REMOTE_SOCKET: usize = 2;
const SOCKET_STATE: usize = 3;
const UID: usize = 7;
const INODE: usize = 9;


#[derive(Debug, PartialEq)]
pub struct IpAddress(pub u8,pub u8,pub u8,pub u8);

#[derive(Debug, PartialEq)]
pub struct EndPoint {
    port : u16,
    address: IpAddress,
}

impl EndPoint {
    fn new(address : IpAddress, port : u16) -> EndPoint {
        EndPoint{
            port,
            address,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SocketInfo {
    pub local_address: IpAddress,
    pub local_port: u16,
    pub remote_address: IpAddress,
    pub remote_port: u16,
    pub state : String,
    pub inode: usize,
    pub uid: usize,
}


impl SocketInfo {
    pub fn new(procfs_record: &str) -> Result<SocketInfo, sscanf::Error>{
         build_socket_metadata(procfs_record)
    }
}


fn build_socket_metadata(socket_data : &str) -> Result<SocketInfo, sscanf::Error>{
    let mut socket_info : SocketInfo = SocketInfo {
        local_address: IpAddress(127, 0, 0, 1),
        local_port: 0,
        remote_address: IpAddress(0, 0, 0, 0),
        remote_port: 0,
        state : String::from("UNKNOWN"),
        inode: 0,
        uid: 0,
    };

    let socket_vec_entry = utils::split_text_by_words(socket_data);

    let local_addr_info = get_local_socket(&socket_vec_entry)?;
    let remote_addr_info = get_remote_socket(&socket_vec_entry) ?;

    let s_state = get_socket_state(&socket_vec_entry)?;

    let inode_num = get_socket_inode(& socket_vec_entry)?;

    let user_id = get_socket_uid(& socket_vec_entry)?;

    //println!("State : {}", tcp_state.get(& state));
    socket_info = SocketInfo{
        local_address : local_addr_info.address,
        local_port: local_addr_info.port,
        remote_address : remote_addr_info.address,
        remote_port: remote_addr_info.port,
        state : s_state,
        inode: inode_num,
        uid: user_id,
        ..socket_info
    };
    //println!("Socket meta data {:?}",socket_info);
    return Ok(socket_info);
}

fn get_socket_inode(socket_vec_entry: &Vec<&str>) -> Result<usize, sscanf::Error> {
    get_entry_usize(socket_vec_entry[INODE])
}

fn get_entry_usize(entry: &str) -> Result<usize, sscanf::Error> {
    let value: usize = sscanf!(entry,"{usize}")?;
    Ok(value)
}


fn get_socket_data(socket_entry: &str) -> Result<EndPoint, sscanf::Error> {

    let little_endian : (u8,u8,u8,u8, u16) = sscanf!(socket_entry,"{u8:x}{u8:x}{u8:x}{u8:x}:{u16:x}")? ;
    let endpoint = EndPoint::new( IpAddress(little_endian.3,
                                            little_endian.2, little_endian.1 ,
                                            little_endian.0) , little_endian.4 );
    Ok(endpoint)
}

fn get_local_socket(socket_record : &Vec<&str>) -> Result<EndPoint, sscanf::Error>{
    get_socket_data(socket_record[LOCAL_SOCKET])
}

fn get_remote_socket(socket_record : &Vec<&str>) -> Result<EndPoint, sscanf::Error>{
    get_socket_data(socket_record[REMOTE_SOCKET])
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
