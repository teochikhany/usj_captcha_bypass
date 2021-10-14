use std::process::{Command, Stdio};
use std::io::{BufRead, Write, BufReader};
use std::vec::Vec;
use std::{thread, time};
use std::str;

fn main() {
    
    println!("Hello, world!");

    // .\tshark.exe -i wifi -w AAA.pcap -f "tcp and host 193.227.187.169"
    let mut tshark_capture = Command::new("D:\\Apps\\wireshark\\tshark.exe")
                            .args(["-i", "wifi", "-w", "BAA.pcap", "-f", "tcp and host 193.227.187.169" ])
                            // .stdout(Stdio::piped())
                            .spawn()
                            .expect("tshark command failed to start");

    thread::sleep(time::Duration::from_secs(5));

    tshark_capture.kill().unwrap();

    // .\tshark.exe -r .\AAA.pcap -c 13 -x
    let tshark_analyse = Command::new("D:\\Apps\\wireshark\\tshark.exe")
                .args(["-r", "BAA.pcap", "-x"])
                .output()
                .expect("tshark command failed to start");


    let res = &tshark_analyse.stdout;
    let char2: Vec<char> = res.iter().map(|b| *b as char).collect::<Vec<_>>();

    for i in 0 .. char2.len() - 4
    {
        // print!("{:?} ", char2[i]);

        if res[i] == 0x89 && res[i+1] == 0x50 && res[i+2] == 0x4E
        {
            println!("found one png");
        }
    }

}
