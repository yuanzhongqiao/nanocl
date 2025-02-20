use tabled::Tabled;
use chrono::TimeZone;
use clap::{Parser, Subcommand};

use bollard_next::exec::CreateExecOptions;

use nanocld_client::stubs::{
  cargo::CargoSummary,
  cargo_spec::{CargoSpecUpdate, Config, CargoSpecPartial, HostConfig},
};

use super::{
  GenericInspectOpts, GenericListOpts, GenericRemoveForceOpts,
  GenericRemoveOpts, GenericStartOpts, GenericStopOpts,
};

/// `nanocl cargo create` available options
#[derive(Clone, Parser)]
pub struct CargoCreateOpts {
  /// Name of the cargo
  pub name: String,
  /// Image of the cargo
  pub image: String,
  /// Volumes of the cargo
  #[clap(short, long = "volume")]
  pub volumes: Option<Vec<String>>,
  /// Environment variables of the cargo
  #[clap(short, long = "env")]
  pub(crate) env: Option<Vec<String>>,
}

/// Convert CargoCreateOpts to CargoSpecPartial
impl From<CargoCreateOpts> for CargoSpecPartial {
  fn from(val: CargoCreateOpts) -> Self {
    Self {
      name: val.name,
      container: Config {
        image: Some(val.image),
        // network: val.network,
        // volumes: val.volumes,
        env: val.env,
        host_config: Some(HostConfig {
          binds: val.volumes,
          ..Default::default()
        }),
        ..Default::default()
      },
      ..Default::default()
    }
  }
}

/// `nanocl cargo run` available options
#[derive(Clone, Parser)]
pub struct CargoRunOpts {
  /// Name of the cargo
  pub name: String,
  /// Image of the cargo
  pub image: String,
  /// Volumes of the cargo
  #[clap(short, long = "volume")]
  pub volumes: Option<Vec<String>>,
  /// Environment variables of the cargo
  #[clap(short, long = "env")]
  pub env: Option<Vec<String>>,
  #[clap(long = "rm", default_value = "false")]
  pub auto_remove: bool,
  /// Command to execute
  pub command: Vec<String>,
}

/// Convert CargoRunOpts to CargoSpecPartial
impl From<CargoRunOpts> for CargoSpecPartial {
  fn from(val: CargoRunOpts) -> Self {
    Self {
      name: val.name,
      container: Config {
        image: Some(val.image),
        // network: val.network,
        // volumes: val.volumes,
        env: val.env,
        cmd: Some(val.command),
        host_config: Some(HostConfig {
          binds: val.volumes,
          auto_remove: Some(val.auto_remove),
          ..Default::default()
        }),
        ..Default::default()
      },
      ..Default::default()
    }
  }
}

/// `nanocl cargo start` available options
#[derive(Clone, Parser)]
pub struct CargoStartOpts {
  // Name of cargo to start
  pub name: String,
}

/// `nanocl cargo restart` available options
#[derive(Clone, Parser)]
pub struct CargoRestartOpts {
  // List of cargo to stop
  pub names: Vec<String>,
}

/// `nanocl cargo patch` available options
#[derive(Clone, Parser)]
pub struct CargoPatchOpts {
  /// Name of cargo to update
  pub(crate) name: String,
  /// New name of cargo
  #[clap(short = 'n', long = "name")]
  pub(crate) new_name: Option<String>,
  /// New image of cargo
  #[clap(short, long = "image")]
  pub(crate) image: Option<String>,
  /// New environment variables of cargo
  #[clap(short, long = "env")]
  pub(crate) env: Option<Vec<String>>,
  /// New volumes of cargo
  #[clap(short, long = "volume")]
  pub(crate) volumes: Option<Vec<String>>,
}

/// Convert CargoPatchOpts to CargoSpecUpdate
impl From<CargoPatchOpts> for CargoSpecUpdate {
  fn from(val: CargoPatchOpts) -> Self {
    CargoSpecUpdate {
      name: val.new_name,
      container: Some(Config {
        image: val.image,
        env: val.env,
        ..Default::default()
      }),
      ..Default::default()
    }
  }
}

/// `nanocl cargo exec` available options
#[derive(Clone, Parser)]
pub struct CargoExecOpts {
  /// Allocate a pseudo-TTY.
  #[clap(short = 't', long = "tty")]
  pub tty: bool,
  /// Name of cargo to execute command
  pub name: String,
  /// Command to execute
  #[clap(last = true, raw = true)]
  pub command: Vec<String>,
  /// Override the key sequence for detaching a container.
  #[clap(long)]
  pub detach_keys: Option<String>,
  /// Set environment variables
  #[clap(short)]
  pub env: Option<Vec<String>>,
  /// Give extended privileges to the command
  #[clap(long)]
  pub privileged: bool,
  /// Username or UID (format: "<name|uid>[:<group|gid>]")
  #[clap(short)]
  pub user: Option<String>,
  /// Working directory inside the container
  #[clap(short, long = "workdir")]
  pub working_dir: Option<String>,
}

