## Architecture
Modules are created in rust and exported as a shared library. This library will later be loaded at runtime. The basic structure of a module is dictated by the `Module` trait, which follows the following spec.
### Spec
- `test(position: TestPosition) -> (i32, Option<String>)` - tests the module for correct operation | =0, Any - Good | >0, Any - Warning | <0, Any - Error
	- `position` will dictate where the test is run (`Early`, `Init`, `Checkup`)
		- `Early` will run when the module is first connected
		- `Init` will run when the module is activated
		- `Checkup` can be run at any time after activation to check stability
	- If test() results in Good, operation continues (info will be printed if `Verbose` is defined)
	- If test() results in Error, operation exits (info will always be printed as long as `Quiet` is not defined)
	- If test() results in Warning, operation continues, if `Pedantic` is defined, operation will exit (info will always be printed as long as `Quiet` is not defined)
- `get_flags() -> ModuleFlags` - returns all flags defined for the module
	- If the `surface` flag is defined, Isotope will allow for the [[Panel Manager]] to create a new panel using the element from `view()`
- `view() -> Element<Message>` - gets the ICED element, to be displayed (only when the `surface` flag is defined, see `get_flags()` above)