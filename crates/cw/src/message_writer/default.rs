use async_trait::async_trait;
use colored::Colorize;
use tokio::io::{AsyncWriteExt, BufWriter, Stderr, stderr, Stdout, stdout};

use libcw::{Parser, Stats};

use crate::message_writer::{Message, MessageWriter};

const TOTAL: &str = "total";

pub struct DefaultMessageWriter {
    canary: u8,
    stdout: BufWriter<Stdout>,
    stderr: BufWriter<Stderr>,
    total: Stats,
    error_count: i32,
}

impl DefaultMessageWriter {
    pub async fn init(parser: Parser) -> Self {
        let mut stdout = BufWriter::new(stdout());
        let stderr = BufWriter::new(stderr());

        let parser_string_blue = parser.to_string().as_str().blue();
        let files_blue = "File(s)\n".blue();
        let s = format!("{parser_string_blue} {files_blue}");
        let _ = stdout.write(s.as_bytes()).await;

        let canary = 0x2;
        let total = Stats::default();
        let error_count = 0;

        Self { canary, stdout, stderr, total, error_count }
    }
}

#[async_trait]
impl MessageWriter for DefaultMessageWriter {
    async fn message_received(&mut self, message: Message) {
        let (path, result) = message;
        self.canary >>= 1;
        match result {
            Ok(x) => {
                let s = format!("{x}{path}\n");
                let _ = self.stdout.write(s.as_bytes()).await;
                self.total = self.total.clone().combine(x);
            }
            Err(err) => {
                let s = format!("{path}: {err}\n");
                let _ = self.stderr.write(s.as_bytes()).await;
                self.error_count += 1;
            }
        }
    }

    async fn terminate(&mut self) -> i32 {
        let DefaultMessageWriter { stdout, stderr, total, canary, .. } = self;

        let _ = stdout.flush().await;
        let _ = stderr.flush().await;

        if *canary == 0 {
            // Total files
            let merged_string_green = total.to_string().as_str().green();
            let total_string_green = TOTAL.green();
            let s = format!("{merged_string_green}{total_string_green}\n");
            let _ = stdout.write(s.as_bytes()).await;
            let _ = stdout.flush().await;
            let _ = stderr.flush().await;
        }
        self.error_count
    }
}