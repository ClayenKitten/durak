use bevy::prelude::*;
use bevy_mod_reqwest::reqwest::{
    header::{HeaderMap, AUTHORIZATION},
    Method, Url,
};
use durak_lib::{
    game::card::Card,
    network::{AuthHeader, CreateGameData, CreateGameResponse, JoinGameData, JoinGameResponse},
    status::{GameState, RoundStatus},
};

use super::MyRequest;

#[derive(Debug, Component)]
pub struct CreateGameRequest(pub CreateGameData);

impl MyRequest for CreateGameRequest {
    type Response = CreateGameResponse;

    type Query = [(&'static str, String); 2];

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/create", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn query(&self) -> Option<Self::Query> {
        Some([
            ("name", self.0.name.clone()),
            ("password", self.0.password.clone()),
        ])
    }
}

#[derive(Debug, Component)]
pub struct JoinGameRequest(pub JoinGameData);

impl MyRequest for JoinGameRequest {
    type Response = JoinGameResponse;

    type Query = [(&'static str, String); 3];

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/join", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn query(&self) -> Option<Self::Query> {
        Some([
            ("id", self.0.id.0.to_string()),
            ("name", self.0.name.clone()),
            ("password", self.0.password.clone()),
        ])
    }
}

#[derive(Debug, Component)]
pub struct LeaveGameRequest(pub AuthHeader);

impl MyRequest for LeaveGameRequest {
    type Response = ();

    type Query = ();

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/leave", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
#[deprecated]
pub struct StateRequest(pub AuthHeader);

impl MyRequest for StateRequest {
    type Response = GameState;

    type Query = ();

    fn method(&self) -> Method {
        Method::GET
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/state", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct StatusRequest(pub AuthHeader);

impl MyRequest for StatusRequest {
    type Response = RoundStatus;

    type Query = ();

    fn method(&self) -> Method {
        Method::GET
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/status", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct StartGameRequest(pub AuthHeader);

impl MyRequest for StartGameRequest {
    type Response = ();

    type Query = ();

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/start", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct PlayCardRequest {
    pub auth: AuthHeader,
    pub card: Card,
}

impl MyRequest for PlayCardRequest {
    type Response = ();

    type Query = Card;

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/play", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn query(&self) -> Option<Self::Query> {
        Some(self.card)
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.auth.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct TakeRequest(pub AuthHeader);

impl MyRequest for TakeRequest {
    type Response = ();

    type Query = ();

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/take", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}

#[derive(Debug, Component)]
pub struct RetreatRequest(pub AuthHeader);

impl MyRequest for RetreatRequest {
    type Response = ();

    type Query = ();

    fn method(&self) -> Method {
        Method::POST
    }

    fn url(&self) -> Url {
        let url = format!("{}/game/retreat", Self::URL);
        Url::parse(&url).unwrap()
    }

    fn headers(&self) -> HeaderMap {
        let mut map = HeaderMap::new();
        map.insert(AUTHORIZATION, self.0.into_header());
        map
    }
}
