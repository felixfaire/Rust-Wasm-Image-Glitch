use image::{DynamicImage, GenericImage, GenericImageView, GrayImage, Pixel};

/// Horizontal strip of pixels in an image to be sorted
#[derive(Debug, Clone, Copy)]
pub struct PixelSortRange {
    pub row: u32,
    pub start: u32,
    pub end: u32,
}

///
/// Generate a vec of pixel strips to sort based on a greyscale threshold
///
pub fn get_pixel_sort_ranges(grey_img: &GrayImage, thresh: u8) -> Vec<PixelSortRange> {
    let mut pixel_ranges = vec![];
    let (w, h) = grey_img.dimensions();

    for y in 0..h {
        let mut prev_black = true;
        let mut pix_range = PixelSortRange {
            row: y,
            start: 0,
            end: 0,
        };

        for x in 0..w {
            let p = grey_img.get_pixel(x, y);
            let current_black = p[0] <= thresh;

            // find start of a 'black' range
            if current_black && (!prev_black || x == 0) {
                pix_range.start = x;
            }

            // find end of a 'black' range (or end of row)
            if prev_black && (!current_black || x == w - 1) {
                pix_range.end = x;

                if (pix_range.end - pix_range.start) > 2 {
                    pixel_ranges.push(pix_range);
                }
                pix_range.start = x + 1;
            }

            prev_black = current_black;
        }
    }

    pixel_ranges
}

///
/// Sorts the pixels in an image horizontally using a greyscale threshold
///
pub fn sort_pixels_horiz(img: &mut DynamicImage, thresh: u8) {
    let gray_img = img.to_luma8();

    // Get pixel sort ranges for each row
    let sort_ranges = get_pixel_sort_ranges(&gray_img, thresh);

    // Preallocate a list within which to sort a strip of pixels
    let mut sorted_pixels = Vec::with_capacity(img.width() as usize);

    // Iterate sort ranges and sort pixels
    for r in sort_ranges {
        let y = r.row;

        sorted_pixels.clear();

        // Copy pixels into sort range
        for x in r.start..=r.end {
            let pixel = img.get_pixel(x, y);
            sorted_pixels.push(pixel);
        }

        // sort pixels
        sorted_pixels.sort_by(|a, b| {
            let v1 = a.to_luma()[0];
            let v2 = b.to_luma()[0];
            v2.cmp(&v1)
        });

        // reinsert sorted pixels
        for x in r.start..=r.end {
            img.put_pixel(x, y, sorted_pixels[(x - r.start) as usize]);
        }
    }
}

///
/// Sorts the pixels of an image vertically using a greyscale threshold.
///
pub fn sort_pixels_vert(img: &mut DynamicImage, thresh: u8) {
    *img = img.rotate90();
    sort_pixels_horiz(img, thresh);
    *img = img.rotate270();
}
