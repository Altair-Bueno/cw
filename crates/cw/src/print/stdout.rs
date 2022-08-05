use async_trait::async_trait;
use colored::Colorize;
use libcw::Stats;
use tokio::io::{stderr, stdout, AsyncWriteExt, BufWriter, Stderr, Stdout};

use super::{Message, Printer};

#[derive(Debug)]
pub struct StdoutPrinter {
    canary: u8,
    stdout: BufWriter<Stdout>,
    stderr: BufWriter<Stderr>,
    total: Stats,
}

impl StdoutPrinter {
    pub fn new(stats: Stats) -> Self {
        let stdout = BufWriter::new(stdout());
        let stderr = BufWriter::new(stderr());

        let canary = 0x2;
        Self {
            canary,
            stdout,
            stderr,
            total: stats,
        }
    }
}

#[async_trait]
impl Printer for StdoutPrinter {
    async fn print(&mut self, message: Message) -> std::io::Result<()> {
        let (path, result) = message;
        self.canary >>= 1;
        match result {
            Ok(x) => {
                let path = path.to_string_lossy();
                let s = format!("{x}{path}\n");
                let _ignore = self.stdout.write(s.as_bytes()).await?;
                self.total += x;
            }
            Err(err) => {
                let path = path.to_string_lossy();
                let s = format!("{path}: {err}\n");
                let _ignore = self.stderr.write(s.as_bytes()).await?;
            }
        };
        Ok(())
    }
    async fn close(&mut self) -> std::io::Result<()> {
        let StdoutPrinter {
            stdout,
            stderr,
            total,
            canary,
            ..
        } = self;

        stdout.flush().await?;
        stderr.flush().await?;

        if *canary == 0 {
            // Total files
            let s = format!("{}\n", total.to_string().as_str().bold().green());
            let _ignore = stdout.write(s.as_bytes()).await?;
            stdout.flush().await?;
            stderr.flush().await?;
        }
        Ok(())
    }
}
