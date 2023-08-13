## Message
A message is a struct to convey something to the program. This can range from something as low-level as a key combination being pressed to something as high-level as the vertical split of a pane being requested.
### Internal
A message is a trait to be implemented for every message type and are suffixed with `_message`. Messages have to be registered to the API via calling `register_message<message>(alias: String)`. This will add the command to an internal map of string to string, where the strings are alias and internal name. This is required, so that later on, when the command is issued there is an internal name, which just is the name of the message struct plus a user-friendly name to make the API more accessible.

*Revised:*
A message has to be registered to the API via calling `register_message<message>()`. This will in turn later allow to call `message!(message)`, which will call the update function of Isotope with the given message.