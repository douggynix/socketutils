/* Fixing IntelliJ rust test failure
 https://stackoverflow.com/questions/76936606/intellij-automatically-adds-z-unstable-options-when-rust-tests-are-run
 There was a breaking change in Rust 1.70, that broke the test experience.
 Do you have org.rust.cargo.test.tool.window enabled in
 Help | Find Action | Experimental Features enabled? If yes, try to disable it.
 */
mod socketinfo_test {

    use crate::socketinfo::linuxsocket::{IpAddress, SocketInfo};

    struct TestData {
        input : &'static str,
        expected_vector: Vec<&'static str>,
        expected_socketinfo: SocketInfo,
    }


    fn setup() -> TestData {
        return TestData{
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

        use crate::socketinfo::linuxsocket_utils;
        use super::setup;

        #[test]
        fn test_get_word_vector(){
            let input_data = setup();
            let sock_metadata = linuxsocket_utils::get_word_vector(input_data.input);
            //println!("Sock_metaData length {:?}",sock_metadata);

            assert_eq!(sock_metadata.len(), input_data.expected_vector.len());

            for i in 0..input_data.expected_vector.len() {
                assert_eq!(sock_metadata[i],input_data.expected_vector[i]);
            }
        }
    }

    #[cfg(test)]
    mod linuxsocket_tests{
        use crate::socketinfo::linuxsocket::SocketInfo;
        use super::setup;

        #[test]
        fn test_socket_instanciation(){
            let input_data = setup();
            let socket_info = SocketInfo::new(input_data.input);

            assert_eq!(socket_info,input_data.expected_socketinfo);
        }
    }
}

