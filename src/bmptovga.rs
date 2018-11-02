extern crate stb_image;
extern crate exoquant;

#[path = "vgapalette.rs"]
pub mod vgapalette;

use stb_image::image;
use stb_image::image::LoadResult;
use stb_image::image::LoadResult::*;

use exoquant::*;

pub fn bmp_to_vga(image: &Vec<u8>) -> Vec<u8>
{
    let vga_colors = vgapalette::get_vga_colors();
    let mut exocolors: Vec<exoquant::Color> = Vec::new();

    for i in 0..65536 
    {
        let r = image[i*3];
        let g = image[i*3+1];
        let b = image[i*3+2];

        let mut c = Colorf::zero();
        c.r = r as f64;
        c.g = g as f64;
        c.b = b as f64;

        exocolors.push(exoquant::Color::new(r, g, b, 0));
    }

    let colorspace = SimpleColorSpace::default();
    let vga_ditherer = exoquant::ditherer::FloydSteinberg::new();
    let vga_remapper = exoquant::Remapper::new(&vga_colors, &colorspace, &vga_ditherer);
    let indexed_image_data = vga_remapper.remap(&exocolors, 256);

    indexed_image_data
}
