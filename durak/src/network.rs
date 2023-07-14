//! Network requests to server.

use bevy::prelude::*;
use bevy_mod_reqwest::ReqwestPlugin;

pub struct NetworkPlugin;

impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins(ReqwestPlugin);
    }
}
