# TurbECS - A Turbo Entity Component System

Welcome to the TurbECS repository, an open-source codebase that works on making an Entity Component System possible within the Turbo ecosystem!

## What's in the box?

Natively, TurbECS works with the Turbo game state to keep track of any entities and components, and store their data easily. 

This is done two main steps. The vectors and structs tossed into the GameState, and the lifetime functions built into the engine.

### The extra data

When it comes to the extra data, they tend to fall into their own groups. This data tends to be kept under it's own manger within that game state, as shown here: 

```
#[turbo::game]
struct GameState {
    
    pub scene_manager : SceneManager,
    pub entity_manager : EntityManager,
    pub component_manager : ComponentManager,
    pub render_manager : Vec<Vec<usize>>,

    ...

}
```

### The lifetime cycle

On top of this, through the TurbECS lifecycle, the code will iterate through each lifetime function every frame, and an entity has a component that is within that lifetime function, then the code will complete the associated function for the attached components!

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
2. Within that update loop, the GameState will run all lifetime functions in order:
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
5. on_???(&self (Component) , ent : &mut Entity , state : &mut GameState)
    - Looks at the associated match case for the component type to see how to move forward

## How to make a project?

In order to make a project with TurbECS, there are two general paths you can take.

### Making a new project with TurbeECS (~5 minutes)

#### Completion time : ~5 minutes

Of the two paths, this is pretty easy!

All you need to do is download the main turbecs folder [(this is the folder for reference)](./turbecs/), drop the folder in a preferred directory, and start adding your own work from there!

- Note: it is heavily recommended that you rename the two folders titled turbecs to the name of your project!

### Implementing TurbECS into an existing Turbo Project 

#### Completion time : ~15 minutes

This'll take some work, but it shouldn't be too bad!

#### 1. Download folders + file!

Within the initial folder named the turbecs, download these folders + file:

- [folder: assets](./turbecs/src/assets/)
- [folder: turbecs](./turbecs/src/turbecs/)
- [file: ecs_gamestate.rs](./turbecs/src/ecs_gamestate.rs)

#### 2. Drop all the files into your src folder

Here's where Turbo expects all the code within a project to be placed, so make that the files are placed here!

#### 3. Add all necessary imports

In your lib.rs file, add this block at the top, to link the rest of the files to the lib file

```
// Links the driver file to the main lib file
mod ecs_gamestate;

// Initial TurbECS import
mod turbecs;

// Manager imports
use turbecs::{managers};

use managers::entity_manager::EntityManager;
use managers::component_manager::ComponentManager;
use managers::scene_manager::SceneManager;

// Community work/manager imports
use turbecs::{particles::ParticleManager};
```

#### 4. Update your GameState to hold TurbECS

In your GameState struct, add all necessary managers and optional managers that you'd like!

```
#[turbo::game]
struct GameState {
    
    // Core managers for turbecs

    pub scene_manager : SceneManager,
    pub entity_manager : EntityManager,
    pub component_manager : ComponentManager,
    pub render_manager : Vec<Vec<usize>>,

    // Community integrated work/managers

    pub particle_manager : ParticleManager,

    ...

}
```

#### 5. Update your new() to initialize TurbECS

Add any initializers for TurbECS managers to your new()!

```
impl GameState {
    fn new() -> Self {

        ...

        Self {
            
            // Core managers for turbecs

            scene_manager : SceneManager::new(),
            entity_manager : EntityManager::new(),
            component_manager : ComponentManager::new(),
            render_manager : Vec::with_capacity(10),
            
            // Community integrated work/managers

            particle_manager : ParticleManager::new(),

            ...

        }
    
    }

    ...

}
```


#### 6. Update your update() to hold TurbECS

Since all the driver code for TurbECS, this is all you'll need to add to the update() functions are these two functions!

```
impl GameState {

    ...

    fn update(&mut self) {
        
        // Checks the scene state before continuing

        self.check_scene_state();

        // From here TurbECS will run it's lifetime functions!

        self.run_lifetime();
        
        ...

    }

}
```

#### 7. Run the project!

Once all is said and done, run the project as you have before, you should be good to go!

---
### For extra info, check the docs!

- [Click here to check them out](./docs/README.md)

--- 

### Special Thanks To:

- The entire Turbo team
- Josiah Savary
- Alex Feigenbaum

--- 

Last updated : 10/10/2025