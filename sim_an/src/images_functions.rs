use std::collections::HashMap;
use image::{ImageBuffer,RgbImage,Rgb};

pub struct ImageSizes{
    pub max_x:u32,
    pub max_y:u32,
    pub min_x:u32,
    pub min_y:u32
}

fn surround_pixel( img:&mut RgbImage, x:u32, y:u32, color: [u8;3], surround_color:[u8;3]){
    for x0 in (x-1)..=(x+1){
        for y0 in (y-1)..=(y+1){
            let pixel = img.get_pixel_mut(x0, y0);
            match pixel{
                Rgb([0u8,0u8,0u8]) => *pixel = Rgb(surround_color),
                _ => ()
            }
        }
    }
    let mut pixel = img.get_pixel_mut(x, y);
    match pixel{
        Rgb([0u8,0u8,0u8]) => *pixel = Rgb(color),
        Rgb(surround_color) => *pixel = Rgb(color),
        _ => ()
    }
}
pub fn make_image(mut img:RgbImage,node_coords:HashMap<i32,[i32;2]>, color: [u8;3], save_image:bool) -> RgbImage{
    for (_node_nr, coord) in node_coords{
        let (x,y) = (1 + coord[0].abs() as u32, 1+ coord[1].abs() as u32); // TODO: This is a bandaid in case of negative coordinates...

    }
    if save_image{
        img.save("test.png").unwrap();
    }
    img
}