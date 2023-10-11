/* Fixing IntelliJ rust test failure
 https://stackoverflow.com/questions/76936606/intellij-automatically-adds-z-unstable-options-when-rust-tests-are-run
 There was a breaking change in Rust 1.70, that broke the test experience.
 Do you have org.rust.cargo.test.tool.window enabled in
 Help | Find Action | Experimental Features enabled? If yes, try to disable it.
 */
mod socketinfo_test {

    use crate::socketinfo::linuxsocket::{IpAddress, SocketInfo};
    use rstest::fixture;

    struct TestData {
        input : &'static str,
        expected_vector: Vec<&'static str>,
        expected_socketinfo: SocketInfo,
    }


    #[fixture]
    fn input_data() -> TestData {
        TestData{
            input : "8: 1400000A:C4B2 5A131268:01BB 06 00000000:00000000 03:000001F6 00000000     0        0 0 3 0000000052f0bc2a",
            expected_vector: Vec::from(
                ["8:", "1400000A:C4B2","5A131268:01BB", "06", "00000000:00000000","03:000001F6",
                    "00000000", "0", "0", "0", "3", "0000000052f0bc2a" ],
            ),
            expected_socketinfo: SocketInfo {
                local_address: IpAddress(10, 0, 0, 20),
                local_port: 50354,
                remote_address: IpAddress(104, 18, 19, 90),
                remote_port: 443,
                state: String::from("TIME_WAIT"),
                inode: 0, uid: 0
            }
        }
    }


    #[cfg(test)]
    mod linuxsocket_utils_tests{
        use rstest::rstest;
        use crate::socketinfo::socketinfo_tests::socketinfo_test::TestData;
        use crate::socketinfo::utils;
        use super::input_data;

        #[rstest]
        fn test_split_text_by_words(input_data: TestData){
            let sock_metadata = utils::split_text_by_words(input_data.input);
            //println!("Sock_metaData length {:?}",sock_metadata);

            assert_eq!(sock_metadata.len(), input_data.expected_vector.len());

            for i in 0..input_data.expected_vector.len() {
                assert_eq!(sock_metadata[i],input_data.expected_vector[i]);
            }
        }
    }

    #[cfg(test)]
    mod linuxsocket_tests{
        use std::{fs, io};
        use std::os::unix::fs::MetadataExt;
        use rstest::rstest;

        use crate::socketinfo::linuxsocket::SocketInfo;
        use crate::socketinfo::{utils};
        use crate::socketinfo::socketinfo_tests::socketinfo_test::TestData;
        use super::input_data;

        #[rstest]
        fn test_socket_instanciation(input_data : TestData){

            let socket_info = SocketInfo::new(input_data.input).unwrap();

            assert_eq!(socket_info,input_data.expected_socketinfo);
        }

        #[test]
        #[ignore]
        fn testfile_inode() -> io::Result<()> {
            let filepath = "/proc/313348/fd/209";
            let meta = fs::metadata(filepath)?;


            println!("Metadata for {} : {}", filepath, meta.ino());
            Ok(())
        }

        #[test]
        fn test_directory_entry() -> std::io::Result<()> {
            let dir = fs::read_dir("/proc")?;
            dir.for_each( |entry| {
                if let Ok(dir_entry) = entry {

                    let file_name = dir_entry.file_name();
                    if let Some(file) = file_name.to_str() {
                        if utils::isdigit(& file){
                            println!("{:?}",file_name);
                        }
                    }
                }
            });
            Ok(())
        }
    }
}

