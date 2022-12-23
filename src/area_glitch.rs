use image::{DynamicImage, GenericImage, GenericImageView};

/// Rectangular area of an image
#[derive(Debug)]
struct Area {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl Area {
    /// Creates a new random area within certain bounds
    fn new_random(dims: (u32, u32), min: f32, max: f32) -> Area {
        let mut area = Area {
            x: (rand::random::<f32>() * dims.0 as f32) as u32,
            y: (rand::random::<f32>() * dims.1 as f32) as u32,
            w: (min + (rand::random::<f32>() * (max - min)) * dims.0 as f32) as u32,
            h: (min + (rand::random::<f32>() * (max - min)) * dims.1 as f32) as u32,
        };

        if (area.x + area.w) > dims.0 {
            area.w -= (area.x + area.w) - dims.0;
        }

        if (area.y + area.h) > dims.1 {
            area.y -= (area.y + area.h) - dims.1;
        }

        area
    }
}

///
/// This function cuts out and rearranges various rectangles of pixels (of different sizes) within the image
///
pub fn shuffle_areas(img: &mut DynamicImage, num_times: u32) {
    let dims = img.dimensions();

    for _i in 0..num_times {
        let mut area = Area::new_random(dims, 0.05, 0.7);
        let crop = img.crop_imm(area.x, area.y, area.w, area.h);

        area.x = (rand::random::<f32>() * dims.0 as f32) as u32;
        area.y = (rand::random::<f32>() * dims.1 as f32) as u32;

        for src_y in 0..area.h {
            for src_x in 0..area.w {
                let x = area.x + src_x;
                let y = area.y + src_y;

                if x < dims.0 && y < dims.1 {
                    img.put_pixel(x, y, crop.get_pixel(src_x, src_y));
                }
            }
        }
    }
}

///
/// This function slides rectangles of pixels diagonally in a direction
///
pub fn slide_areas(img: &mut DynamicImage, num_times: u32, max_slide: u32) {
    let dims = img.dimensions();

    for _i in 0..num_times {
        let dir_x = (rand::random::<f32>() - 0.5).signum() as i32;
        let dir_y = (rand::random::<f32>() - 0.5).signum() as i32;
        let slide_length = ((max_slide as f32) * rand::random::<f32>()) as u32;

        let mut area = Area::new_random(dims, 0.05, 0.7);
        let crop = img.crop_imm(area.x, area.y, area.w, area.h);

        let t_crop = crop.crop_imm(0, 0, crop.width(), 1);
        let b_crop = crop.crop_imm(0, (crop.height() as i32 - 1).max(0) as u32, crop.width(), 1);
        let l_crop = crop.crop_imm(0, 0, 1, crop.height());
        let r_crop = crop.crop_imm((crop.width() as i32 - 1).max(0) as u32, 0, 1, crop.height());

        // copy the top, bottom and side pixels to the new start pos
        for _s in 0..slide_length {
            let top = area.y;
            let bottom = area.y + area.h;
            let left = area.x;
            let right = area.x + area.w;

            if top >= dims.1
                || top == 0
                || bottom >= dims.1
                || bottom == 0
                || left >= dims.0
                || left == 0
                || right >= dims.0
                || right == 0
            {
                break;
            }

            if t_crop.height() > 0 {
                for x in 0..t_crop.width() {
                    if (left + x) < dims.0 {
                        img.put_pixel(left + x, top, t_crop.get_pixel(x, 0));
                    }
                }
            }
            if b_crop.height() > 0 {
                for x in 0..b_crop.width() {
                    if (left + x) < dims.0 {
                        img.put_pixel(left + x, bottom, b_crop.get_pixel(x, 0));
                    }
                }
            }

            if l_crop.width() > 0 {
                for y in 0..l_crop.height() {
                    if (top + y) < dims.1 {
                        img.put_pixel(left, top + y, l_crop.get_pixel(0, y));
                    }
                }
            }
            if r_crop.width() > 0 {
                for y in 0..r_crop.height() {
                    if (top + y) < dims.1 {
                        img.put_pixel(right, top + y, r_crop.get_pixel(0, y));
                    }
                }
            }

            // shift the rectangle
            let new_x = area.x as i32 + dir_x;
            if new_x >= 0 && new_x < dims.0 as i32 {
                area.x = new_x as u32;
            }

            let new_y = area.y as i32 + dir_y;
            if new_y >= 0 && new_y < dims.1 as i32 {
                area.y = new_y as u32;
            }
        }

        // copy full image to final place at the end
        // for src_y in 0..area.h {
        //   for src_x in 0..area.w {

        //     let x = area.x + src_x;
        //     let y = area.y + src_y;

        //     if x < dims.0 && y < dims.1 {
        //       img.put_pixel(x, y, crop.get_pixel(src_x, src_y));
        //     }
        //   }
        // }
    }
}
