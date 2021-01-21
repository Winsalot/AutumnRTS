# AutumnRTS

Real time strategy game/engine written in Rust and Godot. Project is in very early stage of development and it is not playable yet. 

![](/media/AutumnRTS_2021_jan.gif)

## FAQ

### Why another Open Source RTS project?

Short answer: Because I want to play RTS game in spirit of [SC:BW](https://en.wikipedia.org/wiki/StarCraft:_Brood_War)/[SC:R](https://en.wikipedia.org/wiki/StarCraft:_Remastered) on Linux.

Long answer: Yes, there are already multiple successful Open Source RTS projects such as OpenRA and ZeroK. However, neither of those games scratch my itch for a RTS that is similar to SC:BW gameplay wise. 

Since I am already interested in Rust and GameDev in general, I decided to start my own project with goals of:

- Writing my own RTS engine in Rust.
- Make sure that this engine is lightweight (unlikely because I use `.clone()` too liberally).
- Making a game for my own tastes using that engine.
- Have fun.

### Goals for this engine and what will set it apart

Currently the game logic (simulation) is written in Rust, and rendering as well as user input is done in Godot (renderer). As of now renderer and simulation are already decoupled and run in separate threads. I believe that this architecture gives me a lot of freedom on future development.

For example, the game state could be rendered in either 2d or 3d, with only changes done in Godot, leaving Rust code completely unchanged. 

In theory, someday even bots and mods could be written purely in Godot.

### Why name it "AutumnRTS"?

Simply because there already exists [SpringRTS](https://en.wikipedia.org/wiki/Spring_Engine). Since I am also making a RTS engine I wanted to continue the theme of seasons of the year. However, my engine has very different design goals from what games made with Spring Engine offer in terms of game features, so I chose the opposite season to Spring: Autumn.

### What is the current state of project?

Development is rather slow. You can check when the last commit happened to see if it is still going. 

If you want to see exactly what I am working on at the moment you can refer to a document called `.DevelopmentProgress.txt` (**WARNING**: I write in this document with assumption that no one besides me will read it, therefore if you decide to read it please expect to see lots of swearing, grammatical mistakes and incoherent rambling).

### How do I build and run this project?

To test the code locally navigate to project folder and build rust library:

	cd Simulation
	cargo build --release

After that you can open the project in Godot and run it.

### Where can I read about the plans for the game itself?

I write out most of my ideas and philosophy about the game in [YASI.md](https://github.com/Winsalot/AutumnRTS/blob/master/YASI.md)  document.

