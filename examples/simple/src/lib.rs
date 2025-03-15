mod client_grpc;
use godot::prelude::*;

use bevy::{
    app::{App, Update},
    ecs::system::Res,
    prelude::{
        in_state, AppExtStates, Commands, IntoSystemConfigs, OnEnter, Query, Resource, States,
    },
    state::app::StatesPlugin,
};
use bevy_godot4::prelude::{
    bevy_app, AsPhysicsSystem, ErasedGd, ErasedGdResource, GodotScene, SystemDeltaTimer,
};
use godot::{
    builtin::Vector2,
    classes::{ResourceLoader, Sprite2D},
};
use godot::{init::ExtensionLibrary, prelude::gdextension};
// use helloworld::greeter_client::GreeterClient;
// use helloworld::client_grpc::HelloRequest;
// use helloworld::client_grpc::HelloReply;

use tonic_web_wasm_client::Client;
use crate::client_grpc::{
    HelloRequest,
    HelloReply,
    greeter_client::GreeterClient
};

#[derive(Debug, Default, Clone, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Playing,
}

#[bevy_app]
fn build_app(app: &mut App) {
    godot_print!("Player ready!");
    //perform_get_message_grpc();
    app.add_plugins(StatesPlugin)
        .init_state::<GameState>()
        .init_resource::<MyAssets>()
        .add_systems(OnEnter(GameState::Playing), spawn_sprite)
        .add_systems(
            Update,
            move_sprite
                .as_physics_system()
                .run_if(in_state(GameState::Playing)),
        );
      
}

#[derive(Resource, Debug)]
pub struct MyAssets {
    pub sprite: ErasedGdResource,
}

impl Default for MyAssets {
    fn default() -> Self {
        let mut resource_loader = ResourceLoader::singleton();
        let sprite = ErasedGdResource::new(resource_loader.load("sprite.tscn").unwrap());

        Self { sprite }
    }
}
#[warn(unused_must_use)]
fn spawn_sprite(mut commands: Commands, assets: Res<MyAssets>) {
    godot_print!("Player ready!");

    commands.spawn(
        GodotScene::from_resource(assets.sprite.clone())
            .with_translation2d(Vector2 { x: 200.0, y: 200.0 }),
    );
    godot_print!("Player ready!");
    //perform_get_message_grpc();
}

fn move_sprite(mut sprite: Query<&mut ErasedGd>, mut delta: SystemDeltaTimer) {
    if let Ok(mut sprite) = sprite.get_single_mut() {
        let mut sprite = sprite.get::<Sprite2D>();
        let delta = delta.delta_seconds() * 20.0;
        let position = sprite.get_position();

        sprite.set_position(Vector2 {
            x: position.x + delta,
            y: position.y + delta,
        });
    }
}

pub async fn perform_get_message_grpc() -> Result<String, String> {
    let base_url = "http://localhost:5051";

    let mut query_client = GreeterClient::new(
        Client::new(base_url.to_string())
    );

    // creating a new Request
    let request = tonic::Request::new(
        HelloRequest {
            name: "Rust Client".to_string(),
        },
    );
 
    // sending request and waiting for response
    let response: HelloReply = match query_client.say_hello(request).await {
        Ok(resp) => resp.into_inner(),
        Err(err) => {
            return Err(err.to_string())
        }
    };

    if response.message != "" {
        Ok(response.message)
    } else {
        Err(response.message)
    }
}