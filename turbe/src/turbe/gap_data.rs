use std::collections::VecDeque;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct GapData {
    pub empty_spaces : VecDeque<usize>,
    pub recently_filled : VecDeque<usize>
}

impl GapData {
    
    pub fn new() -> GapData {
        return GapData{empty_spaces : VecDeque::new(), recently_filled : VecDeque::new()};
    }

}