#spec
## Overview
Tabula Rasa (TR) is a spec and module group for editors.
TR aims to achieve a versatile editor and plugin API for ultimate customizability whilst maintaining ease of use.
### Dependencies
*None*
## Design
TR is mainly designed for text editors but supports visual editors if you become a little crafty.
### Text
Text in TR is split up into blocks to achieve easy inline annotation. There is not a minimum amount of blocks required in TR, but when no annotations are active, there most commonly is just one block.
When a [plugin](Plugin) decides it wants to add an annotation to a line, it can cut out a block and add an annotation to that block. A [plugin](Plugin) does not have to clean up after itself, when a annotation isn't needed anymore, besides deleting the annotation. Anytime a change in annotation happens to a block, TR will check if the block is (still) annotated after the change, if this is the case, the block stays separated. If there are no annotations found in the block it will automatically be combined with the most viable option.
This is the combination algorithm:
1. Check above - if not annotated, append
2. Check below - if not annotated, prepend
3. Don't combine
4. If a combination happened, repeat until none happen
#### Multi-line annotation
Annotations can be added to a range of lines as well using multi-line annotation.
This works by grouping a specified range of lines into a block, which in itself can be split into blocks. This allows for you to have say a error message, to the right of the statement where it happened, extending over all lines, where this occurred if the statement was extending over multiple lines.
#### Full-Split mode
Full-Split mode works the same as normal, just that every line is it's own block.
#### Example
![[Tabula Rasa Example.excalidraw]]
#### Procedural Annotations
Procedural Annotations are used to add annotations to all lines in a procedural manner. An example use-case would be adding line numbers or a reference count or something like that.
### Status Bar
TR includes spec for a status bar. This bar can be anywhere, depending on the implementation.
Any plugin can add buttons or other indicators to this bar by using the builtin API.
These elements can be positioned left (will append to left side) or right (will append to right side). Text, progress bars, buttons, sub-menus and selectors are integrated as part of the status bar API. Other widgets need to be added manually and it is suggested, that they also follow the current theming to avoid visual clashing.