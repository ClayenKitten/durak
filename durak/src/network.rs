//! Network requests to server.

mod requests;

pub use requests::*;

use std::{fmt::Debug, marker::PhantomData};

use bevy::prelude::*;
use bevy_mod_reqwest::{
    reqwest::{header::HeaderMap, Method, Url},
    *,
};

use serde::{de::DeserializeOwned, Serialize};

/// Plugin that manages all network functionality of the game.
pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ReqwestPlugin)
            .init_resource::<ReqwestClient>()
            .add_plugins(RequestPlugin::<CreateGameRequest>::new())
            .add_plugins(RequestPlugin::<JoinGameRequest>::new());
    }
}

/// Generic plugin that handles requests of specific type.
///
/// It adds systems that handle requests and [OnResponce] event.
struct RequestPlugin<R: MyRequest>(PhantomData<R>);

impl<R: MyRequest> RequestPlugin<R> {
    pub fn new() -> Self {
        RequestPlugin(PhantomData::<R>)
    }
}

impl<R> Plugin for RequestPlugin<R>
where
    R: MyRequest + Component + Send + Sync,
{
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            FixedUpdate,
            (send_request::<R>, handle_responses::<R>).chain(),
        )
        .add_event::<OnResponce<R>>();
    }
}

fn send_request<R: MyRequest + Component>(
    client: Res<ReqwestClient>,
    mut commands: Commands,
    requests: Query<(Entity, &R), Added<R>>,
) {
    for (entity, request) in requests.iter() {
        let mut builder = client
            .0
            .request(Method::POST, request.url())
            .headers(request.headers());
        if let Some(ref query) = request.query() {
            builder = builder.query(query);
        }
        let request = builder
            .build()
            .expect("request should be built successfully");
        commands.entity(entity).insert(ReqwestRequest::new(request));
    }
}

fn handle_responses<R: MyRequest + Component>(
    mut commands: Commands,
    results: Query<(Entity, &ReqwestBytesResult), With<R>>,
    mut event_writer: EventWriter<OnResponce<R>>,
) {
    for (entity, res) in results.iter() {
        let str = res.as_str().unwrap();
        let responce: R::Responce = serde_json::from_str(str).unwrap();
        event_writer.send(OnResponce(responce));
        commands.entity(entity).despawn_recursive();
    }
}

/// Convenience trait that is used to map various custom requests into reqwest's request.
pub trait MyRequest {
    // TODO: make base url changeable.
    const URL: &'static str = "http://127.0.0.1:3000";

    /// Type that will be returned by the server.
    type Responce: Debug + DeserializeOwned + Send + Sync;

    /// Type that will be serialized into query params.
    type Query: Serialize;

    fn url(&self) -> Url;

    fn query(&self) -> Option<Self::Query> {
        None
    }

    fn headers(&self) -> HeaderMap {
        HeaderMap::new()
    }
}

/// Event that is being fired when responce for `R` has arrived.
#[derive(Debug, Event)]
pub struct OnResponce<R: MyRequest>(pub R::Responce);
