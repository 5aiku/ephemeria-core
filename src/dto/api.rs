// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

use serde::{Serialize, Deserialize};

// For future internal service like db connections
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Status {
    Up,
    Down,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HealthStatusResponse {
    pub status: Status,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestResponse {
    pub season_name: String,
    pub season_description: String,
    pub game_version: String,
    pub java_version: String,
    pub mod_loader: String,
    pub mods_hash: String,
    pub server_ip: String,
    pub server_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatusResponse {
    pub online: bool,
    pub players: u16,
    pub max_players: u16,
    pub motd: String,
    pub game_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LauncherVersionResponse {
    pub latest_version: String,
    pub hash: String,
    pub release_notes: String,
}
