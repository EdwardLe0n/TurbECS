// Initial imports

use std::collections::VecDeque;
use std::collections::BTreeMap;

mod turbe;
use turbe::{entity::Entity, component::{Component, ComponentLifecycle}};

mod assets;
use assets::prefabs;

use turbo::{canvas::rect::Rectangle, prelude::*};

#[turbo::game]
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
struct GameState {
    
    entities : BTreeMap<u32, Entity<Component>>

}

impl GameState {
    fn new() -> Self {

        let mut bTree = BTreeMap::new();

        let mut ent = prefabs::new_rect();

        bTree.insert(ent.locat, ent);

        ent = prefabs::new_spr();

        bTree.insert(ent.locat, ent);

        Self {entities : bTree}
    
    }

    fn update(&mut self) {
        // Update the game & draw stuff

        self.run_lifetime();

    }

    /*

        Lifetime System!!!

     */

    fn run_lifetime(&mut self) {

        self.on_update();
        self.on_render();

    }

    fn on_update(&mut self) {

        for (id, ent) in self.entities.iter_mut() {

            let mut entDraft = ent.clone();

            for components in ent.components.iter_mut() {

                components.on_update(&mut entDraft);

            }

            *ent = entDraft;

        }

    }

    fn on_render(&mut self) {

        clear(0xffffffff);

        for (id, ent) in self.entities.iter_mut() {

            for components in ent.components.iter_mut() {

                components.render(0, 0);

            }

        }

    }

}
