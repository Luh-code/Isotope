use crate::window::main_window::Message;
use std::ffi::OsStr;
use std::any::Any;
use libloading::{Library, Symbol};
use std::collections::HashMap;


bitflags::bitflags! {
	#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
	pub struct ModuleFlags: u32 {
		const SURFACE = 0x01;
		const INSTANCED = 0x02;
	}	
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestPosition {
	Early,
	Init,
	Checkup,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestResult {
	Ok,
	Warning,
	Error,
}

/// Trait for a module
pub trait Module: Any + Send + Sync {
	/// Get a name for the module
	fn name(&self) -> &'static str;
	/// A callback fired directly after the the module loaded.
	/// Usually used for initialization
	fn on_plugin_load(&self) {}
	/// A callback fired immediately before the module is unloaded.
	/// Usually used for cleanup
	fn on_plugin_unload(&self) {}
	/// A function to check module operability
	fn test(&self, postion: TestPosition) -> Result<TestResult, Option<String>>;
	/// Gets the module flags for this module 
	fn get_flags(&self) -> ModuleFlags;
	/// Gets the component, that holds UI data for displaying a surface.
	/// Only gets used when the flag ModuleFlags::SURFACE is defined
	fn get_components(&self) ->	iced::Element<Message>;
	/// Gets an array of the names of dependencies
	fn get_module_dependencies(&self) -> &'static Vec<String>;
}

#[macro_export]
macro_rules! declare_module {
    ($module_type:ty, $constructor:path) => {
        #[no_mangle]
        pub extern "C" fn _module_create() -> *mut dyn crate::modules::Module {
            // make sure the constructor is the correct type.
            let constructor: fn() -> $module_type = $constructor;

            let object = constructor();
            let boxed: Box<dyn crate::modules::Module> = Box::new(object);
            Box::into_raw(boxed)
        }
    };
}

#[macro_export]
macro_rules! set_module_name {
	($module_name:tt) => {
		#[no_mangle]
		pub extern "C" fn _get_module_name() -> &'static str
		{
			$module_name
		}
	}
}

type ModuleStack = Vec<Box<dyn Module>>;

type ModuleCreate = unsafe fn() -> *mut dyn Module;

pub struct ModuleManager<'a> {
	// Constructors for the Modules
	constructors: HashMap<String, Box<Symbol<'a, ModuleCreate>>>,
	// Libs
	loaded_libraries: HashMap<String, Library>,
	// Module stacks with aliases and hashes
	module_stacks: HashMap<String, ModuleStack>,
}

impl<'a> ModuleManager<'a> {
	pub fn new() -> ModuleManager<'a> {
    ModuleManager {
      constructors: HashMap::new(),
      loaded_libraries: HashMap::new(),
      module_stacks: HashMap::new(),
    }
  }

	pub unsafe fn load_module<P: AsRef<OsStr>>(&mut self, filename: P) -> Result<(), &'static str> {
		let lib = Library::new(filename.as_ref()).unwrap();

		type NameFunction = unsafe fn() -> &'static str;

		let name_symbol: Symbol<NameFunction> = lib.get(b"_get_module_name").unwrap();

		let name: &str = name_symbol();

		self.loaded_libraries.insert(name.to_string(), lib);

		Ok(())
	}
}

fn test() {
	let mm = ModuleManager::new();
}