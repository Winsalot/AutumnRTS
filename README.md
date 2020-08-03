#AutumnRTS

Real time strategy game/engine written in Rust and Godot. Project is in very early stage of developement and it is not playable yet. 

When will it be ready? Nobody knows.

## Why another Open Source RTS project?

Yes, there are already multiple successful Open Source RTS projects such as OpenRA and ZeroK. However, neither of those games scratch my itch for a RTS that is similar to SC:BW gameplay wise. 

Since I am already interested in Rust and Gamedev in general, I decided to start my own project with goals of:

- Writing my own RTS engine in Rust.
- Make sure that this engine is lightweight.
- Making a game for my own tastes in that engine.
- Have fun.

Overall I plan to aim to make a game that:

- Has competitive multiplayer experience.
- Focuses on balance between strategy, micromanagement and efficient macromanagement (economy).
- Does not have hard-counters in terms of strategies.

Regarding lore, setting and graphics, this is too early to say. Probably some sci fi thing.

##Goals for this project and what will set it apart

Currently the game logic (simulation) is written in Rust, and rendering as well as user input is done in Godot (renderer). Currently renderer and simulation are already decoupled and run in separate threads. I believe that this architecture gives me a lot of freedom on future developement.

For example, the game state could be rendered in either 2d or 3d, with only changes done in Godot, leving Rust code completely unchanged. 

In theory, someday even bots and mods could be written purely in Godot.

##Current state of project

Developement is rather slow. You can check the last commit to see if it is still going.  

##Installation

To test the code locally navigate to project folder and build rust library:

	cd Simulation
	cargo build --release

After that you can open the project in Godot and run it.