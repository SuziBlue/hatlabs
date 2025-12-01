pub struct Car {
    wheels: i32,
    fuel: f32,
    speed: f32,
    liscense_plate: String,
}

impl Car {
    pub fn print_info(self: Self) -> String {
        format!(
            "This is my fancy car! Plate: {}, Wheels: {}, Going {} mph!",
            self.liscense_plate, self.wheels, self.speed
        )
    }
}

impl Default for Car {
    fn default() -> Self {
        Self {
            wheels: 4,
            fuel: 50f32,
            speed: 0f32,
            liscense_plate: String::from("AAAA00"),
        }
    }
}
