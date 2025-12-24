use ray_tracer_core::*;
use rand::prelude::ThreadRng;
use rand::thread_rng;
use std::sync::{Arc, Mutex};
use std::thread;

pub struct ThreadedCamera {
    pub core_camera: Camera,
}

impl ThreadedCamera {
    pub fn new() -> Self {
        ThreadedCamera {
            core_camera: Camera::new(),
        }
    }

    pub fn render(&self, world: Arc<HittableList>, y_blocks: i64, verbose: bool) -> String {
        let buf_size = self.core_camera.image_height * self.core_camera.image_width * 3;
        let block_height = self.core_camera.image_height / y_blocks;
        let block_size = self.core_camera.image_width * 3;

        let buf = Arc::new(Mutex::new(vec![0.0; buf_size as usize]));

        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        // iterate over blocks
        for j in 0..y_blocks {
            let camera: Camera = self.core_camera;
            let buf: Arc<Mutex<Vec<f64>>> = Arc::clone(&buf);
            let world: Arc<HittableList> = world.clone();
            let block: i64 = j;
            let width: i64 = self.core_camera.image_width;

            let handle: thread::JoinHandle<()> = thread::spawn(move || {
                let mut rng = thread_rng();

                let q = block_height * block_size;
                let mut local_buf = vec![0.0; q as usize];

                // iterate internally on block
                for y in 0..block_height {
                    for x in 0..width {
                        let c = camera.render_pixel(&world, &mut rng, x, block * block_height + y);
                        local_buf[(y * block_size + x * 3) as usize] = c.x();
                        local_buf[(y * block_size + x * 3 + 1) as usize] = c.y();
                        local_buf[(y * block_size + x * 3 + 2) as usize] = c.z();
                    }
                    if verbose {
                        println!("Thread {} completed block {}", block, y);
                    }
                }

                let mut buf = buf.lock().unwrap();
                buf[((block * block_height * block_size) as usize)
                    ..((((block + 1) * block_height) * block_size) as usize)]
                    .copy_from_slice(&local_buf);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let buf = buf.lock().unwrap();

        // Unwrapping buffer to a string
        let mut str_buf: String = String::new();
        str_buf.push_str(format!("P3\n{} {}\n255\n", self.core_camera.image_width, self.core_camera.image_width).as_str());

        for j in 0..self.core_camera.image_height {
            for i in 0..self.core_camera.image_width {
                let x = buf[(j * block_size + i * 3) as usize];
                let y = buf[(j * block_size + i * 3 + 1) as usize];
                let z = buf[(j * block_size + i * 3 + 2) as usize];
                write_color(&mut str_buf, Vec3::new(x, y, z));
            }
            write_new_line(&mut str_buf);
        }

        return str_buf;
    }
}