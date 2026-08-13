// SPDX-License-Identifier: Apache-2.0
// Copyright (C) 2026 Alexander Galay <alexander.galay@proton.me>

#![allow(unused)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_code: String,
    pub message: String,
}
