use clap::{ArgAction, Parser};
use eyre::Result;
use inquire::Confirm;
use ptree::TreeBuilder;

/// Command line arguments.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Verbosity level (0-4).
    #[arg(long, short, action = ArgAction::Count, default_value = "0")]
    v: u8,

    /// Dry run mode.
    /// If this flag is provided, the cli will not execute commands,
    /// printing the directories and files that would be created instead.
    #[arg(long)]
    dry_run: bool,

    /// Overwrite existing files.
    /// If this flag is provided, the cli will overwrite existing files.
    #[arg(long)]
    overwrite: bool,

    /// The project name.
    /// This will be used for the binary application name.
    #[arg(long, short, default_value = "example")]
    name: String,

    /// Add github actions ci workflow.
    #[arg(long, short)]
    with_ci: bool,

    /// Copy the specified ci workflow file to the project's `.github/workflows/` directory.
    #[arg(long, short)]
    ci_yml: Option<String>,

    /// The path to the project directory.
    /// By default, the current working directory is used.
    /// If any rust artifacts are detected in the specified
    /// or unspecified directory, an error will be thrown.
    #[arg(default_value = ".")]
    project_dir: String,
}

/// CLI Entrypoint.
pub fn run() -> Result<()> {
    let Args {
        v,
        dry_run,
        name,
        project_dir,
        overwrite,
        with_ci,
        ci_yml,
    } = Args::parse();

    crate::telemetry::init_tracing_subscriber(v)?;

    let mut builder = TreeBuilder::new(project_dir.clone());
    let project_dir_path = std::path::Path::new(&project_dir);
    if !dry_run {
        std::fs::create_dir_all(project_dir_path)?;
    }

    match overwrite {
        true => {
            tracing::warn!("Overwrite flag is set, existing files will be overwritten");
            if !Confirm::new("[WARNING] Overwrite mode will overwrite any conflicting files and directories. Are you sure you wish to proceed?").prompt()? {
                println!("Phew, close call... aborting");
                return Ok(());
            }
        }
        false => crate::utils::check_artifacts(project_dir_path, dry_run)?,
    }

    // we don't need to prompt the user twice if overwrite mode is enabled
    if !dry_run && !overwrite {
        tracing::warn!("Running in non-dry run mode.");
        tracing::warn!("Files and directories will be created.");
        tracing::warn!("This action may be destructive.");
        if !Confirm::new("Running amble in non-dry mode, are you sure you wish to proceed?")
            .prompt()?
        {
            println!("Phew, close call... aborting");
            return Ok(());
        }
    }

    crate::root::create(project_dir_path, &name, dry_run, Some(&mut builder))?;
    crate::bins::create(
        &project_dir_path.join("bin"),
        &name,
        dry_run,
        Some(&mut builder),
    )?;
    crate::libs::create(
        &project_dir_path.join("crates"),
        "common",
        dry_run,
        Some(&mut builder),
    )?;

    if with_ci || ci_yml.is_some() {
        crate::ci::create(project_dir_path, dry_run, ci_yml, Some(&mut builder))?;
    }

    if dry_run {
        let tree = builder.build();
        ptree::print_tree(&tree).expect("Error printing tree");
    }

    Ok(())
}
