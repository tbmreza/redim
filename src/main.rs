extern crate image;
use image::imageops::FilterType;
use image::GenericImageView;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input_file = &args[1];
    let divisor = &args[2];
    let divisor = divisor.parse::<u32>().unwrap();

    if args.len() == 3 {
        // Shrink dimension by a divisor.
        redimension(divisor, &input_file, false);
    } else if args.len() == 4 && &args[3] == "--resize" {
        // In attempt to reduce file size, shrink dimension by a divisor then expand.
        resize(divisor, &input_file, true);
    } else {
        // Wrong usage
        println!("To shrink image dimension: \n\tredim input.jpeg [divisor]\n");
        println!("To reduce image file size: \n\tredim input.jpeg [divisor] --resize");
    }
}
fn redimension(divisor: u32, input_file: &str, flag: bool) -> (image::DynamicImage, u32, u32) {
    let img = image::open(input_file).unwrap();
    let img_w = img.width();
    let img_h = img.height();

    let redim_img = img.resize(img_w / divisor, img_h / divisor, FilterType::Nearest);
    if !flag {
        // Skip saving redim file if (resize) flag is set true.
        let mut y = std::fs::File::create(format!("rdim_{}", input_file)).unwrap();
        redim_img
            .write_to(&mut y, parse_image_format(input_file))
            .unwrap();
    }
    (redim_img, img_w, img_h)
}
fn resize(divisor: u32, input_file: &str, flag: bool) {
    let (redim_img, img_w, img_h) = redimension(divisor, input_file, flag);

    let resized_img = redim_img.resize(img_w, img_h, FilterType::Gaussian);
    let output_path;
    if &input_file[(input_file.len() - 5)..] == ".jpeg" {
        output_path = format!("rsiz_{}", input_file);
    } else {
        output_path = format!("rsiz_{}.jpeg", input_file);
    }

    let mut z = std::fs::File::create(output_path).unwrap();
    resized_img
        .write_to(&mut z, image::ImageFormat::Jpeg)
        .unwrap();
}

fn parse_image_format(filename: &str) -> image::ImageFormat {
    let mut tail = vec![];
    for c in filename.chars().rev() {
        tail.push(c);
        if c == '.' {
            break;
        }
    }
    let tail: String = tail.iter().collect();
    let format: &str = &tail;
    match format {
        // Comment formats I didn't test. (or want to)
        // Lossy: PNG (transparency), WebP (color), and possibly others.
        ".png" => image::ImageFormat::Png,
        ".jpeg" => image::ImageFormat::Jpeg,
        ".jpg" => image::ImageFormat::Jpeg,
        // ".gif" => image::ImageFormat::Gif,
        ".webp" => image::ImageFormat::WebP,
        // ".pnm" => image::ImageFormat::Pnm,
        // ".tiff" => image::ImageFormat::Tiff,
        // ".tif" => image::ImageFormat::Tiff,
        // ".tga" => image::ImageFormat::Tga,
        // ".dds" => image::ImageFormat::Dds,
        // ".bmp" => image::ImageFormat::Bmp,
        // ".ico" => image::ImageFormat::Ico,
        // ".hdr" => image::ImageFormat::Hdr,
        // ".farbfeld" => image::ImageFormat::Farbfeld,
        _ => {
            // Fallback to JPEG
            image::ImageFormat::Jpeg
        }
    }
}
