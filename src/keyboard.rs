use gtk::prelude::*;
use gtk::{Button, Grid};
use crate::input::{InputSimulator, InputError};

pub struct VirtualKeyboard {
    grid: Grid,
}

impl VirtualKeyboard {
    pub fn new() -> Self {
        let grid = Grid::builder()
            .margin_top(10)
            .margin_bottom(10)
            .margin_start(10)
            .margin_end(10)
            .row_spacing(5)
            .column_spacing(5)
            .build();

        let input_simulator = InputSimulator::new();

        // Define the keyboard layout
        let rows = [
            "1234567890",
            "qwertyuiop",
            "asdfghjkl",
            "zxcvbnm",
        ];

        // Create buttons for each key
        for (row_idx, row) in rows.iter().enumerate() {
            for (col_idx, key) in row.chars().enumerate() {
                let button = Button::builder()
                    .label(&key.to_string())
                    .width_request(40)
                    .height_request(40)
                    .build();

                let key_char = key;
                let input_simulator = input_simulator.clone();
                button.connect_clicked(move |_| {
                    if let Err(e) = input_simulator.simulate_key_press(key_char) {
                        eprintln!("Failed to simulate key press: {}", e);
                    }
                });

                grid.attach(&button, col_idx as i32, row_idx as i32, 1, 1);
            }
        }

        // Add space bar
        let space = Button::builder()
            .label("Space")
            .width_request(200)
            .height_request(40)
            .build();

        let space_input_simulator = input_simulator.clone();
        space.connect_clicked(move |_| {
            if let Err(e) = space_input_simulator.simulate_space() {
                eprintln!("Failed to simulate space: {}", e);
            }
        });

        grid.attach(&space, 0, 4, 10, 1);

        Self { grid }
    }

    pub fn get_grid(&self) -> &Grid {
        &self.grid
    }
} 