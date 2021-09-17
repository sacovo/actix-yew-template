use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct MyTestStruct {
    v1: usize,
    v2: Option<String>,
}

impl MyTestStruct {
    pub fn new() -> Self {
        Self {
            v1: rand::thread_rng().gen_range(0..1000),
            v2: None,
        }
    }

    pub fn from(v1: i32) -> Self {
        Self {
            v1: v1 as usize,
            v2: None,
        }
    }
}
