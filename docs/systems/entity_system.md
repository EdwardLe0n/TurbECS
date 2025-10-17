# The Entity System

## Intro

The entity system is all about the Entity, a generalized struct that is meant to hold a number of components!

## Directory

- [Making a Prefab](#making-a-prefabricated-entity)

## Making a Prefabricated Entity

Oftentimes, you'll want to make a template entity in one area, and use it else where. Whether it be for reusability, or just for code cleanliness, this is where prefabs come in!

When making a prefab, you'll need a both an Entity, and a VecDeque of Components. This is due to the fact that since the Entity and the Component structs are not directly connected, we'll be adding them to TurbECS in their own managers, but still at the same time.

To look at a basic example, take a look at this!
```
pub fn new_title () -> (Entity, VecDeque<Component>) {

    // Immediately, make the necessary entity and VecDeque
    let mut ent = Entity::new_base("Title".to_string());
    let mut ent_queue = VecDeque::new();
    
    // Sets the render layer of the entity to 10
    ent.set_layer(10);

    // Made a text_box component, and from here just adding/modifying data
    let mut text_box = TextBoxComponent::new("TurbECS".to_string());

    text_box.font = "large".to_string();
    text_box.color = 0xff0000ff;

    text_box.transform.set_width(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_x() * 4);
    text_box.transform.set_height(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_y() as i32 * -2);
    // text_box.transform.set_scale(1.2);

    ent.transform.nudge_y(text_box.transform.get_height() / 2);

    text_box.transform.position.set_horizontal_pref(bound_data::Horizontal::Center);

    // Once we're done modifying the component, add it to the VecDeque 
    ent_queue.push_front(Component::new(ComponentData::TextBox(text_box)));

    // Return both the entity, and the VecDeque that was made
    return (ent, ent_queue);

}
```
- Source : [Click here!](../../turbecs/src/assets/prefabs/general_prefabs.rs)

---

## Still in progress! Expect this to be done soon!

---
Last updated: 10/17/2025