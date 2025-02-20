use std::{collections::BTreeMap, fs};

use anyhow::Result;
use clap::Parser;

use crate::tools::{GitRunner, NpmRunner};

use camino::Utf8PathBuf;

#[derive(Debug, Parser)]
pub struct Docs {
    #[clap(long, short, default_value = "./dev-docs")]
    path: Utf8PathBuf,

    // The monodocs branch to check out
    #[clap(long, short, default_value = "main")]
    pub(crate) branch: String,

    // The monodocs org to clone
    #[clap(long, short, default_value = "apollographql")]
    pub(crate) org: String,
}

impl Docs {
    pub fn run(&self, verbose: bool) -> Result<()> {
        let git_runner = GitRunner::new(verbose, &self.path)?;
        let docs = git_runner.clone_docs(&self.org, &self.branch)?;
        let local_sources_yaml_path = docs.join("sources").join("local.yml");
        let local_sources_yaml = fs::read_to_string(&local_sources_yaml_path)?;
        let mut local_sources: BTreeMap<String, Utf8PathBuf> =
            serde_yaml::from_str(&local_sources_yaml)?;
        local_sources.insert(
            "rover".to_string(),
            crate::utils::PKG_PROJECT_ROOT.join("docs").join("source"),
        );
        fs::write(
            &local_sources_yaml_path,
            serde_yaml::to_string(&local_sources)?,
        )?;
        let npm_runner = NpmRunner::new(true)?;
        npm_runner.dev_docs(&self.path)?;
        Ok(())
    }
}
