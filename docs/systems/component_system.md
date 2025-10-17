# Component System

## Intro

The component system functions both as a specified data container, but can be modified to have 

## Directory

- [Making an instance of a component](#making-an-instance-of-a-component-based-off-of-a-template)
- [Making a new custom component](#making-a-new-custom-component)

---

## Making an instance of a component based off of a template

When making a new instance of a component based off a template, you'll need to do four things:

1. Refer to the individual component type to check on how to make it
2. Modify the component as much as you'd like before continuing
3. Cast the data into a component data enum dependent on the enum type 
4. Use the new() call within the component to initialize a component

Example:

```
pub fn some_function() {

    ...

    // Making a mutable reference to a text box component
    let mut text_box = TextBoxComponent::new("TurbECS".to_string());

    // Basic styling updates 
    text_box.font = "large".to_string();
    text_box.color = 0xff0000ff;

    // changes the dimensions ot match the text
    text_box.transform.set_width(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_x() * 4);
    text_box.transform.set_height(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_y() as i32 * -2);

    // Centering all text in the textbox
    text_box.transform.position.set_horizontal_pref(bound_data::Horizontal::Center);

    // adds the new component to a VecDequeue
    ent_queue.push_front(Component::new(ComponentData::TextBox(text_box)));
    
    ...
}
```

## Making a new custom component

One of the central points of TurbECS is to make sure 

### Making a component data structure

### Adding components to the ComponentData enum

### Linking components to the ComponentData system

#### Adding to a component to the has system

#### Linking a component to the lifetime system

---

Last updated: 10/16/25