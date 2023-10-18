/* Fixing IntelliJ rust test failure
 https://stackoverflow.com/questions/76936606/intellij-automatically-adds-z-unstable-options-when-rust-tests-are-run
 There was a breaking change in Rust 1.70, that broke the test experience.
 Do you have org.rust.cargo.test.tool.window enabled in
 Help | Find Action | Experimental Features enabled? If yes, try to disable it.
 */
mod socketinfo_test {

    use crate::socketinfo::linuxsocket::{EndPoint, Protocol, SocketInfo};
    use rstest::fixture;
    use crate::socketinfo::linuxsocket::AddressType::IPV4;

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
                protocol: Protocol::TCP,
                local_endpoint: EndPoint::new(vec![10, 0, 0, 20],50354, IPV4),
                remote_endpoint: EndPoint::new(vec![104, 18, 19, 90],443, IPV4),
                state: String::from("TIME_WAIT"),
                inode: 0, uid: 0
            }
        }
    }


    #[cfg(test)]
    mod linuxsocket_utils_tests{
        use rstest::rstest;
        use crate::socketinfo::linuxsocket::{Protocol};
        use crate::socketinfo::linuxsocket_builder::SocketInfoBuilder;
        use crate::socketinfo::socketinfo_tests::socketinfo_test::TestData;
        use crate::socketinfo::utils;
        use super::input_data;

        #[rstest]
        fn test_split_text_by_words(input_data: TestData){
            let sock_metadata = utils::split_text_by_words(input_data.input);
            //println!("Sock_metaData length {:?}",sock_metadata);

            assert_eq!(sock_metadata.len(), input_data.expected_vector.len());

            for (i, & expected_vec_data) in input_data.expected_vector.iter().enumerate() {
                assert_eq!(sock_metadata[i],expected_vec_data);
            }
        }

        #[rstest]
        fn test_socketinfobuilder(input_data: TestData){
            /*let sock_data = String::from("00000000000000000000000001000000:B2D4 00000000000000000000000001000000:0016 08 00000000:00000000 00:00000000 00000000  1000        0 24271887 1 0000000088a11a71 20 3 28 10 -1");
            let socket_info = SocketInfoBuilder::new(sock_data, Protocol::TCP6).build();
            println!("{:?}",socket_info.unwrap()); */
            //println!("{:x}",10);
        }
    }

    #[cfg(test)]
    mod linuxsocket_tests{
        use std::{fs, io};
        use std::os::unix::fs::MetadataExt;
        use rstest::rstest;

        use crate::socketinfo::linuxsocket::{Protocol, SocketInfo};
        use crate::socketinfo::{utils};
        use crate::socketinfo::socketinfo_tests::socketinfo_test::TestData;
        use crate::socketinfo::socketprocessinfo::ProcessInfo;
        use crate::socketinfo::socketprocessinfo_builder::{get_process_cmdline};
        use super::input_data;

        #[rstest]
        fn test_socket_instanciation(input_data : TestData){

            let socket_info =
                SocketInfo::builder(input_data.input.to_string(),Protocol::TCP)
                            .build().unwrap();

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
        #[ignore]
        fn test_directory_entry() -> std::io::Result<()> {
           Ok(())
        }
    }
}

