use std::ffi::OsStr;
use std::any::Any;
use libloading::{Library, Symbol};
use log::{trace, debug};

#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _plug_create() -> *mut dyn $crate::plugins::Plugin {
            let constructor: fn() -> $plugin_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn $crate::plugins::Plugin> = Box::new(object);

            println!("hello2");

            Box::into_raw(boxed)
        }
    };
}

pub trait Plugin: Any + Send + Sync{
    fn name(&self) -> &'static str;
    fn on_plugin_load(&self) {}
    fn on_plugin_unload(&self) {}
    fn get_package_version(&self) {}
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
    loaded_libraries: Vec<Library>,
}

impl PluginManager {
    pub fn new() -> PluginManager {
        PluginManager {
            plugins: Vec::new(),
            loaded_libraries: Vec::new(),
        }
    }

    pub fn load_plugin<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(), Box<dyn std::error::Error>> {
        type PluginCreate = unsafe fn() -> *mut dyn Plugin;

        let lib = Library::new(filename.as_ref()).unwrap();

        self.loaded_libraries.push(lib);

        let lib = self.loaded_libraries.last().unwrap();

        unsafe {
            let constructor: Symbol<PluginCreate> = lib.get(b"_plug_create").unwrap();
            debug!("Plugin {:?}", constructor);
            // let _boxed_raw = constructor();    
        }

        // let plugin = Box::from_raw(boxed_raw);
        // debug!("Loaded plugins: {}", plugin.name());
        // plugin.on_plugin_load();
        // self.plugins.push(plugin);

        Ok(())
    }

    pub fn get_package_version(&mut self) {
        debug!("Getting package version");

        for plugin in &mut self.plugins {
            trace!("Getting package version for {:?}", plugin.name());
            plugin.get_package_version();
        }
    }

    pub fn unload(&mut self) {
        debug!("Unloading plugins");

        for plugin in self.plugins.drain(..) {
            trace!("Running clean up for {:?}", plugin.name());
            plugin.on_plugin_unload();
        }

        for lib in self.loaded_libraries.drain(..) {
            drop(lib);
        }
    }
}

impl Drop for PluginManager {
    fn drop(&mut self) {
        if !self.plugins.is_empty() || !self.loaded_libraries.is_empty() {
            self.unload();
        }
    }
}