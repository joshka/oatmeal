use anyhow::Result;
use once_cell::sync::OnceCell;
use tokio::sync::mpsc;

static SENDER: OnceCell<mpsc::UnboundedSender<String>> = OnceCell::new();

pub struct ClipboardService {}

impl ClipboardService {
    pub async fn start() -> Result<()> {
        let (tx, mut rx) = mpsc::unbounded_channel::<String>();
        SENDER.set(tx).unwrap();
        let mut clipboard = arboard::Clipboard::new()?;

        loop {
            let event = rx.recv().await;
            if event.is_none() {
                continue;
            }

            clipboard.set_text(event.unwrap())?;
        }
    }

    pub fn set(text: String) -> Result<()> {
        SENDER.get().unwrap().send(text)?;
        return Ok(());
    }
}
