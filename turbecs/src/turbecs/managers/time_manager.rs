use turbo::*;

static RENDER_TIME : bool = false;

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct TimeManager {

    last_time : u64,
    curr_time : u64,
    pub delta : f32,
    pub time_scale : f32,
    pub delta_scaled : f32,
    pub frames_per_second : i32,

}

impl TimeManager {
    
    pub fn new() -> Self {
        return Self {
            last_time : time::now(),
            curr_time : time::now(),
            delta : 0.0,
            time_scale : 1.0,
            delta_scaled : 0.0,
            frames_per_second : 0,
        };
    }

    pub fn update(&mut self) {

        // Standard time usage
        self.last_time = self.curr_time;
        self.curr_time = time::now();

        // Divides by 1000 since the delta can be as small as milliseconds,
        // it's preferred to have the data be relative to seconds
        self.delta = ((self.curr_time - self.last_time) as f32) / 1000.0;

        // Gets the number frames that can happen per second based on the delta
        self.frames_per_second = (1.0 / self.delta) as i32;

        // Used for when people want to slow down or speed up everything
        self.delta_scaled = self.delta * self.time_scale;

    }

    pub fn render(&mut self) {

        if RENDER_TIME {

            let mut some_string = "".to_string();

            // adds frame info

            some_string.push_str("FPS: ");
            some_string.push_str(&self.frames_per_second.to_string());
            some_string.push_str("\n");

            // add delta time info

            some_string.push_str("Delta: ");
            some_string.push_str(&self.delta.to_string());
            some_string.push_str("\n");

            // background rect just in case

            rect!(
                x = 1,
                y = 1,
                w = 80,
                h = 20,
                color = 0xffffff55,
                rotation = 0,
                border_size = 0,
                border_color = 0,
                border_radius = 4,
                fixed = true
            );
            
            text!(
                &some_string,
                x = 4,
                y = 4,
                color = 0x000000ff,
                font = "small",
                fixed = true
            )

        }

    }

}