

use turbo::log;

use crate::kitchen::componentSystem::Component;
use crate::assets::scripts::playerInp;

use crate::GameState;

impl Component {

    pub fn update_cam_pos(&mut self) {

        let mut state: GameState = GameState::load();

        let mut some_vals : (i32, i32) = playerInp::player_controller();
        some_vals.0 += self.get_x();
        some_vals.1 += self.get_y();
    
        // Currently changes the component xy, not the game object xy

        // log!("{:?}", some_vals.0);
        
        state.current_game_objects[self.comp_locat.0].nudge_position(some_vals.0, some_vals.1);

        // log!("{:?}", state.current_game_objects[self.comp_locat_vec[0]].get_pos_x());

        state.save();
    
    }

}