extern crate cv;

use cv::*;
use cv::highgui::*;
use cv::imgcodecs::ImreadModes;

fn main() {
    ////////////////////////////////
    //
    // 1. Read the image
    //
    ///////////////////////////////

    let args: Vec<_> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: calchist <Path to Image>");
        std::process::exit(-1);
    }

    let mat = Mat::from_path(&args[1], ImreadModes::ImreadGrayscale).expect("Failed to read from path");

    if !mat.is_valid() {
        println!("Could not open or find the image");
        std::process::exit(-1);
    }

    ////////////////////////////////
    //
    // 2. Calculate the histogram
    //    (only demo one channel)
    //
    ///////////////////////////////

    let hsize = 256;
    let ranges = [0_f32, 256_f32];
    let p_ranges: [*const f32; 1] = [ranges.as_ptr() as *const f32];
    let channels = [0];
    let hist = mat.calc_hist(channels.as_ptr(), Mat::new(), 1, &hsize, p_ranges.as_ptr());

    ////////////////////////////////
    //
    // 3. Display the histogram
    //
    ///////////////////////////////

    // Create a 256x200 window, the bin width
    let hist_w = hsize;
    let hist_h = 200;
    let hist_image = Mat::with_size(hist_h, hist_w, CvType::Cv8UC3 as i32);

    // Normalize the histogram to the height of the histogram window
    let b_hist = hist.normalize(0.0, hist_h as f64, NormTypes::NormMinMax);

    // Plot each segment as a line element
    for i in 1..hsize {
        let start = Point2i::new(i - 1, hist_h - b_hist.at::<f32>(i - 1) as i32);
        let end = Point2i::new(i, hist_h - b_hist.at::<f32>(i) as i32);
        hist_image.line(start, end);
    }

    // Show the histogram
    highgui_named_window("Display window", WindowFlags::WindowNormal);
    hist_image.show("Histogram", 0).unwrap();
}
