use bevy::app::{App, Startup, Update};
use bevy::asset::{AssetServer, Handle};
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::RunSystemOnce;
use bevy::prelude::{
    Component, Entity, IntoSystem, Message, MessageReader, Messages, Mut, Resource, Time,
};
use bevy::text::Font;
use bevy::MinimalPlugins;
use std::error::Error;
use std::time::Duration;

pub fn get_update_systems(app: &App) -> impl Iterator {
    app.get_schedule(Update)
        .expect("Update schedule not found")
        .systems()
        .expect("No systems found")
}

pub fn get_startup_systems(app: &App) -> impl Iterator {
    app.get_schedule(Startup)
        .expect("Startup schedule not found")
        .systems()
        .expect("No systems found")
}

pub fn contains_system(app: &App, schedule: impl ScheduleLabel, name: &str) -> bool {
    app.get_schedule(schedule)
        .expect("Startup schedule not found")
        .systems()
        .expect("No systems found")
        .any(|system| system.1.name().shortname().to_string() == name)
}

pub fn get_resource_or_fail<T: Resource>(app: &mut App) -> &T {
    app.world()
        .get_resource::<T>()
        .expect("Resource not found in world")
}

pub fn get_resource_mut<T: Resource>(app: &mut App) -> Mut<'_, T> {
    app.world_mut()
        .get_resource_mut::<T>()
        .expect("Resource not found in world")
}

pub fn get_component<T: Component>(app: &mut App, entity: Entity) -> &T {
    app.world()
        .get::<T>(entity)
        .expect("Cannot get component for entity")
}

pub fn contains_message<T: Message>(app: &App) -> bool {
    app.world().contains_resource::<Messages<T>>()
}

pub fn contains_entity(app: &App, entity: Entity) -> bool {
    app.world().get_entity(entity).is_ok()
}

pub fn contains_component<T: Component>(app: &mut App) -> bool {
    app.world_mut().query::<&T>().iter(app.world()).len() == 1
}

pub fn component_despawned<T: Component>(app: &mut App) -> bool {
    app.world_mut().query::<&T>().iter(app.world()).len() == 0
}

pub fn count_components<T: Component>(app: &mut App) -> usize {
    app.world_mut().query::<&T>().iter(app.world()).count()
}

pub fn verify_message_fired<T: Message>(app: &mut App) -> Result<(), Box<dyn Error>> {
    app.world_mut()
        .run_system_once(move |mut reader: MessageReader<T>| {
            let mut iterator = reader.read();

            iterator
                .next()
                .unwrap_or_else(|| panic!("Message did not arrive!"));
        })
        .map_err(|e| format!("Cannot run system: {e}"))?;
    Ok(())
}

pub fn spawn_dummy_entity(app: &mut App) -> Entity {
    app.world_mut().spawn_empty().id()
}

pub fn send_message<T: Message>(app: &mut App, message: T) {
    app.world_mut().write_message(message);
}

pub fn advance_time_by_seconds(app: &mut App, seconds: f32) {
    let mut time = app.world_mut().resource_mut::<Time>();
    time.advance_by(Duration::from_secs_f32(seconds));
}

pub fn run_system<T, Out, Marker>(app: &mut App, system: T) -> Result<Out, String>
where
    T: IntoSystem<(), Out, Marker>,
{
    app.world_mut()
        .run_system_once(system)
        .map_err(|e| format!("Cannot run system: {e}"))
}

pub fn dummy_font(app: &App) -> Handle<Font> {
    let asset_server = app.world().resource::<AssetServer>().clone();
    asset_server.load("test.ttf")
}

pub fn minimal_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app
}