/// Convert CargoExecOpts to CreateExecOptions
impl From<CargoExecOpts> for CreateExecOptions {
  fn from(val: CargoExecOpts) -> Self {
    CreateExecOptions {
      cmd: Some(val.command),
      tty: Some(val.tty),
      detach_keys: val.detach_keys,
      env: val.env,
      privileged: Some(val.privileged),
      user: val.user,
      working_dir: val.working_dir,
      attach_stderr: Some(true),
      attach_stdout: Some(true),
      ..Default::default()
    }
  }
}

/// `nanocl cargo history` available options
#[derive(Clone, Parser)]
pub struct CargoHistoryOpts {
  /// Name of cargo to browse history
  pub name: String,
}

/// `nanocl cargo revert` available options
#[derive(Clone, Parser)]
pub struct CargoRevertOpts {
  /// Name of cargo to revert
  pub name: String,
  /// Revert to a specific historic
  pub history_id: String,
}

/// `nanocl cargo logs` available options
#[derive(Clone, Parser)]
pub struct CargoLogsOpts {
  /// Name of cargo to show logs
  pub name: String,
  /// Only include logs since unix timestamp
  #[clap(short = 's')]
  pub since: Option<i64>,
  /// Only include logs until unix timestamp
  #[clap(short = 'u')]
  pub until: Option<i64>,
  /// If integer only return last n logs, if "all" returns all logs
  #[clap(short = 't')]
  pub tail: Option<String>,
  /// Bool, if set include timestamp to ever log line
  #[clap(long = "timestamps")]
  pub timestamps: bool,
  /// Bool, if set open the log as stream
  #[clap(short = 'f')]
  pub follow: bool,
}

/// `nanocl cargo stats` available options
#[derive(Clone, Parser)]
pub struct CargoStatsOpts {
  /// Names of cargo to show stats
  pub names: Vec<String>,
  /// Disable streaming stats and only pull the first result
  #[clap(long)]
  pub no_stream: bool,
  // TODO: Show all containers (default shows just running)
  // pub all: bool,
}

/// `nanocl cargo` available commands
#[derive(Clone, Subcommand)]
pub enum CargoCommand {
  /// List existing cargo
  #[clap(alias("ls"))]
  List(GenericListOpts),
  /// Create a new cargo
  Create(CargoCreateOpts),
  /// Start cargoes by names
  Start(GenericStartOpts),
  /// Stop cargoes by names
  Stop(GenericStopOpts),
  /// Restart a cargo by its name
  Restart(CargoRestartOpts),
  /// Remove cargo by its name
  #[clap(alias("rm"))]
  Remove(GenericRemoveOpts<GenericRemoveForceOpts>),
  /// Inspect a cargo by its name
  Inspect(GenericInspectOpts),
  /// Update a cargo by its name
  Patch(CargoPatchOpts),
  /// Execute a command inside a cargo
  Exec(CargoExecOpts),
  /// List cargo history
  History(CargoHistoryOpts),
  /// Revert cargo to a specific history
  Revert(CargoRevertOpts),
  /// Show logs
  Logs(CargoLogsOpts),
  /// Run a cargo
  Run(CargoRunOpts),
  /// Show stats of cargo
  Stats(CargoStatsOpts),
}

/// `nanocl cargo` available arguments
#[derive(Clone, Parser)]
pub struct CargoArg {
  /// namespace to target by default global is used
  #[clap(long, short)]
  pub namespace: Option<String>,
  #[clap(subcommand)]
  pub command: CargoCommand,
}

/// A row of the cargo table
#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct CargoRow {
  /// Name of the cargo
  pub(crate) name: String,
  /// Image of the cargo
  pub(crate) image: String,
  /// Status of the cargo
  pub(crate) status: String,
  /// Number of running instances
  pub(crate) instances: String,
  /// Spec version of the cargo
  pub(crate) version: String,
  /// When the cargo was created
  #[tabled(rename = "CREATED AT")]
  pub(crate) created_at: String,
  /// When the cargo was last updated
  #[tabled(rename = "UPDATED AT")]
  pub(crate) updated_at: String,
}

/// Convert CargoSummary to CargoRow
impl From<CargoSummary> for CargoRow {
  fn from(cargo: CargoSummary) -> Self {
    let binding = chrono::Local::now();
    let tz = binding.offset();
    // Convert the created_at and updated_at to the current timezone
    let created_at = tz
      .timestamp_opt(cargo.created_at.and_utc().timestamp(), 0)
      .unwrap()
      .format("%Y-%m-%d %H:%M:%S");
    let updated_at = tz
      .timestamp_opt(cargo.spec.created_at.and_utc().timestamp(), 0)
      .unwrap()
      .format("%Y-%m-%d %H:%M:%S");
    Self {
      name: cargo.spec.name,
      image: cargo.spec.container.image.unwrap_or_default(),
      version: cargo.spec.version,
      status: format!("{}/{}", cargo.status.actual, cargo.status.wanted),
      instances: format!("{}/{}", cargo.instance_running, cargo.instance_total),
      created_at: format!("{created_at}"),
      updated_at: format!("{updated_at}"),
    }
  }
}
