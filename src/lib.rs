use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CollisionEventPlugin;

impl Plugin for CollisionEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionAbsorberEvent>()
            .add_systems(Update, detect_collision);
    }
}

#[derive(Component)]
pub struct CollisionAbsorber;

#[derive(Event)]
pub struct CollisionAbsorberEvent {
    pub is_exit: bool,
    pub entity: Entity,
}

pub fn spawn_in_collision<F>(
    commands: &mut Commands,
    collision_pos: Vec2,
    collision_size: Vec2,
    ui: impl Bundle,
    spawn_children: F,
) where
    F: FnOnce(&mut ChildBuilder),
{
    commands
        .spawn((CollisionAbsorber, ui))
        .with_children(|children: &mut ChildBuilder<'_>| {
            children.spawn((
                Collider::cuboid(collision_size.x, collision_size.y),
                GlobalTransform::from_xyz(collision_pos.x, collision_pos.y, 20.),
                Sensor::default(),
                ActiveEvents::COLLISION_EVENTS,
            ));
            spawn_children(children);
        });
}

pub fn detect_collision(
    mut child: Query<Entity, (With<Parent>, With<Collider>)>,
    mut parent: Query<(Entity, &mut Visibility), With<CollisionAbsorber>>,
    mut contact_events: EventReader<CollisionEvent>,
    mut ev_wr: EventWriter<CollisionAbsorberEvent>,
) {
    parent
        .iter_mut()
        .for_each(|(entity_parent, mut visibility)| {
            child.iter_mut().for_each(|entity| {
                contact_events.read().for_each(|collision| {
                    match collision {
                        CollisionEvent::Started(h1, h2, _) => {
                            if h1 == &entity || h2 == &entity {
                                *visibility = Visibility::Visible;
                                ev_wr.send(CollisionAbsorberEvent {
                                    is_exit: false,
                                    entity: entity_parent,
                                });
                            }
                        }
                        CollisionEvent::Stopped(h1, h2, _) => {
                            if h1 == &entity || h2 == &entity {
                                *visibility = Visibility::Hidden;
                                ev_wr.send(CollisionAbsorberEvent {
                                    is_exit: true,
                                    entity: entity_parent,
                                });
                            }
                        }
                    };
                });
            });
        });
}
