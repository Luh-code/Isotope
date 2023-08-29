## Message
A message is internally used by ICED to associate events to actions. An example would be a button press, could send out a message to close a panel.
### Internal
A message has to be registered to the API via calling `register_message<message>()`. This will in turn later allow to call `set_action<message>(fnType)`. Actions are functions, that do not accept any arguments.