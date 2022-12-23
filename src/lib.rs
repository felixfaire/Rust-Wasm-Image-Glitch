use image::{self, DynamicImage, GenericImage, ImageBuffer, RgbaImage};
use wasm_bindgen::prelude::*;
use web_sys::console;

mod area_glitch;
mod pixel_sort;

///
/// ImageProcessor contains 2 copies of an image and
/// functionality to process the base image with several
/// pixel sorting effects.
///
#[wasm_bindgen]
pub struct ImageProcessor {
    base_image: DynamicImage,
    processed_image: DynamicImage,
}

#[wasm_bindgen]
impl ImageProcessor {
    /// Creates the image processor with image buffers of the correct size
    ///
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> ImageProcessor {
        ImageProcessor {
            base_image: DynamicImage::new_rgba8(width, height),
            processed_image: DynamicImage::new_rgba8(width, height),
        }
    }

    /// Loads an image from js into the image buffer memory
    ///
    pub fn load_base_image(&mut self, width: u32, height: u32, pixels: Vec<u8>) {
        let new_img: RgbaImage = ImageBuffer::from_vec(width, height, pixels).unwrap();

        self.base_image
            .copy_from(&new_img, 0, 0)
            .expect("image too large to copy");
        self.processed_image
            .copy_from(&new_img, 0, 0)
            .expect("image too large to copy");
    }

    /// Resets the processed image to be the same as the base image
    ///
    pub fn reset_image(&mut self) {
        self.processed_image
            .copy_from(&self.base_image, 0, 0)
            .expect("image too large to copy");
    }

    /// Applies the pixel sort effect to the processed image
    ///
    pub fn process_pixel_sort(&mut self, thresh: u8, vertical: bool) {
        console::time_with_label("Sorting Pixels");
        if vertical {
            pixel_sort::sort_pixels_vert(&mut self.processed_image, thresh);
        } else {
            pixel_sort::sort_pixels_horiz(&mut self.processed_image, thresh);
        }
        console::time_end_with_label("Sorting Pixels");
    }

    /// Applies the area shuffle effect to the processed image
    ///
    pub fn process_area_shuffle(&mut self, num_iterations: u32) {
        console::time_with_label("Shuffling areas");
        area_glitch::shuffle_areas(&mut self.processed_image, num_iterations);
        console::time_end_with_label("Shuffling areas");
    }

    /// Applys the rectangle slide effect to the processed image
    ///
    pub fn process_rect_slide(&mut self, num_times: u32, distance: u32) {
        console::time_with_label("Sliding Rects");
        area_glitch::slide_areas(&mut self.processed_image, num_times, distance);
        console::time_end_with_label("Sliding Rects");
    }

    /// Returns the image buffer as a Vec<u8> to be consumed by js
    ///
    pub fn get_pixel_data(&self) -> Vec<u8> {
        if let DynamicImage::ImageRgba8(buffer) = &self.processed_image {
            buffer.to_vec()
        } else {
            vec![0]
        }
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    console_error_panic_hook::set_once();

    // Print line to js to confirm the lib has loaded successfully
    console::log_1(&JsValue::from_str("Rust library loaded!"));

    Ok(())
}
