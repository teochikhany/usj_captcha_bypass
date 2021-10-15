extern crate image;
extern crate hex;

use std::process::Command;
use std::str;
use wait_timeout::ChildExt;
use std::time::Duration;
use image::open;
use std::fs::File;
use std::io::prelude::*;


fn main() {
    
    // println!("Hello, world!");

    // start_capture();

    // println!("finished waiting");

    // let tshark_analyse = parse_pcap();

    // extract_image(tshark_analyse);

    let nb_black = read_image();

    for i in 0 .. 5
    {
        get_nb(nb_black[i]);
    }
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

/// Count how many black pixels vertically, from row 10 to 20 and adding 20 rows for each number
/// until 110 th row. The first 10 and last 10 rows are neglected
fn read_image() -> [u16; 5]
{
    let img = open("captcha.php.png").expect("can not find the image").into_rgb8();

    let black_pixel = image::Rgb([51u8, 51u8, 51u8]);

    let mut first_nb:   u16 = 0;
    let mut second_nb:  u16 = 0;
    let mut third_nb:   u16 = 0;
    let mut forth_nb:   u16 = 0;
    let mut fifth_nb:   u16 = 0;

    // first number
    for i in 10 .. 30
    {
        for j in 0 .. 30
        {
            if img.get_pixel(i, j) == &black_pixel
            {
                first_nb += 1;
            }
        }
    }

    // second number
    for i in 30 .. 50
    {
        for j in 0 .. 30
        {
            if img.get_pixel(i, j) == &black_pixel
            {
                second_nb += 1;
            }
        }
    }

    // third number
    for i in 50 .. 70
    {
        for j in 0 .. 30
        {
            if img.get_pixel(i, j) == &black_pixel
            {
                third_nb += 1;
            }
        }
    }

    // forth number
    for i in 70 .. 90
    {
        for j in 0 .. 30
        {
            if img.get_pixel(i, j) == &black_pixel
            {
                forth_nb += 1;
            }
        }
    }


    // fifth number
    for i in 90 .. 110
    {
        for j in 0 .. 30
        {
            if img.get_pixel(i, j) == &black_pixel
            {
                // println!("{}, {}", i, j);
                fifth_nb += 1;
            }
        }
    }

    // println!("first: {}", first_nb);
    // println!("second: {}", second_nb);
    // println!("third: {}", third_nb);
    // println!("forth: {}", forth_nb);
    // println!("fifth: {}", fifth_nb);

    return [first_nb, second_nb, third_nb, forth_nb, fifth_nb];
}


fn get_nb(nb_black : u16)
{
    match nb_black
    {
        36 => println!("0"),
        16 => println!("1"),
        21 => println!("2"),
        37 => println!("5"),
        15 => println!("7"),
        40 => println!("8"),
        39 => println!("9"),
        20 => or_0_7(),
        27 => or_1_2_4(),
        32 => or_2_8(),
        31 => or_3_9(),
        22 => or_3_9(),
        23 => or_5_6(),
        _ => println!("unknows nb of black pixel: {}", nb_black)
    }
}


fn or_0_7()
{
    println!("either 0 or 7");
}


fn or_1_2_4()
{
    println!("either 1 or 2 or 4");
}


fn or_2_8()
{
    println!("either 2 or 8");
}

fn or_3_9()
{
    println!("either 3 or 9");
}

fn or_5_6()
{
    println!("either 5 or 6");
}