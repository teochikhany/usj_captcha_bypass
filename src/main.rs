extern crate image;
extern crate hex;

use std::process::Command;
use std::str;
use wait_timeout::ChildExt;
use std::time::Duration;


fn main() {
    
    println!("Hello, world!");

    // .\tshark.exe -i wifi -w AAA.pcap -f "tcp and host 193.227.187.169"
    let _tshark_capture = Command::new("D:\\Apps\\wireshark\\tshark.exe")
                            .args(["-i", "wifi", "-w", "BAA.pcap", "-f", "tcp and host 193.227.187.169" ])
                            .spawn()
                            .expect("tshark command failed to start")
                            .wait_timeout(Duration::from_secs(10));

    println!("finished waiting");

    // D:\\Apps\\wireshark\\tshark.exe -r .\BAA.pcap -T json -Y 'http' -x > test.json
    let tshark_analyse = Command::new("D:\\Apps\\wireshark\\tshark.exe")
                .args(["-r", "BAA.pcap", "-T", "json", "-Y", "http", "-x"])
                .output()
                .expect("tshark command failed to start");

    let res = str::from_utf8(&tshark_analyse.stdout).unwrap();

    let responce = json::parse(res).unwrap();

    println!("responce: {}", responce[1]["_source"]["layers"]["png_raw"][0]);

    let image = &responce[1]["_source"]["layers"]["png_raw"][0].as_str().unwrap();

    let buffer = hex::decode(image).expect("Decoding failed");

    println!("responce2 : {:?}", &buffer);

    image::save_buffer("image.png", &buffer, 120, 30, image::ColorType::Rgb8).unwrap();

}
