extern crate image;
extern crate hex;

use std::process::Command;
use std::str;
use wait_timeout::ChildExt;
use std::time::Duration;
// use image::io::Reader as ImageReader;
// use image::DynamicImage;
// use image::ImageBuffer;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    
    println!("Hello, world!");

    start_capture();

    println!("finished waiting");

    let tshark_analyse = parse_pcap();

    extract_image(tshark_analyse);

}

/// Start the capture with tshark using the following command :
///     .\tshark.exe -i wifi -w AAA.pcap -f "tcp and host 193.227.187.169"
fn start_capture()
{
    let _tshark_capture = Command::new("D:\\Apps\\wireshark\\tshark.exe")
                            .args(["-i", "wifi", "-w", "BAA.pcap", "-f", "tcp and host 193.227.187.169" ])
                            .spawn()
                            .expect("tshark command failed to start")
                            .wait_timeout(Duration::from_secs(10));
}


/// Parse the pcap file with tshark using the following command
/// D:\\Apps\\wireshark\\tshark.exe -r .\BAA.pcap -T json -Y 'http' -x > test.json
fn parse_pcap() -> std::process::Output
{
    return Command::new("D:\\Apps\\wireshark\\tshark.exe")
                    .args(["-r", "BAA.pcap", "-T", "json", "-Y", "http", "-x"])
                    .output()
                    .expect("tshark command failed to start");
}


/// read the json file parsed from the pcap and 
/// save the latest image with the dimentions of 120x30
fn extract_image(tshark_analyse: std::process::Output)
{
    let res = str::from_utf8(&tshark_analyse.stdout).expect("could not cast output as utf8");

    let responce = json::parse(res).expect("could not parse the json file");

    // println!("responce: {}", responce[1]["_source"]["layers"]["png_raw"][0]);

    let image = &responce[1]["_source"]["layers"]["png_raw"][0].as_str().unwrap();

    let buffer = hex::decode(image).expect("Decoding failed");

    // println!("responce2 : {:?}", &buffer);

    let mut file = File::create("captcha.png").expect("could do create file");
    file.write_all(&buffer).expect("could not write to file");
}