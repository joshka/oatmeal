use anyhow::Result;
use async_trait::async_trait;

use crate::domain::models::AcceptType;
use crate::domain::models::Editor;
use crate::domain::models::EditorContext;
use crate::domain::services::clipboard::ClipboardService;

#[derive(Default)]
pub struct Clipboard {}

#[async_trait]
impl Editor for Clipboard {
    #[allow(clippy::implicit_return)]
    async fn health_check(&self) -> Result<()> {
        return Ok(());
    }

    #[allow(clippy::implicit_return)]
    async fn get_context(&self) -> Result<Option<EditorContext>> {
        return Ok(None);
    }

    #[allow(clippy::implicit_return)]
    async fn clear_context(&self) -> Result<()> {
        return Ok(());
    }

    #[allow(clippy::implicit_return)]
    async fn send_codeblock<'a>(
        &self,
        _context: EditorContext,
        codeblock: String,
        _accept_type: AcceptType,
    ) -> Result<()> {
        ClipboardService::set(codeblock)?;
        return Ok(());
    }
}
