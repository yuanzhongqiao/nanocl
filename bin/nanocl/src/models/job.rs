use tabled::Tabled;
use chrono::TimeZone;
use clap::{Parser, Subcommand};

use nanocld_client::stubs::{job::JobSummary, process::WaitCondition};

use super::{
  GenericInspectOpts, GenericListOpts, GenericRemoveOpts, GenericStartOpts,
};

/// `nanocl job wait` available options
#[derive(Clone, Parser)]
pub struct JobWaitOpts {
  /// State to wait
  #[clap(short = 'c')]
  pub condition: Option<WaitCondition>,
  /// Name of job to wait
  pub name: String,
}

/// `nanocl job logs` available options
#[derive(Clone, Parser)]
pub struct JobLogsOpts {
  /// Name of job to show logs
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

/// `nanocl job` available commands
#[derive(Clone, Subcommand)]
pub enum JobCommand {
  /// List existing job
  #[clap(alias("ls"))]
  List(GenericListOpts),
  /// Remove job by its name
  #[clap(alias("rm"))]
  Remove(GenericRemoveOpts),
  /// Inspect a job by its name
  Inspect(GenericInspectOpts),
  /// Show logs of a job
  Logs(JobLogsOpts),
  /// Wait for a job to finish
  Wait(JobWaitOpts),
  /// Start a job
  Start(GenericStartOpts),
}

/// `nanocl job` available subcommands
#[derive(Clone, Parser)]
pub struct JobArg {
  #[clap(subcommand)]
  pub command: JobCommand,
}

/// A job row to display job information in a table
#[derive(Tabled)]
#[tabled(rename_all = "UPPERCASE")]
pub struct JobRow {
  /// Name of the job
  pub name: String,
  /// Status of the job
  pub status: String,
  /// Total number of instances
  pub total: usize,
  /// Number of running instances
  pub running: usize,
  /// Number of succeeded instances
  pub succeeded: usize,
  /// Number of failed instances
  pub failed: usize,
  /// When the job was created
  #[tabled(rename = "CREATED AT")]
  pub created_at: String,
  /// When the job was last updated
  #[tabled(rename = "UPDATED AT")]
  pub updated_at: String,
}

/// Convert [JobSummary](JobSummary) to [JobRow](JobRow)
impl From<JobSummary> for JobRow {
  fn from(job: JobSummary) -> Self {
    let binding = chrono::Local::now();
    let tz = binding.offset();
    // Convert the created_at and updated_at to the current timezone
    let created_at = tz
      .timestamp_opt(job.spec.created_at.and_utc().timestamp(), 0)
      .unwrap()
      .format("%Y-%m-%d %H:%M:%S");
    let updated_at = tz
      .timestamp_opt(job.spec.updated_at.and_utc().timestamp(), 0)
      .unwrap()
      .format("%Y-%m-%d %H:%M:%S");
    Self {
      name: job.spec.name,
      status: format!("{}/{}", job.spec.status.actual, job.spec.status.wanted),
      total: job.instance_total,
      running: job.instance_running,
      succeeded: job.instance_success,
      failed: job.instance_failed,
      created_at: format!("{created_at}"),
      updated_at: format!("{updated_at}"),
    }
  }
}
