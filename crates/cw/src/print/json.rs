use std::collections::HashMap;

use libcw::Stats;
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
    pub fn new(stats: Stats) -> Self {
        Self {
            total: stats,
            ..Default::default()
        }
    }
}

#[async_trait::async_trait]
impl Printer for JsonPrinter {
    async fn print(&mut self, message: Message) -> std::io::Result<()> {
        let (path, result) = message;
        match &result {
            Ok(x) => self.total += *x,
            Err(_) => {}
        }

        let either = result.map_err(|x| x.to_string()).into();
        self.summary.insert(path, either);
        Ok(())
    }
    async fn close(&mut self) -> std::io::Result<()>{
        /*
        As stated by Serde:
            Serialization can fail if T's implementation of Serialize decides 
            to fail, or if T contains a map with non-string keys.
        
        It will be fine :)
        */
        let json = unsafe { serde_json::to_vec(self).unwrap_unchecked() };
        let mut stdout = stdout();
        let _ignore = stdout.write(json.as_slice()).await?;
        stdout.flush().await
    }
}
