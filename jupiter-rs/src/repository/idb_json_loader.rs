use crate::ig::json::list_to_doc;
use crate::platform::Platform;
use crate::repository::loader::{Loader, LoaderInfo};
use anyhow::Context;
use serde_json::Value;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub struct IdbJsonLoader {
    platform: Arc<Platform>,
}

impl IdbJsonLoader {
    pub fn new(platform: Arc<Platform>) -> Self {
        IdbJsonLoader { platform }
    }
}

impl Display for IdbJsonLoader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "IDB-JSON")
    }
}

#[async_trait::async_trait]
impl Loader for IdbJsonLoader {
    async fn file_changed(&self, loader_info: &LoaderInfo) -> anyhow::Result<()> {
        let data = loader_info.get_data_str().await?;
        let rows: Value =
            serde_json::from_str(data.as_str()).context("Cannot parse the given JSON data.")?;
        let doc = list_to_doc(rows.as_array().context("The given JSON wasn't an array.")?)
            .context("Cannot transform JSON to doc")?;

        self.update_table(doc, loader_info).await
    }

    fn platform(&self) -> &Arc<Platform> {
        &self.platform
    }
}
