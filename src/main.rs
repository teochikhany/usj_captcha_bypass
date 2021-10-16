extern crate image;


use image::open;
use std::fs::File;
use std::io::prelude::*;
use clipboard_win::{formats, get_clipboard};
// use image::{RgbImage, Rgb, ImageFormat};

type IMAGE = image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;

const BLACK_PIXEL: image::Rgb<u8> = image::Rgb([51u8, 51u8, 51u8]);


fn main() {

    // Getting the image saved in the Clipboard
    let bitmap: Vec<u8> = get_clipboard(formats::Bitmap).expect("Not bitmap format");

    // Create the image on disk and write the clipboard data to it
    let mut file = File::create("test.bmp").expect("Could not create file 'test.bmp' ");
    file.write_all(&bitmap).expect("Could not write data into 'test.bmp' ");
    
    // open the image for processing
    let img = open("test.bmp").expect("can not find the image").into_rgb8();

    // get an array usize 5, each index indicating the number of 
    // black pixels for each number
    let nb_black = read_image(&img);

    for i in 0 .. 5
    {
        get_nb(nb_black[i], &img, i as u32);
    }
}




/// Count how many black pixels vertically, from row 10 to 20 and adding 20 rows for each number
/// until 110 th row. The first 10 and last 10 rows are neglected
fn read_image(img : &IMAGE) -> [u16; 5]
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);

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
            if img.get_pixel(i, j) == &BLACK_PIXEL
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
            if img.get_pixel(i, j) == &BLACK_PIXEL
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
            if img.get_pixel(i, j) == &BLACK_PIXEL
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
            if img.get_pixel(i, j) == &BLACK_PIXEL
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
            if img.get_pixel(i, j) == &BLACK_PIXEL
            {
                // println!("{}, {}", i, j);
                fifth_nb += 1;
            }
        }
    }

    println!("first: {}", first_nb);
    println!("second: {}", second_nb);
    println!("third: {}", third_nb);
    println!("forth: {}", forth_nb);
    println!("fifth: {} \n", fifth_nb);

    return [first_nb, second_nb, third_nb, forth_nb, fifth_nb];
}


fn get_first_pixel(img: &IMAGE, iter:u32) -> (u32, u32)
{
    let lower_bound = 10 + 20 * iter;
    let upper_bound = lower_bound + 20;

    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);

    let mut first_black = (0, 0);

    'outer: for i in lower_bound .. upper_bound
    {
        for j in 0 .. 30
        {
            if img.get_pixel(i, j) == &BLACK_PIXEL
            {
                first_black = (i, j);
                break 'outer;
            }
        }
    }

    return first_black;
}


fn get_nb(nb_black : u16, img: &IMAGE, iter:u32)
{
    match nb_black
    {
        36 => println!("0"),
        16 => println!("1"),
        21 => println!("2"),
        37 => println!("5"),
        15 => println!("7"),
        40 => println!("8"),
        29 => println!("5"),
        20 => or_0_7_4(img, iter),
        27 => or_1_2_4(img, iter),
        39 => or_9_6(img, iter),
        32 => or_2_8(img, iter),
        31 => or_3_9_6(img, iter),
        22 => or_3_9(img, iter),
        23 => or_5_6(img, iter),
        26 => or_3_8_7(img, iter),
        _ => println!("unknows nb of black pixel: {}", nb_black)
    }
}


fn or_0_7_4(img: &IMAGE, iter:u32)
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 4, first_black.1 - 4) == &BLACK_PIXEL 
    {
        print!("4 \t")
    }
    else if img.get_pixel(first_black.0 + 3, first_black.1 + 3) == &BLACK_PIXEL 
    {
        print!("7 \t")
    }
    else
    {
        print!("0 \t")
    }

    println!("either _ or _ or _ (4, 7, 0)");
}


fn or_1_2_4(img: &IMAGE, iter:u32)
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 4, first_black.1 - 4) == &BLACK_PIXEL
    {
        println!("4 \t")
    }
    else if img.get_pixel(first_black.0 + 2, first_black.1 - 2) == &BLACK_PIXEL
        && img.get_pixel(first_black.0 + 1, first_black.1 - 1) == &BLACK_PIXEL
    {
        println!("1 \t")
    }
    else if img.get_pixel(first_black.0 + 2, first_black.1 - 2) != &BLACK_PIXEL
                && img.get_pixel(first_black.0 + 1, first_black.1 - 1) == &BLACK_PIXEL
    {
        println!("2 \t")
    }

    // println!("either _ or 4 (1, 2)");
}


fn or_2_8(img: &IMAGE, iter:u32)
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 7, first_black.1 + 7) == &BLACK_PIXEL 
    {
        println!("2 \t")
    }
    else
    {
        println!("8 \t")
    }

    // println!("either _ or 8 (2)");
}


#[allow(unused_variables)]
fn or_3_9(img: &IMAGE, iter:u32)
{
    println!("either 3 or 9");
}

#[allow(unused_variables)]
fn or_3_9_6(img: &IMAGE, iter:u32)
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 5, first_black.1) == &BLACK_PIXEL 
        && img.get_pixel(first_black.0 + 5, first_black.1 + 1) != &BLACK_PIXEL 
    {
        println!("6 \t")
    }
    else if img.get_pixel(first_black.0 + 5, first_black.1) == &BLACK_PIXEL 
            && img.get_pixel(first_black.0 + 5, first_black.1 + 1) == &BLACK_PIXEL 
    {
        println!("9 \t")
    }
    else
    {
        println!("3 \t")
    }

    // println!("either 3 or 9 or _ (6)");
}


fn or_5_6(img: &IMAGE, iter:u32)
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0, first_black.1 + 4) == &BLACK_PIXEL 
        && img.get_pixel(first_black.0, first_black.1 + 5) != &BLACK_PIXEL
    {
        println!("5 \t")
    }
    else
    {
        println!("6 \t")
    }

    // println!("either _ or 6 (5)");
}


fn or_3_8_7(img: &IMAGE, iter:u32)
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 5, first_black.1 + 4) == &BLACK_PIXEL 
        && img.get_pixel(first_black.0 + 6, first_black.1 + 3) == &BLACK_PIXEL 
    {
        println!("7 \t")
    }
    else if img.get_pixel(first_black.0 + 1, first_black.1) == &BLACK_PIXEL 
    {
        println!("3 \t")
    }
    else
    {
        println!("8 \t")
    }

    // println!("either _ or 8 (3)");
}


fn or_9_6(img: &IMAGE, iter:u32)
{
    // let black_pixel = image::Rgb([51u8, 51u8, 51u8]);
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 6, first_black.1 + 1) != &BLACK_PIXEL 
    {
        println!("6 \t")
    }
    else
    {
        println!("9 \t")
    }

    // println!("either 9 or 6");
}