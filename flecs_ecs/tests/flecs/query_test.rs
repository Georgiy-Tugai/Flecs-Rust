#![allow(dead_code)]
use flecs_ecs::core::*;
use flecs_ecs::macros::*;
use flecs_ecs::newtype_of_entity_view;
use lending_iterator::prelude::HKT;
use lending_iterator::LendingIterator as _;

use crate::common_test::*;

#[test]
fn query_uncached_destruction_no_panic() {
    let world = World::new();
    let query = world.new_query::<&Tag>();
    let query2 = query.clone();
    drop(query);
    query2.run(|mut it| {
        dbg!(it.iter_mut().flags & flecs_ecs::sys::EcsIterIsValid != 0);
        while it.next() {}
        dbg!(it.iter_mut().flags & flecs_ecs::sys::EcsIterIsValid != 0);
    });
    drop(query2);
}

#[test]
#[should_panic]
fn query_cached_destruction_lingering_references_panic() {
    let world = World::new();
    let query = world.query::<&Tag>().set_cached().build();
    let query2 = query.clone();
    query.destruct();
    query2.run(|_| {});
    drop(query2);
}

#[test]
fn query_iter_stage() {
    #[derive(Component, Debug)]
    struct Comp(usize);

    let world = World::new();
    world.set_threads(4);

    let query = world.new_query::<&Comp>();

    for i in 0..4 {
        world.entity().set(Comp(i));
    }

    world
        .system::<&Comp>()
        .multi_threaded()
        .each_entity(move |e, _| {
            query.iter_stage(e).each(|_vel| {});
        });

    world.progress();
}

#[test]
#[should_panic]
fn query_panic_inside() {
    let world = World::new();
    let query = world.query::<&Tag>().build();
    query.run(|_| {
        panic!();
    });
}

#[test]
fn query_run_sparse() {
    let world = World::new();

    world.component::<Position>().add_trait::<flecs::Sparse>();
    world.component::<Velocity>();

    let entity = world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    let q = world.query::<(&mut Position, &Velocity)>().build();

    q.run(|mut it| {
        while it.next() {
            let v = it.field::<Velocity>(1).unwrap();

            for i in it.iter() {
                let p = it.field_at_mut::<Position>(0, i).unwrap();
                p.x += v[i].x;
                p.y += v[i].y;
            }
        }
    });

    entity.get::<&Position>(|p| {
        assert_eq!(p.x, 11);
        assert_eq!(p.y, 22);
    });
}

#[test]
fn query_each_sparse() {
    let world = World::new();

    world.component::<Position>().add_trait::<flecs::Sparse>();
    world.component::<Velocity>();

    let entity = world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    let q = world.query::<(&mut Position, &Velocity)>().build();

    q.each(|(p, v)| {
        p.x += v.x;
        p.y += v.y;
    });

    entity.get::<&Position>(|p| {
        assert_eq!(p.x, 11);
        assert_eq!(p.y, 22);
    });
}

#[test]
fn query_each_iterator() {
    let world = World::new();

    world.component::<Position>();
    world.component::<Velocity>();

    let entity = world
        .entity()
        .set(Position { x: 10, y: 20 })
        .set(Velocity { x: 1, y: 2 });

    let q = world.query::<(&mut Position, &Velocity)>().build();

    let it = q.into_each();
    it.for_each(|(p, v)| {
        p.x += v.x;
        p.y += v.y;
    });

    let it = q.into_each();
    it.for_each(|(p, v)| {
        p.x += v.x;
        p.y += v.y;
    });
    // dbg!(it
    //     .map_to_ref(|[], t| t.1)
    //     .map_into_iter(Clone::clone)
    //     .collect::<Vec<_>>());

    q.into_each_iter()
        .map::<HKT!((&mut Position, &Velocity)), _>(|[], t| t.2)
        .for_each(|t| {
            dbg!(t);
        });

    // while let Some((_it, _idx, (p, v))) = it.next() {
    //     p.x += v.x;
    //     p.y += v.y;
    // }

    // q.each(|(p, v)| {
    //     p.x += v.x;
    //     p.y += v.y;
    // });

    entity.get::<&Position>(|p| {
        assert_eq!(p.x, 11);
        assert_eq!(p.y, 22);
    });
}

use lending_iterator::prelude::*;
macro_rules! implLendingIterator {
    (for<$lt:lifetime> $t:ty) => {impl LendingIterator + for<$lt> LendingIteratorඞItem<$lt, T = $t>}
}

