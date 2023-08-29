#spec
## Overview
Aesthetic is a spec for unifying Theming across Isotope.
Aesthetic manages themes. By simply selecting a different one and reloading the UI, anything using Aesthetic will be updated accordingly.
Aesthetic also simplifies creating UI, as there are helper functions for many ICED components, that return a pre-themed ICED component.
### Dependencies
*None*
## Adding Themes
Themes are [plugins](Modules.md) for Aesthetic, that follow the [theme spec](Theme) or CSS files. CSS themes cannot have any procedural UI like ASCII loading bars, that plugin themes allow.