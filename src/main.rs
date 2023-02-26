extern crate image;

use image::GenericImageView;
use plotters::prelude::*;
use std::fs;
use std::path::Path;
use std::time::Instant;

struct Point {
    x: u32,
    y: u32,
    r: u8,
    g: u8,
    b: u8,
}

struct ImgInfo {
    width: u32,
    height: u32,
    points: Vec<Point>,
}

const PITCH_START: f64 = 0.2;
const PITCH_INCREMENT: f64 = 0.01;
const YAW_START: f64 = 0.0;
const YAW_INCREMENT: f64 = 0.013;
const SCALE: f64 = 0.8;
const IMG_WIDTH: u32 = 1280;
const IMG_HEIGHT: u32 = 720;
const FRAME_DELAY: u32 = 100;
const POINT_SIZE: f32 = 0.7;
const FRAME_AMOUNT: u8 = 60;

fn process_image(input_path: &str) -> Option<ImgInfo> {
    let img = image::open(&Path::new(input_path));
    if img.is_err() {
        println!("Error: {}", img.err().unwrap());
        return None;
    }

    let mut points: Vec<Point> = Vec::new();

    for p in img.as_ref().unwrap().pixels() {
        // print rgb value of pixel
        points.push(Point {
            x: p.0,
            y: p.1,
            r: p.2[0],
            g: p.2[1],
            b: p.2[2],
        });
    }
    return Some(ImgInfo {
        points: points,
        width: img.as_ref().unwrap().width(),
        height: img.as_ref().unwrap().height(),
    });
}

fn main() {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    print!("Please enter image path: ");
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if let Some('\n') = s.chars().next_back() {
        s.pop();
    }
    if let Some('\r') = s.chars().next_back() {
        s.pop();
    }
    let img_info = process_image(&s).unwrap();

    fs::create_dir_all("images").unwrap();
    let start = Instant::now();

    let red_root = BitMapBackend::gif("images/r-val.gif", (IMG_WIDTH, IMG_HEIGHT), FRAME_DELAY)
        .unwrap()
        .into_drawing_area();
    red_root.fill(&WHITE).unwrap();
    let mut red_chart = ChartBuilder::on(&red_root)
        .margin(20)
        .caption(format!("Red values of {s}"), ("sans-serif", 40))
        .build_cartesian_3d(0..img_info.width, 0..img_info.height, 0..255 as u32)
        .unwrap();

    let blue_root = BitMapBackend::gif("images/b-val.gif", (IMG_WIDTH, IMG_HEIGHT), FRAME_DELAY)
        .unwrap()
        .into_drawing_area();
    blue_root.fill(&WHITE).unwrap();
    let mut blue_chart = ChartBuilder::on(&blue_root)
        .margin(20)
        .caption(format!("Blue values of {s}"), ("sans-serif", 40))
        .build_cartesian_3d(0..img_info.width, 0..img_info.height, 0..255 as u32)
        .unwrap();

    let green_root = BitMapBackend::gif("images/g-val.gif", (IMG_WIDTH, IMG_HEIGHT), FRAME_DELAY)
        .unwrap()
        .into_drawing_area();
    green_root.fill(&WHITE).unwrap();
    let mut green_chart = ChartBuilder::on(&green_root)
        .margin(20)
        .caption(format!("Green values of {s}"), ("sans-serif", 40))
        .build_cartesian_3d(0..img_info.width, 0..img_info.height, 0..255 as u32)
        .unwrap();

    let mut generate_red_frame = |i: &u8| {
        // Generate red frame
        red_root.fill(&WHITE).unwrap();
        red_chart.with_projection(|mut pb| {
            pb.pitch = PITCH_START + PITCH_INCREMENT * *i as f64;
            pb.yaw = YAW_START + YAW_INCREMENT * *i as f64;
            pb.scale = SCALE;
            pb.into_matrix()
        });
        red_chart
            .draw_series(PointSeries::of_element(
                img_info
                    .points
                    .iter()
                    .map(|point| (point.x as u32, point.y as u32, point.r as u32)),
                POINT_SIZE,
                RED,
                &|c, s, st| {
                    return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                },
            ))
            .unwrap();
        red_chart.configure_axes().draw().unwrap();
        red_root.present().unwrap();
    };

    let mut generate_green_frame = |i: &u8| {
        green_root.fill(&WHITE).unwrap();
        green_chart.with_projection(|mut pb| {
            pb.pitch = PITCH_START + PITCH_INCREMENT * *i as f64;
            pb.yaw = YAW_START + YAW_INCREMENT * *i as f64;
            pb.scale = SCALE;
            pb.into_matrix()
        });
        green_chart
            .draw_series(PointSeries::of_element(
                img_info
                    .points
                    .iter()
                    .map(|point| (point.x as u32, point.y as u32, point.g as u32)),
                POINT_SIZE,
                GREEN,
                &|c, s, st| {
                    return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                },
            ))
            .unwrap();
        green_chart.configure_axes().draw().unwrap();
        green_root.present().unwrap();
    };

    let mut generate_blue_frame = |i: &u8| {
        // Generate blue frame
        blue_root.fill(&WHITE).unwrap();
        blue_chart.with_projection(|mut pb| {
            pb.pitch = PITCH_START + PITCH_INCREMENT * *i as f64;
            pb.yaw = YAW_START + YAW_INCREMENT * *i as f64;
            pb.scale = SCALE;
            pb.into_matrix()
        });
        blue_chart
            .draw_series(PointSeries::of_element(
                img_info
                    .points
                    .iter()
                    .map(|point| (point.x as u32, point.y as u32, point.b as u32)),
                POINT_SIZE,
                BLUE,
                &|c, s, st| {
                    return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                },
            ))
            .unwrap();
        blue_chart.configure_axes().draw().unwrap();
        blue_root.present().unwrap();
    };

    for i in 0..FRAME_AMOUNT {
        let start_frame = Instant::now();
        println!("Generating frame {}", i + 1);
        generate_red_frame(&i);
        generate_green_frame(&i);
        generate_blue_frame(&i);
        println!(
            "Finished generating frame {}, time elapsed: {:?}",
            i + 1,
            start_frame.elapsed()
        );
    }

    println!("Time elapsed generating all frames: {:?}", start.elapsed());
}
