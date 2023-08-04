# Isotope-editor
Isotope is a Text editor written in Rust.
I'm building this, because I am very particular about the editors I use, and switch constantly, so to fix that, this project exists.
Isotope will support plugins like any other big Editor you might think of like VSCode, Atom/Pulsar, Vim/NeoVim, etc.
I'm trying to make something like 4Coder, with the extendability of NeoVim and ease of use that VSCode/Sublime Text provides.

Besides NeoVim being a big inspiration, the project will not be console-based, but run natively via Rust's Iced. 
This will maybe also enable it to be made into a web app in the (far) future! (But no promises)
Anyways, the reason Isotope is *not* running in the console is, that I want it to support some more complex graphics, that would be difficult to realize in a console.
For example: I am a big fan of Obsidian, and find the Node Graph Obsidian provides to be really useful in understanding connections between topics, and I always wanted to try such a node graph for code, which this will enable me to do!

## little disclaimer
I am a hobbyist and this project is one of my first times using Rust, so please don't go to hard on me when it comes to code quality :D
