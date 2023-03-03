use csv::ReaderBuilder;
use image::GenericImageView;
use std::error::Error;
use std::fs::File;
use std::io::{stdin, stdout, Write};

fn main() -> Result<(), Box<dyn Error>> {
    // Title
    println!("--- Spatial Filtering ---");

    // Input image
    print!("[INPUT ]input image : ");
    stdout().flush().unwrap();

    let mut input_image = String::new();
    stdin().read_line(&mut input_image)?;
    input_image = input_image.trim().to_string();

    print!("[STATUS]open image : ");
    stdout().flush().unwrap();

    let img = image::open(input_image).unwrap();

    println!("Done");

    // Input filter
    print!("[INPUT ]input filter : ");
    stdout().flush().unwrap();

    let mut input_csv = String::new();
    stdin().read_line(&mut input_csv)?;
    input_csv = input_csv.trim().to_string();

    print!("[STATUS]open csv : ");
    stdout().flush().unwrap();

    let buf_file = File::open(&input_csv)?;
    let mut reader_buf = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(buf_file);

    let csv_file = File::open(input_csv)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(csv_file);

    println!("Done");

    print!("[STATUS]read csv : ");
    stdout().flush().unwrap();

    let mut row: usize = 0;
    let mut flag: i32 = 0;
    let mut col: usize = 0;
    for result in reader_buf.records() {
        let record = result?;
        row += 1;
        if flag == 0 {
            col = record.len();
            flag = 1;
        } else {
            if col != record.len() {
                panic!("Incorrect input data form (col)");
            }
        }
    }
    if row != col {
        panic!("Incorrect input data form (row != col)");
    }
    if row == 0 {
        panic!("Incorrect input data form (row == col == 0)");
    }
    if row % 2 == 0 {
        panic!("Incorrect input data form (row : col : even number)");
    }

    let mut v: Vec<f32> = Vec::new();
    for result in reader.records() {
        let record = result?;
        for i in 0..row {
            let float_value = record[i].parse::<f32>();
            match float_value {
                Ok(_) => {}
                Err(e) => panic!("Error parsing float: {}", e),
            }
            v.push(float_value.unwrap());
        }
    }

    println!("Done");

    println!("[INFO  ]filter :");
    for i in 0..row {
        for j in 0..col {
            print!("{}, ", v[i * col + j]);
        }
        println!("");
    }

    // Spatial Filtering
    print!("[STATUS]spatial filtering : ");
    stdout().flush().unwrap();

    let mut dst = image::RgbImage::new(
        img.width() - ((row - 1) as u32),
        img.height() - ((row - 1) as u32),
    );
    for y in 0..dst.height() {
        for x in 0..dst.width() {
            let mut r: f32 = 0.0;
            let mut g: f32 = 0.0;
            let mut b: f32 = 0.0;

            for i in 0..row {
                for j in 0..row {
                    let pixel = img.get_pixel(x + (j as u32), y + (i as u32));
                    r += pixel[0] as f32 * v[j + i * row];
                    g += pixel[1] as f32 * v[j + i * row];
                    b += pixel[2] as f32 * v[j + i * row];
                }
            }
            let pixel_dst = image::Rgb([r as u8, g as u8, b as u8]);

            dst.put_pixel(x, y, pixel_dst)
        }
    }

    println!("Done");

    // output file
    print!("[INPUT ]output image : ");
    stdout().flush().unwrap();

    let mut output_image = String::new();
    stdin().read_line(&mut output_image)?;
    output_image = output_image.trim().to_string();

    print!("[STATUS]write image : ");
    stdout().flush().unwrap();

    dst.save(output_image).unwrap();

    println!("Done");

    Ok(())
}
