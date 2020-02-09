use log::info;

use scarper::declare_plugin;
use scarper::plugins::Plugin;

#[derive(Debug, Default)]
pub struct PlugGimp;

impl Plugin for PlugGimp {
    fn name(&self) -> &'static str {
        "PlugGimp"
    }

    fn on_plugin_load(&self) {
        info!("PlugGimp loaded");
    }

    fn on_plugin_unload(&self) {
        info!("PlugGimp unloaded");
    }

    fn get_package_version(&self) {
        info!("PlugGimp Get Version");
    }
}

declare_plugin!(PlugGimp, PlugGimp::default);