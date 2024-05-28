use image::{io::Reader as ImageReader, ImageBuffer, Pixel, Rgb, Rgb32FImage, RgbImage};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn safe_get_pixel(img: &ImageBuffer<Rgb<f32>, Vec<f32>>, x: i32, y: i32) -> Rgb<f32> {
    let width = img.width();
    let height = img.height();
    *img.get_pixel(
        x.clamp(0, width as i32 - 1) as u32,
        y.clamp(0, height as i32 - 1) as u32,
    )
}

fn save_image(img: ImageBuffer<Rgb<f32>, Vec<f32>>, path: &str) -> Result<()> {
    let img = RgbImage::from_fn(img.width(), img.height(), |x, y| {
        let pixel = img.get_pixel(x, y);
        Rgb([
            (pixel[0] * 255.0).round() as u8,
            (pixel[1] * 255.0).round() as u8,
            (pixel[2] * 255.0).round() as u8,
        ])
    });
    println!("Saving image to {}", path);
    img.save(path)?;

    Ok(())
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: {} <image_path> <iterations> <ratio>", args[0]);
        std::process::exit(1);
    }

    let img_path = &args[1];
    let iterations = args[2]
        .parse::<u32>()
        .expect("Iterations must be an integer");
    let ratio = args[3].parse::<i32>().expect("Ratio must be an integer");

    println!("Blurring image {} with {} iterations", img_path, iterations);
    let img = ImageReader::open(img_path).unwrap().decode().unwrap();
    let mut img_buf = img.to_rgb32f();
    let mut new_img = Rgb32FImage::new(img.width(), img.height());
    for _ in 0..iterations {
        for y in 0..img.height() {
            for x in 0..img.width() {
                let mut r = 0.0;
                let mut g = 0.0;
                let mut b = 0.0;
                for i in -ratio..=ratio {
                    for j in -ratio..=ratio {
                        let pixel = safe_get_pixel(&img_buf, x as i32 + i, y as i32 + j);
                        r += pixel[0] * pixel[0];
                        g += pixel[1] * pixel[1];
                        b += pixel[2] * pixel[2];
                    }
                }
                let average_r = (r / 9.0).sqrt();
                let average_g = (g / 9.0).sqrt();
                let average_b = (b / 9.0).sqrt();
                new_img.put_pixel(x, y, Rgb::<f32>([average_r, average_g, average_b]));
            }
        }
        img_buf = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
            new_img.get_pixel(x, y).to_rgb()
        });
    }

    let file_name = img_path
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap();

    save_image(new_img, format!("{}_blurred.jpg", file_name).as_str())?;

    Ok(())
}
