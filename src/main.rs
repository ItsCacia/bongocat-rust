#![cfg_attr(windows, windows_subsystem = "windows")]

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use image::ImageReader;
use indexmap::IndexSet;
use rdev::{listen, EventType, Key};
use std::collections::HashMap;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;
use sfml::cpp::FBox;
use sfml::graphics::{Color, RenderStates, RenderTarget, RenderWindow, Sprite, Texture};
use sfml::window::{ContextSettings, Event, Style, VideoMode};

mod img_loader;
mod kuvster;
mod arm_map;

static FPS: u8 = 60;

#[tokio::main]
async fn main() {
    let vm = VideoMode::new(612, 344, 24);
    let settings = ContextSettings {
            antialiasing_level: 3,
            ..Default::default()
        };
    let mut window = RenderWindow::new(vm, "bongocat-rust", Style::CLOSE, &settings).unwrap();
    let _ = window.set_active(true);
    window.set_visible(true);
    window.set_framerate_limit(FPS as u32);
    window.set_vertical_sync_enabled(false);

    let icon_path = Path::new("./icon.ico");
        let icon_image = ImageReader::open(icon_path)
            .expect("Failed to open icon file")
            .decode()
            .expect("Failed to decode image")
            .to_rgba8();

    let (width, height) = icon_image.dimensions();
    unsafe {
        window.set_icon(width, height, &icon_image);
    }

    let textures = img_loader::load_images();

	let key_arms = arm_map::get_hm();
	let keys_pressed: Arc<Mutex<IndexSet<Key>>> =  Arc::new(Mutex::new(IndexSet::new()));
	let keys_pressed_shared = Arc::clone(&keys_pressed);

	let volume: Arc<Mutex<f32>> = Arc::new(Mutex::new(0.0));
	let volume_shared = Arc::clone(&volume);

	let shutdown: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
	let shutdown_audio = Arc::clone(&shutdown);

	// Key handler
    let _ = thread::spawn(move || {
        let hm = arm_map::get_hm();
        if let Err(err) = listen(move |event: rdev::Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let mut keys = keys_pressed_shared.lock().unwrap();
                    if hm.contains_key(&key) {
                        keys.insert(key);
                    }
                },
                EventType::KeyRelease(key) => {
                    let mut keys = keys_pressed_shared.lock().unwrap();
                    keys.shift_remove(&key);
                },
                _ => ()
            }
        }) {
            eprintln!("Error: {:?}", err);
        }
    });

    // Mic handler
    let _ = tokio::task::spawn_blocking(move || {
        let host = cpal::default_host();
        let device = host.default_input_device().expect("Failed to find input device");
        let config = device.default_input_config().expect("Failed to get default input config");

        let stream = device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                let mut sum = 0.0;
                for &sample in data {
                    sum += sample * sample;
                }
                let rms = (sum / data.len() as f32).sqrt();
                let mut volume = volume_shared.lock().unwrap();
                *volume = rms.max(*volume * 0.75);
            },
            move |err| {
                eprintln!("An error occurred on the input audio stream: {}", err);
            },
            None
        ).expect("Failed to create audio stream");

        stream.play().expect("Failed to start audio stream");

        while !shutdown_audio.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
        }
    });

    'mainloop: while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => break 'mainloop,
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        draw(&textures, &mut window, &keys_pressed, &key_arms, &volume);
        window.display();
    }

    shutdown.store(true, Ordering::Release);
}

fn draw(textures: &HashMap<&str, FBox<Texture>>, window: &mut RenderWindow, keys_pressed: &Arc<Mutex<IndexSet<Key>>>, key_arms: &HashMap<Key, &'static str>, mic_volume: &Arc<Mutex<f32>>) {
    // background
    window.draw_sprite(&Sprite::with_texture(
        textures.get("background").unwrap()
    ), &RenderStates::DEFAULT);

    // base
    window.draw_sprite(&Sprite::with_texture(
        textures.get("base").unwrap()
    ), &RenderStates::DEFAULT);

    // mouth (inside) TODO
    kuvster::draw_mouth(window, mic_volume);

    // mouth
    window.draw_sprite(&Sprite::with_texture(
        textures.get("mouth").unwrap()
    ), &RenderStates::DEFAULT);

    // mouse arm
    kuvster::draw_mouse_arm(textures, window);

    // keyboard arm
    window.draw_sprite(&Sprite::with_texture({
        if keys_pressed.lock().unwrap().len() == 0 {
            textures.get("up").unwrap()
        } else {
            textures.get(key_arms.get(keys_pressed.lock().unwrap().last().unwrap()).unwrap()).unwrap()
        }
    }), &RenderStates::DEFAULT);

    // border
    window.draw_sprite(&Sprite::with_texture(
        textures.get("border").unwrap()
    ), &RenderStates::DEFAULT);
}
