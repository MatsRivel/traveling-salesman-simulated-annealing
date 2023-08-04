use std::{collections::HashMap, fs::File, borrow::Cow};
use image::{ImageBuffer,RgbImage,Rgb};
use gif::{Frame,Encoder,Repeat};

use crate::constants::{MAX_Y, MAX_X};

fn surround_pixel( img:&mut RgbImage, x:u32, y:u32, color: [u8;3]){
    let surround_color = [50u8,50u8,50u8];
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
    match pixel.0{
        [0u8,0u8,0u8] => *pixel = Rgb(color),
        surround_color => *pixel = Rgb(color),
        _ => ()
    }
}
pub fn make_image(mut img:RgbImage,node_coords:HashMap<i32,[i32;2]>, color: [u8;3], save_image:bool) -> RgbImage{
    for (idx, (_node_nr, coord)) in node_coords.iter().enumerate(){
        let (x,y) = (1 + coord[0].abs() as u32, 1+ coord[1].abs() as u32); // TODO: This is a bandaid in case of negative coordinates...
        surround_pixel(&mut img, x, y, color);
        if save_image{
            img.save(format!("test{idx}.png")).unwrap();
        }
    }


    img
}
 
pub fn make_gif(mut img:RgbImage,node_coords:HashMap<i32,[i32;2]>, color: [u8;3], save_image:bool) -> RgbImage{
    let color_map = &[0xFF, 0xFF, 0xFF, 100, 200, 0];
    let mut gif_file = File::create("target/test.gif").unwrap();
    let mut encoder = Encoder::new(&mut gif_file, MAX_Y as u16, MAX_X as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for (_node_nr, coord) in node_coords{
        let (x,y) = (1 + coord[0].abs() as u32, 1+ coord[1].abs() as u32); // TODO: This is a bandaid in case of negative coordinates...
        surround_pixel(&mut img, x, y, color);
        if save_image{
            let mut frame = Frame::default();
            let img_clone = img.clone();
            let state = img_clone.into_raw();
            frame.width = MAX_Y as u16;
            frame.height = MAX_X as u16;
            frame.buffer = Cow::Borrowed(&*state);   
            encoder.write_frame(&frame).unwrap();         
        }
    }

    img
}