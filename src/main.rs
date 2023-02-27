extern crate image;

use image::GenericImageView;
use plotters::prelude::*;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Instant;

#[derive(Clone)]
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

const ANIMATION_DURATION: f32 = 10.0;
const FPS: f32 = 60.0;
const FRAME_DELAY: u32 = (1.0 / FPS * 1000.0) as u32;
const FRAME_AMOUNT: u16 = ANIMATION_DURATION as u16 * FPS as u16;
const PITCH_START: f64 = 0.0;
const PITCH_END: f64 = 0.55;
const PITCH_INCREMENT: f64 = (PITCH_END - PITCH_START) / FRAME_AMOUNT as f64;
const YAW_START: f64 = 0.2;
const YAW_END: f64 = 10.0;
const YAW_INCREMENT: f64 = (YAW_END - YAW_START) / FRAME_AMOUNT as f64;
const SCALE: f64 = 0.8;
const IMG_WIDTH: u32 = 1280;
const IMG_HEIGHT: u32 = 720;
const POINT_SIZE: f32 = 0.7;

fn process_image(input_path: &str) -> Option<ImgInfo> {
    let img = image::open(&Path::new(input_path));
    if img.is_err() {
        println!("Error: {}", img.err().unwrap());
        return None;
    }

    let mut points: Vec<Point> = Vec::new();
    let width = img.as_ref().unwrap().width();
    let height = img.as_ref().unwrap().height();

    for p in img.as_ref().unwrap().pixels() {
        // print rgb value of pixel
        points.push(Point {
            x: p.0,
            y: height - p.1,
            r: p.2[0],
            g: p.2[1],
            b: p.2[2],
        });
    }
    return Some(ImgInfo {
        points,
        width,
        height,
    });
}

fn spawn_composition_thread(
    title: String,
    file_name: String,
    points: Vec<Point>,
    color_extractor: fn(&Point) -> u32,
    color: String,
    color_obj: RGBColor,
    height: u32,
    width: u32,
) -> std::thread::JoinHandle<()> {
    thread::spawn(move || {
        let root = BitMapBackend::gif(file_name, (IMG_WIDTH, IMG_HEIGHT), FRAME_DELAY)
            .unwrap()
            .into_drawing_area();
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .caption(title, ("sans-serif", 40))
            .build_cartesian_3d(0..width, 0..height, 0..255 as u32)
            .unwrap();

        for i in 0..FRAME_AMOUNT {
            // Generate red frame
            let start_frame = Instant::now();
            println!("Generating {} frame {}", color, i + 1);
            root.fill(&WHITE).unwrap();
            chart.with_projection(|mut pb| {
                pb.pitch = PITCH_START + PITCH_INCREMENT * i as f64;
                pb.yaw = YAW_START + YAW_INCREMENT * i as f64;
                pb.scale = SCALE;
                pb.into_matrix()
            });
            chart
                .draw_series(PointSeries::of_element(
                    points
                        .iter()
                        .map(|point| (point.x as u32, point.y as u32, color_extractor(point))),
                    POINT_SIZE,
                    color_obj,
                    &|c, s, st| {
                        return EmptyElement::at(c) + Circle::new((0, 0), s, st.filled());
                    },
                ))
                .unwrap();
            chart.configure_axes().draw().unwrap();
            root.present().unwrap();
            println!(
                "Finished generating {} frame {}, time elapsed: {:?}",
                color,
                i + 1,
                start_frame.elapsed()
            );
        }
    })
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
    let img_info_result = process_image(&s);
    if img_info_result.is_none() {
        println!("Could not process image {}", s);
        return;
    }
    let img_info = img_info_result.unwrap();

    fs::create_dir_all("images").unwrap();
    let start = Instant::now();

    let red_title = format!("Red values of {s}");
    let red_file_name = "images/r-val.gif";
    let mut red_points = Vec::new();
    red_points.resize(
        img_info.points.len(),
        Point {
            x: 0,
            y: 0,
            r: 0,
            g: 0,
            b: 0,
        },
    );
    red_points.clone_from_slice(img_info.points.as_slice());

    let green_file_name = "images/g-val.gif";
    let green_title = format!("Green values of {s}");
    let mut green_points = Vec::new();
    green_points.resize(
        img_info.points.len(),
        Point {
            x: 0,
            y: 0,
            r: 0,
            g: 0,
            b: 0,
        },
    );
    green_points.clone_from_slice(img_info.points.as_slice());

    let blue_file_name = "images/b-val.gif";
    let blue_title = format!("Blue values of {s}");
    let mut blue_points = Vec::new();
    blue_points.resize(
        img_info.points.len(),
        Point {
            x: 0,
            y: 0,
            r: 0,
            g: 0,
            b: 0,
        },
    );
    blue_points.clone_from_slice(img_info.points.as_slice());

    let red_thread = spawn_composition_thread(
        red_title,
        red_file_name.to_string(),
        red_points,
        |point| point.r as u32,
        "red".to_string(),
        RED,
        img_info.height.clone(),
        img_info.width.clone(),
    );

    let green_thread = spawn_composition_thread(
        green_title,
        green_file_name.to_string(),
        green_points,
        |point| point.g as u32,
        "green".to_string(),
        GREEN,
        img_info.height.clone(),
        img_info.width.clone(),
    );

    let blue_thread = spawn_composition_thread(
        blue_title,
        blue_file_name.to_string(),
        blue_points,
        |point| point.b as u32,
        "blue".to_string(),
        BLUE,
        img_info.height.clone(),
        img_info.width.clone(),
    );

    println!("Generating total of {} frames", FRAME_AMOUNT);

    red_thread.join().unwrap();
    green_thread.join().unwrap();
    blue_thread.join().unwrap();

    println!(
        "Time elapsed generating all {} frames: {:?}",
        FRAME_AMOUNT,
        start.elapsed()
    );
}
