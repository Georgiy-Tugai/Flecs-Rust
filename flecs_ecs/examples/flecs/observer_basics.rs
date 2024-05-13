use crate::z_snapshot_test::*;
snapshot_test!();
use flecs_ecs::prelude::*;

#[derive(Debug, Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[test]
fn main() {
    let world = World::new();

    //ignore snap in example, it's for snapshot testing
    world.import::<Snap>();

    // Create an observer for three events
    world
        .observer::<flecs::OnAdd, &Position>()
        .add_event::<flecs::OnRemove>() //or .add_event_id(OnRemove::ID)
        .add_event::<flecs::OnSet>()
        .each_iter(|it, index, pos| {
            if it.event() == flecs::OnAdd::ID {
                // No assumptions about the component value should be made here. If
                // a ctor for the component was registered it will be called before
                // the EcsOnAdd event, but a value assigned by set won't be visible.
                fprintln!(
                    it,
                    " - OnAdd: {}: {}",
                    it.event_id().to_str(),
                    it.entity(index)
                );
            } else {
                fprintln!(
                    it,
                    " - {}: {}: {}: with {:?}",
                    it.event().name(),
                    it.event_id().to_str(),
                    it.entity(index),
                    pos
                );
            }
        });

    // Create entity, set Position (emits EcsOnAdd and EcsOnSet)
    let entity = world.entity_named(c"e1").set(Position { x: 10.0, y: 20.0 });

    // Remove Position (emits EcsOnRemove)
    entity.remove::<Position>();

    // Remove Position again (no event emitted)
    entity.remove::<Position>();

    world.get::<&Snap>(|snap| 
        snap.test("observer_basics".to_string()));

    // Output:
    //  - OnAdd: Position: e1
    //  - OnSet: Position: e1: with Position { x: 10.0, y: 20.0 }
    //  - OnRemove: Position: e1: with Position { x: 10.0, y: 20.0 }
}
