
// Necessry imports

use crate::kitchen::objectSystem::GameObject;
use crate::kitchen::componentSystem::Component;

use super::componentTypes::ComponentTypes;


/*

    List of prefabs!

 */

pub fn title () -> GameObject {

    let mut title : GameObject = GameObject::new("Title Card".to_string());

    title.set_layer(9);

    let mut title_box_bg :Component = Component::new_rectangle(100, 12);
    title_box_bg.set_color(0x0000ffff);
    title_box_bg.set_ui_element_status(true);
    title_box_bg.set_center_status(false);
    title_box_bg.change_component_offset(-120, -122);

    title.add_component(title_box_bg);

    let mut title_text_comp : Component = Component::new_text("The title!".to_string(), true);
    
    title_text_comp.change_text_size("large".to_string());
    title_text_comp.change_component_offset(-120, -120);


    title.add_component(title_text_comp);

    return title;

}

pub fn test_rect() -> GameObject {

    let mut new_rect : GameObject = GameObject::new("test rectangle".to_string());

    new_rect.set_layer(9);

    let mut rect_vis : Component = Component::new_rectangle(10, 20);
    rect_vis.set_color(0x111111ff);
    rect_vis.set_center_status(true);

    new_rect.add_component(rect_vis);

    return new_rect;

}

pub fn test_rect_2() -> GameObject {

    let mut new_rect : GameObject = GameObject::new("test rectangle".to_string());
    new_rect.nudge_position(10, 10);

    new_rect.set_layer(8);

    let mut rect_vis : Component = Component::new_rectangle(10, 20);
    rect_vis.set_color(0x333333ff);
    new_rect.add_component(rect_vis);

    return new_rect;

}

pub fn square() -> GameObject {

    let mut new_square : GameObject = GameObject::new("test cube".to_string());

    let mut square_vis : Component =  Component::new_square(3);
    square_vis.set_color(0xd63600ff);

    new_square.add_component(square_vis);

    return new_square;

}

pub fn player() -> GameObject {

    let mut new_char : GameObject = GameObject::new("player".to_string());

    let mut new_box : Component =  Component::new_square(7);
    new_box.set_color(0xd63600ff);

    new_box.set_ui_element_status(true);

    new_char.add_component(new_box);

    return new_char;

}

pub fn moving_cam() -> GameObject {

    let mut new_cam : GameObject = GameObject::new_camera();

    let cam_controller : Component = Component::new(ComponentTypes::CameraController);

    new_cam.add_component(cam_controller);

    return new_cam;

}

