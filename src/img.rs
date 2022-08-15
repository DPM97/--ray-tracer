use image::{ImageBuffer, Rgb, RgbImage};
use std::{
    io::{stdout, Write},
    sync::Mutex,
    thread::sleep,
    thread::spawn,
    time::Duration,
};

#[derive(Copy, Clone)]
struct Image {
    cur_x: u32,
    cur_y: u32,
    height: u32,
    width: u32,
}

pub fn create_png(out_path: std::path::PathBuf) {
    let img_data = Image {
        cur_x: 0,
        cur_y: 0,
        height: 1024,
        width: 1024,
    };

    let img_data_mutex: &'static Mutex<Image> = Box::leak(Box::new(Mutex::new(img_data)));
    spawn(move || {
        display_percent_done(&img_data_mutex, 500);
    });

    let mut buffer: RgbImage = ImageBuffer::new(img_data.width, img_data.height);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        let mut mutable_img_data = img_data_mutex.lock().unwrap();
        mutable_img_data.cur_x = x;
        mutable_img_data.cur_y = y;
        drop(mutable_img_data);

        let r = x as f64 / (img_data.width - 1) as f64;
        let g = y as f64 / (img_data.height - 1) as f64;
        let b = 0.25;

        let ir = (255.999 * r) as u8;
        let ig = (255.999 * g) as u8;
        let ib = (255.999 * b) as u8;

        *pixel = Rgb([ir, ig, ib]);
    }

    buffer.save(out_path.to_str().unwrap_or_default()).unwrap();
}

fn display_percent_done(data: &'static Mutex<Image>, delay_ms: u64) {
    let mut tmp_data = data.lock().unwrap();
    let total_pixels = tmp_data.width as u64 * tmp_data.height as u64;
    let mut cur_pixel = (tmp_data.cur_y as u64 * tmp_data.height as u64) + tmp_data.cur_x as u64;
    drop(tmp_data);
    while cur_pixel + 1 < total_pixels {
        tmp_data = data.lock().unwrap();
        let total_pixels = tmp_data.width as u64 * tmp_data.height as u64;
        cur_pixel = (tmp_data.cur_y as u64 * tmp_data.height as u64) + tmp_data.cur_x as u64;
        let mut stdout = stdout();
        let cur_pct = (cur_pixel as f64 / total_pixels as f64) * (100 as f64);
        drop(tmp_data);
        print!(
            "\rProcessing pixel {} of {} ({:.2}%)...",
            cur_pixel, total_pixels, cur_pct
        );
        stdout.flush().unwrap();
        sleep(Duration::from_millis(delay_ms));
    }
    println!("Finished processing image!");
}
