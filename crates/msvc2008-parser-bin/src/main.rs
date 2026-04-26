#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(os_string_truncate)]

use std::path::Path;

use anyhow::Context;
use clap::Parser;

use msvc2008_parser_lib::{sln, vcproj};

#[derive(clap::Parser)]
pub struct Cli {
    #[arg(long, value_hint = clap::ValueHint::FilePath)]
    pub sln_path: std::path::PathBuf,

    /// Project to build.
    #[arg(long)]
    pub project_name: String,
}

fn main() -> anyhow::Result<()> {
    let Cli {
        sln_path,
        project_name,
    } = Cli::parse();
    let sln = std::fs::read_to_string(&sln_path)?;
    let sln = match sln::Sln::parse(&sln) {
        Ok((_leftovers, sln)) => sln,
        Err(error) => anyhow::bail!("{error}"),
    };

    let deps = sln
        .find_project_dependencies(&project_name)
        .context("Project is not found")?;

    println!("Found {} dependencies for '{}'", deps.len(), project_name);
    for dep in &deps {
        println!("> {}", dep.name);
    }
    println!();

    let mut project_path = sln_path
        .parent()
        .context("Sln path must have a parent")?
        .to_path_buf();
    let base_len = project_path.as_os_str().as_encoded_bytes().len();

    for dep in deps {
        project_path.as_mut_os_string().truncate(base_len);

        for component in dep.path.split(['\\', '/']) {
            project_path.push(component);
        }

        let vcproj = std::fs::read_to_string(&project_path)?;
        let mut vcproj = vcproj::VCProject::parse_xml(&vcproj)
            .with_context(|| format!("Failed parsing '{}' at '{}'", dep.name, dep.path))?;

        vcproj
            .configurations
            .retain(|config| config.name == "Release|Win32" || config.name == "Master Gold|Win32");
        vcproj.platforms = Default::default();
        vcproj.files = vcproj::Files::default();

        for cfg in vcproj.configurations {
            if let Some(cl) = &cfg.compiler_tool {
                println!("[{}] [{}]: {}", vcproj.name, cfg.name, cl.to_flags(&cfg));
            }
        }

        // println!("[{}]\n {}\n", vcproj.name, skip_nones(&vcproj));
    }

    Ok(())
}

fn skip_nones(object: impl std::fmt::Debug) -> String {
    format!("{object:#?}")
        .lines()
        .filter(|line| !line.contains("None"))
        .collect::<Vec<_>>()
        .join("\n")
}
