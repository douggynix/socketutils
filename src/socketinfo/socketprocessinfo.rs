#[derive(Debug,Eq,Hash,PartialEq,Default)]
pub struct ProcessInfo{
    pub pid: usize,
    pub ppid: Option<usize>,
    pub process_cmdline: String
}


