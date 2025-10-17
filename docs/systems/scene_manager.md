# Scene Manager

## Intro

The scene manager is one of the core managers within TurbECS that allows for:

1. The distinction of what scene is currently loaded
2. The ability to make new scenes by adding a new enum option
3. The ability to add new entities to a scene when the scene first loads

## Directory

- [Loading a scene](#loading-a-new-scene)
- [Adding a new scene enum](#making-a-new-scene-enum)
- [Making a new make_scene function](#making-a-new-make_scene-function)

---

## Loading a new scene

When loading a scene, you'll need a mutable reference to the GameState, as well as the enum of the scene you would like to load.

Then from there's the call is as simple as this!

```
// Using whatever variable GameState is in, call the load_scene function from the scene_manager!

state.scene_manager.load_scene(Scenes::Misc);
```

**NOTE:** When loading a new scene, this will natively delete any previously made entities! This is similar to running LoadScene in Unity with the Single option, instead of additive!

Example : Code snippet of a button acting as a scene loader
```
pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    _state.scene_manager.load_scene(Scenes::Misc);

}
```
- Source : [Click here!](../../turbecs/src/turbecs/component_system/components/buttons/scene_loader_buttons/to_misc.rs)

## Adding a new scene enum

Located withing the scene_manager file, there'll be an enum that looks like this:

```
#[turbo::serialize]
#[derive(Copy, PartialEq)]

pub enum Scenes {
    Title,
    Misc
}
```
- Source : [Click here!](../../turbecs/src/turbecs/managers/scene_manager.rs)

Feel free to add as many possible enum options as you'd like

## Making a new make_scene function

Making a new make_scene functions is fairly uncomplicated!

All you need to do is:
```
1. Make a function that has the return type of VecDeque<(Entity, VecDeque<Component>)>
2. Within that function, generate a VecDeque to return
3. Use push_back functionality to add as many entities to the scene
4. Return the VecDeque you've been adding to
5. Make sure to add 
```

As an example, here's a template make_scene function!

```
pub fn make_some_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    // VecDeque you'll be returning at the end of the function
    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(/*insert some prefab here*/);

    return ent_vec;

}
```
- Source : [Click here!](../../turbecs/src/turbecs/managers/scene_manager.rs)

Also, located in [this file here](../../turbecs/src/turbecs/managers/scene_manager.rs), make sure to update the match function to include your new enum and function.

```
pub fn make_scene (some_scene : Scenes) ->  VecDeque<(Entity, VecDeque<Component>)>{

    match some_scene {
        Scenes::Title => {return make_title_scene()},
        Scenes::Misc => {return make_misc_scene()},
        _default => {
            return VecDeque::new();
        }
    }

}
```
- Source : [Click here!](../../turbecs/src/turbecs/managers/scene_manager.rs)

---
Last updated: 10/17/2025