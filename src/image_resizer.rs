use clap::Args;
use image::GenericImageView;
use std::io;

#[derive(Args)]
pub struct ResizeOptions {
    #[arg(short = 'i', long, help = "Path to the input image")]
    pub input: String,

    #[arg(short = 'o', long, help = "Path to save the resized image")]
    pub output: String,

    #[arg(short = 'w', long, help = "Width of the resized image (optional)")]
    pub width: Option<u32>,

    #[arg(short = 'e', long, help = "Height of the resized image (optional)")]
    pub height: Option<u32>,

    #[arg(short = 's', long, help = "Scale factor for resizing (optional)")]
    pub scale: Option<f32>,
}

pub fn run_image_resizer(options: ResizeOptions) {
    match resize_image(&options) {
        Ok(_) => println!("Image resized successfully! Saved to {}", options.output),
        Err(e) => eprintln!("Error resizing image: {}", e),
    }
}

fn resize_image(options: &ResizeOptions) -> Result<(), io::Error> {
    // Load the input image
    let img = image::open(&options.input).map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    // Determine new dimensions
    let (new_width, new_height) = match (options.width, options.height, options.scale) {
        (Some(w), Some(h), _) => (w, h), // Width and height specified
        (None, None, Some(scale)) => {
            let (orig_width, orig_height) = img.dimensions();
            (
                (orig_width as f32 * scale) as u32,
                (orig_height as f32 * scale) as u32,
            )
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid dimensions or scale provided",
            ));
        }
    };

    // Resize the image
    let resized_img = img.resize_exact(new_width, new_height, image::imageops::Lanczos3);

    // Save the resized image
    resized_img
        .save(&options.output)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    Ok(())
}
