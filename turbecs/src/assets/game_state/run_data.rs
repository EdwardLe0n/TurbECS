#[turbo::serialize]
#[derive(PartialEq)]
pub struct RunData {
    pub player_posts : Vec<i32>,
    pub time : u32,
}

impl RunData {
    
    pub fn new() -> RunData {
        return RunData{player_posts : Vec::new(), time : 1};
    }

    pub fn clear(&mut self) {

        self.player_posts.clear();
        self.time = 1;

    }

}