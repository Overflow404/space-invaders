use bevy::MinimalPlugins;
use bevy::app::{App, Plugin, PluginGroup};
use bevy::asset::{AssetPlugin, AssetServer, Handle};
use bevy::ecs::schedule::ScheduleLabel;
use bevy::ecs::system::RunSystemOnce;
use bevy::image::Image;
use bevy::input::ButtonInput;
use bevy::prelude::{
    AssetApp, Component, Entity, KeyCode, Message, MessageReader, Messages, Mut, Resource, Text,
    Time, Transform, Vec3, With,
};
use bevy::text::Font;
use bevy::time::TimePlugin;
use std::time::Duration;

pub fn smoke_test_plugin<P: Plugin>(plugin: P) -> App {
    TestAppBuilder::new().with_plugin(plugin).build()
}

pub fn smoke_test_plugin_with_assets<P: Plugin>(plugin: P) -> App {
    TestAppBuilder::new()
        .with_assets()
        .with_plugin(plugin)
        .build()
}

pub fn contains_system_or_fail(app: &App, schedule: impl ScheduleLabel, name: &str) -> bool {
    app.get_schedule(schedule)
        .expect("Schedule not found")
        .systems()
        .expect("No systems found")
        .any(|system| system.1.name().shortname().to_string() == name)
}

pub fn get_resource_or_fail<T: Resource>(app: &mut App) -> &T {
    app.world()
        .get_resource::<T>()
        .expect("Resource not found in world")
}

pub fn get_resource_mut_or_fail<T: Resource>(app: &mut App) -> Mut<'_, T> {
    app.world_mut()
        .get_resource_mut::<T>()
        .expect("Resource not found in world")
}

pub fn get_component_or_fail<T: Component>(app: &mut App, entity: Entity) -> &T {
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

pub fn contains_single_component<T: Component>(app: &mut App) -> bool {
    app.world_mut().query::<&T>().iter(app.world()).len() == 1
}

pub fn did_component_despawn<T: Component>(app: &mut App) -> bool {
    app.world_mut().query::<&T>().iter(app.world()).len() == 0
}

pub fn despawn(app: &mut App, entity: Entity) -> bool {
    app.world_mut().despawn(entity)
}

pub fn count_components<T: Component>(app: &mut App) -> usize {
    app.world_mut().query::<&T>().iter(app.world()).count()
}

pub fn did_message_fire<T: Message>(app: &mut App) -> bool {
    app.world_mut()
        .run_system_once(move |mut reader: MessageReader<T>| reader.read().next().is_some())
        .unwrap_or(false)
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

pub fn dummy_font(app: &App) -> Handle<Font> {
    let asset_server = app.world().resource::<AssetServer>().clone();
    asset_server.load("test.ttf")
}

pub fn minimal_app(disable_time_plugin: bool) -> App {
    let mut app = App::new();
    app.add_plugins(match disable_time_plugin {
        true => MinimalPlugins.build().disable::<TimePlugin>(),
        false => MinimalPlugins.build(),
    });
    app
}

pub fn query_single_transform<T: Component>(app: &mut App) -> Vec3 {
    app.world_mut()
        .query_filtered::<&Transform, With<T>>()
        .single(app.world())
        .expect("Component not found or multiple instances exist")
        .translation
}

pub fn query_single_text<T: Component>(app: &mut App) -> String {
    app.world_mut()
        .query_filtered::<&Text, With<T>>()
        .single(app.world())
        .expect("Text component not found or multiple instances exist")
        .0
        .clone()
}

pub fn assert_text_equals<T: Component>(app: &mut App, expected: &str) {
    let actual = query_single_text::<T>(app);
    assert_eq!(actual, expected, "Text component value mismatch");
}

pub fn get_single_entity<T: Component>(app: &mut App) -> Entity {
    app.world_mut()
        .query_filtered::<Entity, With<T>>()
        .single(app.world())
        .expect("Component not found or multiple instances exist")
}

pub fn get_all_entities<T: Component>(app: &mut App) -> Vec<Entity> {
    app.world_mut()
        .query_filtered::<Entity, With<T>>()
        .iter(app.world())
        .collect()
}

pub struct TestAppBuilder {
    app: App,
    needs_update: bool,
    auto_update: bool,
}

impl TestAppBuilder {
    pub fn new() -> Self {
        Self {
            app: minimal_app(false),
            needs_update: false,
            auto_update: true,
        }
    }

    pub fn with_time_disabled() -> Self {
        Self {
            app: minimal_app(true),
            needs_update: false,
            auto_update: true,
        }
    }

    pub fn without_auto_update(mut self) -> Self {
        self.auto_update = false;
        self
    }

    pub fn with_plugin<P: Plugin>(mut self, plugin: P) -> Self {
        self.app.add_plugins(plugin);
        self.needs_update = true;
        self
    }

    pub fn with_message<M: Message>(mut self) -> Self {
        self.app.add_message::<M>();
        self
    }

    pub fn with_assets(mut self) -> Self {
        self.app
            .add_plugins(AssetPlugin::default())
            .init_asset::<Image>()
            .init_asset::<Font>();
        self
    }

    pub fn with_input(mut self) -> Self {
        self.app.init_resource::<ButtonInput<KeyCode>>();
        self
    }

    pub fn with_time(mut self) -> Self {
        self.app.init_resource::<Time>();
        self
    }

    pub fn with_setup<F>(mut self, setup_fn: F) -> Self
    where
        F: FnOnce(&mut App),
    {
        setup_fn(&mut self.app);
        self
    }

    pub fn build(mut self) -> App {
        if self.needs_update && self.auto_update {
            self.app.update();
        }
        self.app
    }
}

impl Default for TestAppBuilder {
    fn default() -> Self {
        Self::new()
    }
}
