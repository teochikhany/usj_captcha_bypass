extern crate image;

use image::open;
use std::fs::File;
use std::io::prelude::*;

use clipboard_win::{formats, get_clipboard, set_clipboard_string};

/// just an alias to simplify image parameters
type IMAGE = image::ImageBuffer<image::Rgb<u8>, std::vec::Vec<u8>>;

/// what values represent a black pixel in the captcha
const BLACK_PIXEL: image::Rgb<u8> = image::Rgb([51u8, 51u8, 51u8]);


fn main() {

    // Getting the image saved in the Clipboard
    let bitmap: Vec<u8> = get_clipboard(formats::Bitmap).expect("Not bitmap format");

    // Create the image on disk 
    let mut file = File::create("test.bmp").expect("Could not create file 'test.bmp' ");
    
    // And write the clipboard data to it
    file.write_all(&bitmap).expect("Could not write data into 'test.bmp' ");
    
    // open the image as rgb8 for processing
    let img = open("test.bmp").expect("can not find the image").into_rgb8();

    // get an array of length 5, each index indicating the number of 
    // black pixels for each number
    let nb_black = read_image(&img);

    let mut result: String = "".to_owned();

    for i in 0 .. 5
    {
        let nb : &str = get_nb(nb_black[i], &img, i as u32);
        result.push_str(nb);
    }

    println!("result: {}", result);

    set_clipboard_string(&result).expect("could not save result into clipboard");
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


/// Getting the first black pixel, top -> down, left -> right
/// in the specified section (iter: u32), each captcha is
/// divided into 5 section, one for each number. 
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


/// Determening wich number is in the current section
/// depending on the numbers of black pixel in each section
/// and the postion of pixel depending on the first black pixel
fn get_nb(nb_black : u16, img: &IMAGE, iter:u32) -> &str
{
    match nb_black
    {
        36 => "0",
        16 => "1",
        21 => "2",
        37 => "5",
        15 => "7",
        40 => "8",
        29 => "5",
        20 => or_0_7_4(img, iter),
        27 => or_1_2_4(img, iter),
        39 => or_9_6(img, iter),
        32 => or_2_8(img, iter),
        31 => or_3_9_6(img, iter),
        22 => or_3_9(img, iter),
        23 => or_5_6(img, iter),
        26 => or_3_8_7(img, iter),
        _ => { println!("unknows nb of black pixel: {}", nb_black); return "R" }
    }
}


fn or_0_7_4(img: &IMAGE, iter:u32) -> &str
{
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 4, first_black.1 - 4) == &BLACK_PIXEL 
    {
        return "4";
    }
    else if img.get_pixel(first_black.0 + 3, first_black.1 + 3) == &BLACK_PIXEL 
    {
        return "7";
    }
    else
    {
        return "0";
    }

    // println!("either _ or _ or _ (4, 7, 0)");
}


fn or_1_2_4(img: &IMAGE, iter:u32) -> &str
{
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 4, first_black.1 - 4) == &BLACK_PIXEL
    {
        return "4"
    }
    else if img.get_pixel(first_black.0 + 2, first_black.1 - 2) == &BLACK_PIXEL
        && img.get_pixel(first_black.0 + 1, first_black.1 - 1) == &BLACK_PIXEL
    {
        return "1"
    }
    else if img.get_pixel(first_black.0 + 2, first_black.1 - 2) != &BLACK_PIXEL
                && img.get_pixel(first_black.0 + 1, first_black.1 - 1) == &BLACK_PIXEL
    {
        return "2"
    }

    return "X"
}


fn or_2_8(img: &IMAGE, iter:u32) -> &str
{
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 7, first_black.1 + 7) == &BLACK_PIXEL 
    {
        return "2"
    }
    else
    {
        return "8"
    }
}


#[allow(unused_variables)]
fn or_3_9(img: &IMAGE, iter:u32) -> &str
{
    return "Y"
}


fn or_3_9_6(img: &IMAGE, iter:u32) -> &str
{
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 5, first_black.1) == &BLACK_PIXEL 
        && img.get_pixel(first_black.0 + 5, first_black.1 + 1) != &BLACK_PIXEL 
    {
        return "6"
    }
    else if img.get_pixel(first_black.0 + 5, first_black.1) == &BLACK_PIXEL 
            && img.get_pixel(first_black.0 + 5, first_black.1 + 1) == &BLACK_PIXEL 
    {
        return "9"
    }
    else
    {
        return "3"
    }
}


fn or_5_6(img: &IMAGE, iter:u32) -> &str
{
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0, first_black.1 + 4) == &BLACK_PIXEL 
        && img.get_pixel(first_black.0, first_black.1 + 5) != &BLACK_PIXEL
    {
        return "5"
    }
    else
    {
        return "6"
    }
}


fn or_3_8_7(img: &IMAGE, iter:u32) -> &str
{
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 5, first_black.1 + 4) == &BLACK_PIXEL 
        && img.get_pixel(first_black.0 + 6, first_black.1 + 3) == &BLACK_PIXEL 
    {
        return "7"
    }
    else if img.get_pixel(first_black.0 + 1, first_black.1) == &BLACK_PIXEL 
    {
        return "3"
    }
    else
    {
        return "8"
    }
}


fn or_9_6(img: &IMAGE, iter:u32) -> &str
{
    let first_black = get_first_pixel(img, iter);

    if img.get_pixel(first_black.0 + 6, first_black.1 + 1) != &BLACK_PIXEL 
    {
        return "6"
    }
    else
    {
        return "9"
    }
}



#[cfg(test)]
mod captcha
{
    use super::*;

    fn _main(img: IMAGE) -> String
    {
        let nb_black = read_image(&img);
        let mut result: String = "".to_owned();

        for i in 0 .. 5
        {
            let nb : &str = get_nb(nb_black[i], &img, i as u32);
            result.push_str(nb);
        }

        return result;
    }

    #[test]
    fn _1_15524()
    {
        let img = open(r".\captchas\1-15524.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "15524");
    }

    #[test]
    fn _2_31868()
    {
        let img = open(r".\captchas\2-31868.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "31868");
    }


    #[test]
    fn _3_36152()
    {
        let img = open(r".\captchas\3-36152.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "36152");
    }



    #[test]
    fn _4_46694()
    {
        let img = open(r".\captchas\4-46694.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "46694");
    }


    #[test]
    fn _5_58865()
    {
        let img = open(r".\captchas\5-58865.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "58865");
    }


    #[test]
    fn _6_77164()
    {
        let img = open(r".\captchas\6-77164.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "77164");
    }

    #[test]
    fn _7_27361()
    {
        let img = open(r".\captchas\7-27361.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "27361");
    }

    #[test]
    fn _8_63124()
    {
        let img = open(r".\captchas\8-63124.png").expect("can not find the image").into_rgb8();

        let result = _main(img);

        assert_eq!(result, "63124");
    }

    
}
