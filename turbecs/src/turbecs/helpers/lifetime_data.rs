use std::collections::VecDeque;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct LifetimeData {
    pub new_awake : VecDeque<usize>,
    pub new_start : VecDeque<usize>,
    pub new_destroy : VecDeque<usize>
}

impl LifetimeData {
    
    pub fn new() -> LifetimeData {
        return LifetimeData{
            new_awake : VecDeque::with_capacity(50), 
            new_start : VecDeque::with_capacity(50), 
            new_destroy : VecDeque::with_capacity(50)
        };
    }

}