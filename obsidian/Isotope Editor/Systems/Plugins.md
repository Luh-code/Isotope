## Architecture
Plugins are created in rust and exported as a shared library. This library will later be loaded at runtime. The basic structure of a Plugin is dictated by the `Plugin` trait, which follows the following spec.
### Spec
- `test(position: TestPosition) -> (i32, Option<String>)` - tests the plugin for correct operation | =0, Any - Good | >0, Any - Warning | <0, Any - Error
	- `position` will dictate where the test is run (`Early`, `Init`, `Checkup`)
		- `Early` will run when the plugin is first connected
		- `Init` will run when the plugin is activated
		- `Checkup` can be run at any time after activation to check stability
	- If test() results in Good, operation continues (info will be printed if `Verbose` is defined)
	- If test() results in Error, operation exits (info will always be printed as long as `Quiet` is not defined)
	- If test() results in Warning, operation continues, if `Pedantic` is defined, operation will exit (info will always be printed as long as `Quiet` is not defined)
- `get_messages() -> `
- `view() -> Element<Message>` - gets the iced element with the content of the plugin pane