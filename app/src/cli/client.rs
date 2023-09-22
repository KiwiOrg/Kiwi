use std::path::PathBuf;

use ambient_audio::AudioStream;
use ambient_core::window::ExitStatus;
use ambient_native_std::asset_cache::AssetCache;
use ambient_network::native::client::ResolvedAddr;

use crate::client;

use super::RunCli;

pub fn handle(
    run: &RunCli,
    rt: &tokio::runtime::Runtime,
    assets: AssetCache,
    server_addr: ResolvedAddr,
    golden_image_output_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    let audio_stream = if !run.mute_audio {
        match AudioStream::new() {
            Ok(v) => Some(v),
            Err(err) => {
                log::error!("Failed to initialize audio stream: {err}");
                None
            }
        }
    } else {
        None
    };

    let mixer = if run.mute_audio {
        None
    } else {
        audio_stream.as_ref().map(|v| v.mixer().clone())
    };

    // If we have run parameters, start a client and join a server
    let exit_status = client::run(rt, assets, server_addr, run, golden_image_output_dir, mixer);

    if exit_status == ExitStatus::FAILURE {
        anyhow::bail!("`client::run` failed with {exit_status:?}");
    }

    Ok(())
}