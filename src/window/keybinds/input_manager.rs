use std::vec::Vec;
use iced::keyboard::{KeyCode, Modifiers};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use colored::Colorize;

// Struct to describe a KeyBinding
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct KeyBinding {
	pub key_code: KeyCode,
	pub modifiers: Modifiers,
}

// Implement hashing for KeyBinding
impl Hash for KeyBinding {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.key_code.hash(state);
		self.modifiers.hash(state);
	}
}

// Srtuct to manage key Bindings
#[derive(Debug)]
pub struct KeyBindingManager {
	pub aliases: Vec<String>,
	pub hashes: Vec<u64>,
	pub key_bindings: Vec<KeyBinding>,
}

impl KeyBindingManager {
	pub fn new() -> Self {
		KeyBindingManager{
			aliases: Vec::<String>::new(),
			hashes: Vec::<u64>::new(),
			key_bindings: Vec::<KeyBinding>::new(),
		}
	}

	// Adds a key binding
	pub fn add_key_binding(&mut self, binding: KeyBinding, alias: String, add_anyways: bool) -> i32 {
		let mut hasher = DefaultHasher::new();
		let hash = hasher.finish();
		binding.hash(&mut hasher);
		if self.aliases.iter().position(|x| *x == alias) != None {
			println!("{}", "Key binding already exists!".yellow());
			return -1
		}
		if !add_anyways && self.hashes.iter().position(|&x| x == hash) != None {
			println!("{}", "Key combination already in use!".red());
			return 1
		}
		self.aliases.push(alias);
		self.hashes.push(hash);
		self.key_bindings.push(binding);
		0
	}

	// Removes a key binding
	pub fn remove_key_binding(&mut self, alias: &String) -> bool {
		let temp = self.aliases.iter().position(|x| *x == *alias );
		if temp == None {
			return false
		}

		let index = temp.unwrap();
		self.aliases.remove(index);
		self.hashes.remove(index);
		self.key_bindings.remove(index);
		true
	}

	// Gets the key binding, that corresponds to the alias
	pub fn get_key_binding(&self, alias: &String) -> Option<&KeyBinding> {
		let temp = self.aliases.iter().position(|x| *x == *alias);
		if temp == None {
			return None
		}

		Some(&self.key_bindings[temp.unwrap()])
	}

	// Gets all aliases
	pub fn get_all_keys(&self) -> &Vec<String> {
		&self.aliases
	}
}