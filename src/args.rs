use crate::bug_report;
use anyhow::{anyhow, Result};
use asyncgit::sync::RepoPath;
use clap::ArgAction;
use clap::{
	crate_authors, crate_description, crate_name, crate_version, Arg,
	Command as ClapApp,
};
use simplelog::{Config, LevelFilter, WriteLogger};
use std::{
	env,
	fs::{self, File},
	path::PathBuf,
};

pub struct CliArgs {
	pub secret: String,
	pub tag: String,
	pub t: String,
	pub theme: PathBuf,
	pub repo_path: RepoPath,
	pub notify_watcher: bool,
}

pub fn process_cmdline() -> Result<CliArgs> {
	let app = app();

	let arg_matches = app.get_matches();

	if arg_matches.get_flag("bugreport") {
		bug_report::generate_bugreport();
		std::process::exit(0);
	}
	if arg_matches.get_flag("logging") {
		setup_logging()?;
	}

	let workdir =
		arg_matches.get_one::<String>("workdir").map(PathBuf::from);
	let gitdir = arg_matches
		.get_one::<String>("directory")
		.map_or_else(|| PathBuf::from("."), PathBuf::from);

	#[allow(clippy::option_if_let_else)]
	let repo_path = if let Some(w) = workdir {
		RepoPath::Workdir { gitdir, workdir: w }
	} else {
		RepoPath::Path(gitdir)
	};

	let arg_theme = arg_matches
		.get_one::<String>("theme")
		.map_or_else(|| PathBuf::from("theme.ron"), PathBuf::from);

	let theme = if get_app_config_path()?.join(&arg_theme).is_file() {
		get_app_config_path()?.join(arg_theme)
	} else {
		get_app_config_path()?.join("theme.ron")
	};

	let notify_watcher: bool =
		*arg_matches.get_one("watcher").unwrap_or(&false);

	let secret = arg_matches
		.get_one::<String>("secret")
		.map_or_else(|| String::from("secret"), String::from);

	let tag = arg_matches
		.get_one::<String>("tag")
		.map_or_else(|| String::from("tag"), String::from);

	let t = arg_matches
		.get_one::<String>("t")
		.map_or_else(|| String::from("t"), String::from);

	Ok(CliArgs {
		secret,
		tag, //TODO Vec<String>
		t,   //TODO Vec<String>
		theme,
		repo_path,
		notify_watcher,
	})
}

fn app() -> ClapApp {
	ClapApp::new(crate_name!())
		.author(crate_authors!())
		.version(crate_version!())
		.about(crate_description!())
		.help_template(
			"\
{before-help}gnostr-tui {version}
{author}
{about}

{usage-heading} {usage}

{all-args}{after-help}
		",
		)
		.arg(
			Arg::new("secret")
				.help("a sha256 hash") //TODO nsec...
				.long("sec")
				.value_name("SECRET")
				.default_missing_value("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
				.num_args(1)
				.global(true)
		)
		.subcommand(clap::Command::new("test"))
		.subcommand(clap::Command::new("do-stuff"))
		.arg(
			Arg::new("tag")
				.help("--tag <string> <string>")
				.long("tag")
				//.alias("-tag")
				.value_names(["TAG", "TAG_VALUE"])
				.default_missing_value("gnostr")
				.num_args(2..)
				.default_value("tag_default")
				.value_delimiter(' ')
				.action(ArgAction::Append)
				//.multiple(true),
		)
		.arg(
			Arg::new("t")
				.help("-t <string>")
				.long("t")
				.short('t')
				//.alias("-t")
				.value_name("T")
				.num_args(1..)
				.default_value("t_default")
				.default_missing_value("gnostr")
				.value_delimiter(' ')
				.action(ArgAction::Append)
				//.multiple(true),
		)
		.arg(
			Arg::new("theme")
				.help("Set the color theme (defaults to theme.ron)")
				.long("theme")
				.value_name("THEME")
				.num_args(1),
		)
		.arg(
			Arg::new("logging")
				.help("Stores logging output into a cache directory")
				.short('l')
				.long("logging")
				.num_args(0),
		)
		.arg(
			Arg::new("watcher")
				.help("Use notify-based file system watcher instead of tick-based update. This is more performant, but can cause issues on some platforms. See https://github.com/extrawurst/gitui/blob/master/FAQ.md#watcher for details.")
				.long("watcher")
				.action(clap::ArgAction::SetTrue),
		)
		.arg(
			Arg::new("bugreport")
				.help("Generate a bug report")
				.long("bugreport")
				.action(clap::ArgAction::SetTrue),
		)
		.arg(
			Arg::new("directory")
				.help("Set the git directory")
				.short('d')
				.long("directory")
				.env("GIT_DIR")
				.num_args(1),
		)
		.arg(
			Arg::new("workdir")
				.help("Set the working directory")
				.short('w')
				.long("workdir")
				.env("GIT_WORK_TREE")
				.num_args(1),
		)
}

fn setup_logging() -> Result<()> {
	let mut path = get_app_cache_path()?;
	path.push("gnostr-tui.log");

	println!("Logging enabled. log written to: {path:?}");

	WriteLogger::init(
		LevelFilter::Trace,
		Config::default(),
		File::create(path)?,
	)?;

	Ok(())
}

fn get_app_cache_path() -> Result<PathBuf> {
	let mut path = dirs::cache_dir()
		.ok_or_else(|| anyhow!("failed to find os cache dir."))?;

	path.push("gnostr-tui");
	fs::create_dir_all(&path)?;
	Ok(path)
}

pub fn get_app_config_path() -> Result<PathBuf> {
	let mut path = if cfg!(target_os = "macos") {
		dirs::home_dir().map(|h| h.join(".config"))
	} else {
		dirs::config_dir()
	}
	.ok_or_else(|| anyhow!("failed to find os config dir."))?;

	path.push("gnostr-tui");
	fs::create_dir_all(&path)?;
	Ok(path)
}

#[test]
fn verify_app() {
	app().debug_assert();
}