#[test]
fn blah() {
    newtype_of_entity_view!(struct Thing(EntityView));
    struct FoosIndex(u32);

    #[derive(Component)]
    struct Foos {
        foos: Vec<Foo>,
    };
    struct Foo();

    struct FooState(u32);

    #[derive(Component)]
    struct FoosState {
        states: Vec<FooState>,
    }

    impl Thing<'_> {
        pub fn each_foo<F: FnMut(FoosIndex, &Foo, &FooState)>(&self, mut cb: F) {
            if let Some(bar) = self.target::<&Bar>(0) {
                self.try_get::<&FoosState>(move |state| {
                    bar.try_get::<&Foos>(move |foos| {
                        for (idx, baz) in foos.foos.iter().enumerate() {
                            cb(FoosIndex(idx as u32), baz, &state.states[idx]);
                        }
                    });
                });
            }
        }

        pub fn foo_iter2(
            &self,
        ) -> implLendingIterator!(for<'next> (FoosIndex, &'next Foo, &'next FooState)) {
            let mut foos = self.target::<&Bar>(0).unwrap().get_ref::<&Foos>();
            let mut state = self.get_ref::<&FoosState>();
            let count = foos.try_get(|foos| foos.foos.len()).unwrap_or(0);

            lending_iterator::from_fn::<HKT!((FoosIndex, &Foo, &FooState)), _, _>(
                0usize,
                move |idx| {
                    if *idx >= count {
                        return None;
                    }
                    let ret = (
                        FoosIndex(*idx as u32),
                        foos.try_get(|foos| &foos.foos[*idx])?,
                        state.try_get(|state| &state.states[*idx])?,
                    );
                    *idx += 1;
                    Some(ret)
                },
            )
        }

        pub fn foo_iter3(&self) -> impl Iterator + use<'_> {
            let mut foos = self.target::<&Bar>(0).unwrap().get_ref::<&Foos>();
            let count = foos.try_get(|foos| foos.foos.len()).unwrap_or(0);
            let mut state = self.get_ref::<&FoosState>();

            (0..count).map(move |idx| {
                (
                    FoosIndex(idx as u32),
                    foos.try_get(|foos| &foos.foos[idx]).unwrap(),
                    state.try_get(|state| &state.states[idx]).unwrap(),
                )
            })
        }
    }
}

#[test]
fn query_iter_targets() {
    let world = World::new();

    let likes = world.entity();
    let pizza = world.entity();
    let salad = world.entity();
    let alice = world.entity().add_id((likes, pizza)).add_id((likes, salad));

    let q = world.query::<()>().with_second::<flecs::Any>(likes).build();

    let mut count = 0;
    let mut tgt_count = 0;

    q.each_iter(|mut it, row, _| {
        let e = it.entity(row);
        assert_eq!(e, alice);

        it.targets(0, |tgt| {
            if tgt_count == 0 {
                assert_eq!(tgt, pizza);
            }
            if tgt_count == 1 {
                assert_eq!(tgt, salad);
            }
            tgt_count += 1;
        });

        count += 1;
    });

    assert_eq!(count, 1);
    assert_eq!(tgt_count, 2);
}

#[test]
fn query_iter_targets_second_field() {
    let world = World::new();

    let likes = world.entity();
    let pizza = world.entity();
    let salad = world.entity();
    let alice = world
        .entity()
        .add::<Position>()
        .add_id((likes, pizza))
        .add_id((likes, salad));

    let q = world
        .query::<&Position>()
        .with_second::<flecs::Any>(likes)
        .build();

    let mut count = 0;
    let mut tgt_count = 0;

    q.each_iter(|mut it, row, _| {
        let e = it.entity(row);
        assert_eq!(e, alice);

        it.targets(1, |tgt| {
            if tgt_count == 0 {
                assert_eq!(tgt, pizza);
            }
            if tgt_count == 1 {
                assert_eq!(tgt, salad);
            }
            tgt_count += 1;
        });

        count += 1;
    });

    assert_eq!(count, 1);
    assert_eq!(tgt_count, 2);
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn query_iter_targets_field_out_of_range() {
    let world = World::new();

    let likes = world.entity();
    let pizza = world.entity();
    let salad = world.entity();
    let alice = world.entity().add_id((likes, pizza)).add_id((likes, salad));

    let q = world.query::<()>().with_second::<flecs::Any>(likes).build();

    q.each_iter(|mut it, row, _| {
        let e = it.entity(row);
        assert_eq!(e, alice);

        it.targets(1, |_| {});
    });
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn query_iter_targets_field_not_a_pair() {
    let world = World::new();

    let likes = world.entity();
    let pizza = world.entity();
    let salad = world.entity();
    let alice = world
        .entity()
        .add::<Position>()
        .add_id((likes, pizza))
        .add_id((likes, salad));

    let q = world.query::<&Position>().build();

    q.each_iter(|mut it, row, _| {
        let e = it.entity(row);
        assert_eq!(e, alice);

        it.targets(1, |_| {});
    });
}

#[test]
#[should_panic]
#[cfg(debug_assertions)]
fn query_iter_targets_field_not_set() {
    let world = World::new();

    let likes = world.entity();
    let alice = world.entity().add::<Position>();

    let q = world
        .query::<&Position>()
        .with_second::<flecs::Any>(likes)
        .optional()
        .build();

    q.each_iter(|mut it, row, _| {
        let e = it.entity(row);
        assert_eq!(e, alice);
        assert!(!it.is_set(1));

        it.targets(1, |_| {});
    });
}
