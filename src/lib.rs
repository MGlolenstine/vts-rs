#![allow(unused)]

/// Base structure, that stores information about currently stored queue and the module entrypoint.
pub struct VirtualTouchScreen {
    pub filename: String,
    queue: Vec<String>,
}

impl VirtualTouchScreen {
    /// Create new instance with default location at `/dev/virtual_touchscreen`
    pub fn new() -> VirtualTouchScreen {
        VirtualTouchScreen {
            filename: "/dev/virtual_touchscreen".to_string(),
            queue: Vec::new(),
        }
    }

    /// Create new instance with a custom location of the kernel module entry
    pub fn new_with_path(path: String) -> VirtualTouchScreen {
        VirtualTouchScreen {
            filename: path,
            queue: Vec::new(),
        }
    }

    /// Remove all actions from the queue
    pub fn clear_queue(&mut self) {
        self.queue.clear();
    }

    /// Remove last action from the queue
    pub fn pop_queue(&mut self) {
        self.queue.pop();
    }

    /// Execute the queue
    pub fn submit_queue(&mut self) {
        let mut full_command = String::new();
        for s in self.queue.clone() {
            full_command += &s;
        }

        use std::fs;
        use std::io::prelude::*;
        fs::write(&self.filename, full_command).expect("Unable to write to module entrypoint.");
        self.clear_queue();
    }

    /// Set active pointer
    pub fn set_pointer(&mut self, pointer: u32) {
        self.queue.push(format!("s {}\na {}\n", pointer, pointer));
    }

    /// Set pointer position
    pub fn set_position(&mut self, x: u32, y: u32) {
        self.set_x(x);
        self.set_y(y);
    }

    /// Set X coordinate
    pub fn set_x(&mut self, x: u32) {
        self.queue.push(format!("X {}\nx {}\n", x, x));
    }

    /// Set Y coordinate
    pub fn set_y(&mut self, y: u32) {
        self.queue.push(format!("Y {}\ny {}\n", y, y));
    }
}
