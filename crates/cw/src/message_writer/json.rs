use std::collections::HashMap;

use async_trait::async_trait;
use serde::Serialize;
use tokio::io::{AsyncWriteExt, stdout};

use libcw::Stats;

use crate::message_writer::{Message, MessageWriter};

#[derive(Serialize)]
pub struct JsonMessageWriter {
    total: Stats,
    errors: i32,
    summary: HashMap<String, Stats>,
}

impl JsonMessageWriter {
    pub fn init() -> Self {
        let total = Stats::default();
        let summary = HashMap::new();
        let errors = 0;
        Self { total, summary, errors }
    }
}

#[async_trait]
impl MessageWriter for JsonMessageWriter {
    async fn message_received(&mut self, message: Message) {
        let (path, result) = message;
        match result {
            Ok(x) => {
                self.total = self.total.clone().combine(x.clone());
                self.summary.insert(path, x);
            }
            Err(_) => self.errors += 1,
        }
    }

    async fn terminate(&mut self) -> i32 {
        let json = serde_json::to_vec(self).unwrap();
        let mut stdout = stdout();
        let _ignore = stdout.write(json.as_slice()).await;
        let _ignore = stdout.flush().await;
        self.errors
    }
}