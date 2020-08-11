# AutumnRTS

Real time strategy game/engine written in Rust and Godot. Project is in very early stage of development and it is not playable yet. 

When will it be playable? Nobody knows.

## Why another Open Source RTS project?

Short answer: Because I want to play RTS game in spirit of [SC:BW](https://en.wikipedia.org/wiki/StarCraft:_Brood_War)/[SC:R](https://en.wikipedia.org/wiki/StarCraft:_Remastered) on linux.

Long answer: Yes, there are already multiple successful Open Source RTS projects such as OpenRA and ZeroK. However, neither of those games scratch my itch for a RTS that is similar to SC:BW gameplay wise. 

Since I am already interested in Rust and Gamedev in general, I decided to start my own project with goals of:

- Writing my own RTS engine in Rust.
- Make sure that this engine is lightweight (unlikely because I use `.clone()` too liberally).
- Making a game for my own tastes using that engine.
- Have fun.

Overall I plan to aim to make a game that:

- Has competitive multiplayer experience.
- Focuses on balance between strategy, micromanagement and efficient macromanagement (economy).
- Does not have hard-counters in terms of strategies.

Regarding lore, setting and graphics, this is too early to say. Probably some sci fi thing.

## Goals for this engine and what will set it apart

Currently the game logic (simulation) is written in Rust, and rendering as well as user input is done in Godot (renderer). Currently renderer and simulation are already decoupled and run in separate threads. I believe that this architecture gives me a lot of freedom on future development.

For example, the game state could be rendered in either 2d or 3d, with only changes done in Godot, leaving Rust code completely unchanged. 

In theory, someday even bots and mods could be written purely in Godot.

## Goals for this game and what will set it apart

A lot of "modern" (made like after 2005) RTS games are trying to be too smart. They automate micromanagement, simulate projectiles, give units AI that dodges bullets and retreats when damaged, run in full 3d terrain, have destructible/modifiable terrain and move focus from player's mechanical skill towards decision making and strategizing. 

This is all wrong.

What makes RTS games extremely challenging is not mechanical skill, but the fact that it requires 100% of your attention for whole match (as there is no downtime during match). This is because RTS is about multitasking: scouting, expanding, upgrading, moving armies, thinking about what armies to build, reacting to what ypur enemy does, building sim city, paying constant attention to resources. Paying attention and doing all those things right requires insane multitasking, and player has to keep attention on those things for the whole duration of the match. 

And all those innovative RTS games get filled with stupid QoL features because "in oUR gME you dOn'T NEED 300 aPM to wIn, yoU neEd to StratEgIze". What in fact happens, is that when battle starts player is left helplessly watching his army win or lose with almost no opportunity to actively participate, because the outcome of battle is already decided by all the decisions players made before it started. You built wrong units? Too bad, now all your army is wiped out you dumbass.

On the contrary, in games where mechanical skill matters the player actually **gets to meaningfully participate in battle.** Your enemy has army that soft counters your army? Well, if you manage to surround and focus his units better, then this battle might be an even trade.

The point is that games where mechanical skill matters also require same level of strategy and multitasking. The difference is that player gets to directly participate in a battle which is what makes those games fun.

Therefore with this game I want to get back to the good old SC1 formula of:

- Babysitting your units.
- Dumb unit AI.
- Game state being set in 2 dimensions (even though it might be rendered in 3D).
- Static terrain.
- Heavy micromanagement. 
- "Winning because 300+ APM".
- Multiple units with multiple active abilities.
- Asymmetric factions. 
- Good UI (you would be surprised how many other open source RTS games have horrible UI)

## Why name it "AutumnRTS"?

Simply because there already exists [SpringRTS](https://en.wikipedia.org/wiki/Spring_Engine). Since I am also making a RTS engine I wanted to continue the theme of seasons of the year. However, my engine has very different design goals from Spring engine in terms of game features, so I chose the opposite season to Spring: Autumn.

## Current state of project

Development is rather slow. You can check when the last commit happened to see if it is still going.  

Also at the moment the game is made of colored rectangles and circles, which looks kinda ugly.

## Installation

To test the code locally navigate to project folder and build rust library:

	cd Simulation
	cargo build --release

After that you can open the project in Godot and run it.
