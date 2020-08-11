# Micro Tournament

I got dissilusioned by my ability to finish (or get even close to finish) of this project solo. Since I still feel passionate about making a proper open source RTS I decided to focus on making a smaller game and engine that supports it.

Basically idea is scale down my planned game into a project that is a subset of traditional RTS: **Micro Tournament**.

This is not an original name, and describes a minigame where each player controls a small force of units and they fight each other. Winner is usually the one who is most skilled in micromanaging their units and using their abilities.

Therefore as a subset of traditional RTS this type of game won't implement these features:

- Resource gathering.
- Army building & unit production.
- Tech tree.
- Multiplayer (maybe someday).
- Bots (enemy units can be spawned with existing order to attack move the player).
- Unit upgrades.
- Fog of war.
 
 Instead the game format could look something like this.
 
1.  Player enters game.
2.  From menu player gets to choose a starting army.
3.  Once player's army spawns he gets like ~30 seconds to prepare for a fight.
4.  Enemy army spawns. They mindlessly A-move player.
5.  Fight happens. 
6.  If player's units die game over.
7.  If player wins the engagement then he gets to keep the remaining units and chooses reinforcements from menu.
8.  Another fight in the same format.

So yeah, something like this.

### Roadmap of features needed to finish this game:



 - Simulation features:
	- Spawn units of different teams.
	- Use data files to store maps & unit descriptions.
	- Functional unit collision.
	- Implemented weapons.
	- Use efficient algorithms for spatial logic instead of grid search.
	- ✔️ Implemented active abilities.
	- Win/lose condition.
	- Interactive map features (teleport units that walk too far away).
	- Advanced unit & group control:
		- Group orders.
		- Pathfinding for groups.
		- Schedule orders (shift-orders).
		- A-move, M-move, Follow, stop, hold position commands.
- Renderer features:
	- Everything. Current godot implementation will need to be rewritten entirely. (I am not looking forward to it)
- Content features:
	- Lots of cool maps. 
	- Lots of badass units.
	- Create levels from 2 things above.

As you can see from above points, every feature listed (except content) is also relevant for for traditional RTS games. Therefore if I implement them I will have a much simpler but functional game that could be further scaled up to support full rts gameplay. 