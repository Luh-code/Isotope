#[cfg(test)]
mod tests {
	use crate::window::keybinds::input_manager::{
	  KeyBinding,
	  KeyBindingManager,
	};
	use iced::keyboard::{KeyCode, Modifiers};
	use colored::Colorize;

	#[test]
	fn test_input0_true() {
		let mut mger = KeyBindingManager::new();
	  let bnd = KeyBinding {
	    key_code: KeyCode::C,
	    modifiers: Modifiers::CTRL,
	  };
	  mger.add_key_binding(bnd, String::from("test"), false);

	  let check: String = String::from("test");
	  let bnd: Option<&KeyBinding> = mger.get_key_binding(&check);
	  if bnd == None {
	    println!("{} '{}' {}", "No key aliased".red(), check.red(), "found".red());
	  	assert!(false);
	  }
	}

	#[test]
	fn test_input1_false() {
		let mut mger = KeyBindingManager::new();
	  let bnd = KeyBinding {
	    key_code: KeyCode::C,
	    modifiers: Modifiers::CTRL,
	  };
	  mger.add_key_binding(bnd, String::from("test"), false);

	  let check: String = String::from("test1");
	  let bnd: Option<&KeyBinding> = mger.get_key_binding(&check);
	  if bnd == None {
	    println!("{} '{}' {}", "No key aliased".red(), check.red(), "found".red());
	    return
	  }
	  assert!(false);
	}

	#[test]
	fn test_input2_true() {
		let mut mger = KeyBindingManager::new();
	  let bnd = KeyBinding {
	    key_code: KeyCode::C,
	    modifiers: Modifiers::CTRL,
	  };
	  mger.add_key_binding(bnd, String::from("test"), false);

	  let check: String = String::from("test");
	  let bnd: Option<&KeyBinding> = mger.get_key_binding(&check);
	  if bnd == None {
	    println!("{} '{}' {}", "No key aliased".red(), check.red(), "found".red());
	    assert!(false);
	  }

	  let bnd: &KeyBinding = bnd.unwrap();

	  if *bnd ==
	    (KeyBinding {
	      key_code: KeyCode::C,
	      modifiers: Modifiers::CTRL,
	    })
	  {
	    println!("{}", "True!!!".green());
	  } else {
	    println!("{}", "False...".yellow());
	  }
	  mger.remove_key_binding(&check);
	  let bnd: Option<&KeyBinding> = mger.get_key_binding(&check);
	  if bnd == None {
	    println!("{} '{}'!", "Successfully removed key binding".green(), check.green());
	    return
	  }
	  assert!(false);
	}

	#[test]
	fn test_input3_false() {
		let mut mger = KeyBindingManager::new();
	  let bnd = KeyBinding {
	    key_code: KeyCode::C,
	    modifiers: Modifiers::CTRL,
	  };
	  mger.add_key_binding(bnd, String::from("test"), false);

	  let check: String = String::from("test");
	  let bnd: Option<&KeyBinding> = mger.get_key_binding(&check);
	  if bnd == None {
	    println!("{} '{}' {}", "No key aliased".red(), check.red(), "found".red());
	    assert!(false);
	  }

	  let bnd: &KeyBinding = bnd.unwrap();

	  if *bnd ==
	    (KeyBinding {
	      key_code: KeyCode::D,
	      modifiers: Modifiers::CTRL,
	    })
	  {
	    println!("{}", "True!!!".green());
	  } else {
	    println!("{}", "False...".yellow());
	  }
	  mger.remove_key_binding(&check);
	  let bnd: Option<&KeyBinding> = mger.get_key_binding(&check);
	  if bnd == None {
	    println!("{} '{}'!", "Successfully removed key binding".green(), check.green());
	    return
	  }
	  assert!(false);
	}
}