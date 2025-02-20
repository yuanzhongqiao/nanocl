use std::sync::Arc;

use nanocl_error::io::IoResult;

use nanocld_client::NanocldClient;

use crate::{
  cli::Cli,
  models::{Store, SystemState, SystemStateRef, EventEmitter},
};

use super::{event, metric};

pub async fn init(cli: &Cli) -> IoResult<SystemStateRef> {
  #[allow(unused)]
  let mut client = NanocldClient::connect_with_unix_default();
  #[cfg(any(feature = "dev", feature = "test"))]
  {
    use nanocld_client::ConnectOpts;
    client = NanocldClient::connect_to(&ConnectOpts {
      url: "http://nanocl.internal:8585".into(),
      ..Default::default()
    })?;
  }
  let event_emitter = EventEmitter::new(&client);
  let state = Arc::new(SystemState {
    client,
    event_emitter,
    store: Store::new(&cli.state_dir),
    nginx_dir: cli.nginx_dir.clone(),
  });
  event::spawn(&state);
  metric::spawn(&state);
  Ok(state)
}
