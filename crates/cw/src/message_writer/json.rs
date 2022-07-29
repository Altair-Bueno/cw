use std::collections::HashMap;

use async_trait::async_trait;
use serde::Serialize;
use tokio::io::{stdout, AsyncWriteExt};

use libcw::Stats;

use crate::message_writer::{Message, MessageWriter};

#[derive(Serialize, Debug)]
pub struct JsonMessageWriter {
    total: Stats,
    errors: i32,
    summary: HashMap<String, Either<Stats, String>>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
enum Either<L, R>
where
    L: Serialize,
    R: Serialize,
{
    Left(L),
    Right(R),
}

impl<L, R> From<Result<L, R>> for Either<L, R>
where
    L: Serialize,
    R: Serialize,
{
    fn from(input: Result<L, R>) -> Self {
        match input {
            Ok(x) => Either::Left(x),
            Err(x) => Either::Right(x),
        }
    }
}

impl JsonMessageWriter {
    pub fn init() -> Self {
        let total = Stats::default();
        let summary = HashMap::new();
        let errors = 0;
        Self {
            total,
            summary,
            errors,
        }
    }
}

#[async_trait]
impl MessageWriter for JsonMessageWriter {
    async fn message_received(&mut self, message: Message) {
        let (path, result) = message;
        match &result {
            Ok(x) => self.total = self.total.clone().combine(x.clone()),
            Err(_) => self.errors += 1,
        }

        let either = result.map_err(|x| x.to_string()).into();
        self.summary.insert(path, either);
    }

    async fn terminate(&mut self) -> i32 {
        let json = serde_json::to_vec(self).unwrap();
        let mut stdout = stdout();
        let _ignore = stdout.write(json.as_slice()).await;
        let _ignore = stdout.flush().await;
        self.errors
    }
}
