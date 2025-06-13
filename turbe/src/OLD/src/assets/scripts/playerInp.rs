// Imports

use crate::gamepad;
use turbo::{input::{Button, Gamepad}, log};

use crate::text;

// Player input handler
pub fn player_controller() -> (i32,i32)
{
    
    let p1_gamepad : Gamepad<Button> = gamepad(0);
    
    
    // Checks if the A button is JustPressed or Pressed
    if p1_gamepad.a.pressed() {
        
        // Sanity check
        text!("A is getting held!");
        log!("A is getting held");
    }

    return movement_handling(p1_gamepad);
    
}

// Movement handler function
fn movement_handling(gamepad : Gamepad<Button>) -> (i32,i32)
{

    // stores any movement data in a vector like object
    let mut move_vec : (i32, i32) = (0,0);

    // Initializes a var and sets it to 5
    let speed : i32 = 4;

    // input handling
    if gamepad.left.pressed()
    {
        move_vec.0 -= speed;
    }

    if gamepad.right.pressed()
    {
        move_vec.0 += speed;
    }

    if gamepad.up.pressed()
    {
        move_vec.1 -= speed;
    }

    if gamepad.down.pressed()
    {
        move_vec.1 += speed;
    }

    // returns data!
    return (move_vec.0, move_vec.1);
}