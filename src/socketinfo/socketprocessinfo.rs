use crate::socketinfo::linuxsocket_builder::SocketInfoBuilder;
use crate::socketinfo::socketprocessinfo_builder::ProcessInfoBuilder;

#[derive(Debug,Eq,Hash,PartialEq,Default)]
pub struct ProcessInfo{
    pub pid: usize,
    pub ppid: Option<usize>,
    pub process_cmdline: String
}


impl ProcessInfo {
    pub fn builder(inode: usize) -> ProcessInfoBuilder{
        ProcessInfoBuilder::new(inode)
    }
}

