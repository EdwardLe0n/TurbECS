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
- Source : [Click here!](../../turbecs/src/assets/prefabs/general_prefabs.rs)

## Making a new custom component

One of the central points of TurbECS is to make sure that developers can generate their own components, and then implement their own custom functions with those components as well!

### Making a component data structure

The first step to making a component is to make a desired data structure. Any component made must have:

1. turbo::serialize
2. derive(PartialEq)

In practice, it looks like this:

```
#[turbo::serialize]
#[derive(PartialEq)]
pub struct TextBoxResizerComponent {
    pub w_buffer : u32,
    pub h_buffer : u32
}
```
- Source : [Click here!](../../turbecs/src/assets/components/misc_components/comp_textbox_resizer.rs)

### Adding components to the ComponentData enum

Next, you'll need to:

1. Import the desired struct at the top of the [components](../../turbecs/src/turbecs/component_system/component.rs) file
2. Make a new enum option within the ComponentData enum that holds the recently made struct internally

In practice, adding an enum option looks like this:
```
#[turbo::serialize]
#[derive(PartialEq)]
pub enum ComponentData {
    Rectangle ( RectangleComponent ),
    Text ( TextComponent ),
    TextBox ( TextBoxComponent ),
    Sprite ( SpriteComponent ),
    Button ( ButtonComponent ),
    Particle ( ParticleComponent ),

    // User made components

    Resizer (ResizerComponent),
    TextBoxResizer (TextBoxResizerComponent),
    Fade (FadeComponent),
    TextBoxFiller (TextBoxFillerComponent),
    ScreenManager (ScreenManagerComponent),
}
```
- Source : [click here!](./../../turbecs/src/turbecs/component_system/component.rs)

**Note:** By adding anything to the ComponentData enum, you'll be changing how TurbECS, and in turn, how the game understands some of the project data, which will lead to an automatic reset of the project.

### Adding to a component to the HasX System

This step must be done if you'd like any part of your component to interact with the lifetime system. This is due to the fact that TurbECS will automatically assume that components have nothing to do within a given lifetime function, unless stated otherwise.

In practice, it looks like this:

```
fn init_render(&mut self) {

    match &self.component_data {

        // Core functionality
        ComponentData::Button(_)        => {self.has.has_render = true;},
        ComponentData::Text(_)          => {self.has.has_render = true;},
        ComponentData::TextBox(_)       => {self.has.has_render = true;},
        ComponentData::Sprite(_)        => {self.has.has_render = true;},
        ComponentData::Rectangle(_)     => {self.has.has_render = true;},

        // Extra functionality
        
        _default => {}
    }

}
```
- Source : [Click here!](../../turbecs/src/turbecs/helpers/has_x.rs)

**Note:** If you update any has functionality, you will need to reload the current scene or game, a components HasX functionality only gets checked when it first gets made.

### Linking a component to the Lifetime System

Finally, once the HasX functionality for any given component has been updated, that's when you should finally connect whatever function you have made.

In practice, it looks like this:

```
pub fn on_update(&mut self, ent : &mut Entity, state : &mut GameState) {
    match &mut self.component_data {
        
        // Standard components

        ComponentData::Button( button_component ) => {
            button_component.update(ent, state);
        },

        // User made components

        ComponentData::TextBoxFiller(tb_filler_component) => {
            tb_filler_component.update(ent, state);
        },

        ComponentData::ScreenManager(screen_manager_component) => {
            screen_manager_component.update(state);
        },

        ComponentData::Fade(fade_component) => {
            fade_component.update(ent, state);
        },
        
        _default => {}            
    }
}
```
- Source : [Click here!](../../turbecs/src/turbecs/component_system/component.rs)

**Note:** If you add OnAwake or OnStart functionality, you will need to reload the current scene or game, as these functions only get called when a component is first loaded.

---

Last updated: 10/17/25