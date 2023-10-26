
use crate::gen::internal::file_util::FileUtil;
use crate::gen::internal::server::generator::EntityGenerator;

use async_trait::async_trait;

use crate::core::result::Result;
use crate::gen::internal::server::ctx::Ctx;

pub(crate) struct PythonEntityGenerator {}

impl PythonEntityGenerator {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl EntityGenerator for PythonEntityGenerator {
    async fn generate_entity_files(&self, _ctx: &Ctx, _generator: &FileUtil) -> Result<()> {
        Ok(())
    }
}
