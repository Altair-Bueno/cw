use std::collections::HashMap;

use crate::stats::Stats;
use serde::Serialize;
use tokio::io::{stdout, AsyncWriteExt};

use super::{Message, Printer};

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

#[derive(Serialize, Debug, Default)]
pub struct JsonPrinter {
    total: Stats,
    summary: HashMap<String, Either<Stats, String>>,
}
impl JsonPrinter {
    pub fn new() -> Self {
        Default::default()
    }
}

#[async_trait::async_trait]
impl Printer for JsonPrinter {
    async fn begin(&mut self) {}
    async fn print(&mut self, message: Message) {
        let (path, result) = message;
        match &result {
            Ok(x) => self.total = self.total.clone() + x.clone(),
            Err(_) => {},
        }

        let either = result.map_err(|x| x.to_string()).into();
        self.summary.insert(path, either);
    }
    async fn terminate(&mut self) {
        let json = serde_json::to_vec(self).unwrap();
        let mut stdout = stdout();
        let _ignore = stdout.write(json.as_slice()).await;
        let _ignore = stdout.flush().await;
    }
}
