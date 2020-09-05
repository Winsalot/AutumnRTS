# Things to implement before starting to actively look for contributors

I am well aware that the game I want to make is (*almost*) impossible to do alone. Fortunately, I also **do not want** to make this game alone. I started this project and I made it open source because I want to build a community of developers and players around it.

And someone has told me once that "Open source game will be alive for as long as there are developers interested in fiddling with it". Therefore I see that looking for outside help will be a priority in (hopefully near) future.

Unfortunately, at this moment the project is not yet ready for contributions. Main problem is that first I want to lay the foundations first, and only then let others build on top of them.

Therefore, these are the things that should be implemented before I begin to actively look for contributors:

- Core game logic that implements:
	- Units shooting at each other and killing each other.
	- Move Order support.
	- Use ability order support.
	- Attack order support.
	- Some simple spatial partitioning implementation.
- Code structure/architecture that I feel relatively confident about (ambigious, I know).
- Proper foundations for Godot side of things:
	- Move to 3d (map, models, animations)
	- Actual interpolation of animations.
	- Implement basic GUI.
	- Make proper project layout.
	- Implement unit orders with mouseclicks.
	- Implement deselect.
	- Revise rustbridge for the most recent version of gdnative.
- Some nice to have features:
	- Host a playable web version of game.
	- (Maybe?) Document code.
	- Prepare github for external contributors.
	- Create a list of issues that people could work on.
	- Fix all warnings in code.