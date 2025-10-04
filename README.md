# TurbECS - A Turbo Entity Component System

Welcome to the TurbECS repository, an open-source codebase that works on making an Entity Component System possible within the Turbo ecosystem!

## What's in the box?

Natively, TurbECS works with the Turbo game state to keep track of any entities and components, and store their data easily. 

This is done two main steps. The vectors and structs tossed into the gamestate, and the lifetime functions built into the engine.

### The extra data

When it comes to the extra data, they tend to fall into their own groups. This data tends to be kept under it's own manger within that game state, as shown here: 

```
#[turbo::game]
struct GameState {
    
    pub scene_data : SceneData,
    pub entity_manager : EntityManager,
    pub render_manager : Vec<Vec<usize>>,

    ...

}
```

### The lifetime cycle

On top of this, through the TurbECS lifecycle, the code will iterate through each lifetime function every frame, and an entity has a component that is within that lifetime function, then the code will complete the assocaited function for the attached components!

To look at some code...

```
// Standard Turbo update block w/ TurbECS funcs tossed in 

impl GameState {
    
    ...

    fn update(&mut self) {
        // Update the game & draw stuff

        self.check_scene_state();

        self.run_lifetime();

    }
} ...

// List of everything in the lifetime system!

impl GameState {

    fn run_lifetime(&mut self) {

        self.load_entities();
        self.on_awake();
        self.on_start();
        self.on_update();
        self.on_destroy();
        self.on_render();

    } ...

}

// Code will then go on and handle each entity as seen fit!
```

But to sum that code down:

1. The code starts in the basic Turbo update loop every frame
2. Within that update loop, the gamestate will run all lifetime functions in order:
    1. load_entities()
    2. on_awake()
    3. on_start()
    4. on_update()
    5. on_destroy()
    6. on_render()
3. In each lifetime function, loop through all entities, and if they have something to do within the functions, then the associated functions will be called!

And to get a sense of functions getting called every frame...

1. update(&mut self (GameState) )
    - called every frame natively due to Turbo
2. run_lifetime(&mut self (GameState) )
    - placed within the basic update function
    - calls each lifetime function
3. on_???(&mut self (GameState) ) 
    - loops through each entity/entity that falls within this lifetime cycle
    - only if it has something that gets changed/rendered in this lifetime func
4. on_???(&self (Entity) , _state : &mut GameState)
    - loops through all attached components
    - if the component getting looked at has something within this current lifetime cycle, then the code will move forward
5. on_???(&self (Component) , _state : &mut GameState)
    - Looks at the associated match case for the component type to see how to best move forward

---
### For extra info, check the docs!

- [Click here to check them out](./docs/README.md)

--- 

### Special Thanks To:

- The entire Turbo team
- Josiah Savary
- Alex Feigenbaum

--- 

Last updated : 10/4/2025