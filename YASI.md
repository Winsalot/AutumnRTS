# YASI == Yet Another Stupid Idea

### What the game should have and what it shouldn't have:
	
It should be very similar to SC1/SCBW/SC:R gameplay wise.

On top of basic SC1 formula I believe the game should add:

- Better UI. Especially spellcasting, to allow efficient casting spells even if multiple units are selected. Also allow for efficient unit training when multiple buildings are selected at once.
- New unit designs. Create own, steal/copy form other games. As long as it ends up fun.

However, I think these SC1 features are should not be included:

- Unit selection cap. It is mostly inconvenience. Better give player who splits army into groups a soft advantage through the way unit AI behaves (eg. spliting army into separate control groups gives better control and more opportunities for surround compared to single control group for all units).
- Army size cap. I think there should be soft cap. My idea is to have every unit's cost scale with supply after soft cap has been reached.
- Perma-invis combat units (dark templar). Just anti fun mechanic. Invis should be in the game but either when stationary (eg. lurkers) or disabled for a period after agressive action (eg. riki from Dota2).
- Dark Swarm. It is just bad design to balance whole match up through one overpowered spell.
- Bugs/features that depend on how SC1 engine works. Such as muta stacking, hold position with overlords & lurkers. Reason is that AutumnRTS is its own engine with its own bugs/features and it's bad taste to try to replicate the bugs of other engines.

Why design after SC1 and and not SC2? Because in my opinion SC2 has 2 very big problems (at least when I played it):

- Deathball mechanics. Looks ugly and makes battles worse than in SC1. Multiple small battles across the map is much more cool than 2 balls of units running into each other. Custom game called starbow actually solved it by making unit AI spread out more instead of clumping up into massive ball. 
- Hard counters. In my experience SC2 unit counters were stronger than in SC1. While having strong counters adds weight to strategic decisions, having only soft counters gives more importance to tactics, micromanagement and economy (=army size).

 Some other RTS game features that I don't plan to add:
 
 - Real 3d terrain. In my experience it ends up making game feel very slow (cos armies slow down when going uphill)
 - Simulated projectiles. Unnecesary layer of complexity.
 - Terraforming/changing map layout. Cool, but unnecesary layer of complexity. Building intricate base layouts should be enough. 
- Hero units. 
- Vehicles crushing infantry. Cool concept, but also anti-fun.
 
 

### Lore

Please read BLAME! manga (but don't watch the netflix adaptation, it's shit). Once you finish reading it you will understand what I want the lore to look like.


Races/factions:

**Archaic humans:**

Using shitty 100 times broken and 100 times fixed machinery. They have strong units with friendly fire. 
	
**Transhumanists:**

More advanced than humans. Have variuous fancy enhancements on themselves. Small armies of overpowered and overspecialised units. Every unit starts as a same base type and then receives a series of upgrades to become strong.
	
**Deteriorating AI:**

Very advanced. Highest mobility, swarm units and powerful hard-hitters. Don't have traditional army production buildings, but can spawn units anywhere withing the range of their controll.


### Map features:

- Arbitrary number of z-levels
- Teleporters in map
- Neutral buildings (teleporters & destructible obstacles)

### Additional nice to have features:

- XOR upgrades (either one upgrade, or another, but not both)
- Soft cap on army size (aka. reduced resource extraction after hitting soft ceiling)
- Soft cap on resource extraction (eg. 90% less resources extracted in depleted areas, but never zero).
- 2 main resources. Choose one:
	- SC/WC style minerals & gas / gold & lumber. Both extracted similar way.
	- ZeroK/ OpenRA style: one traditional (minerals) and another produced by dedicated buildings (energy)
	- Mix of both (eg. geothermal extractors in ZeroK)?
- Make game playable and interesting early into developement by adding gamemodes:
	- Defense from waves of enemies (eg. ZeroK chickens)
	- Siege gamemode (attack defending and fortified bot player).
	- Figure out a way to attract AI students to build smart bots for this game (probably won't happen in early developement).
	- Single player campaign, with missions where player controls or has access to a limited number of units.
	- Epic multiplayer team vs team battles with big armies and lots of units (imagine ZeroK 7v7 games that last for ~1 hour or more)
	- Balance and framerate decoupling. This would allow game to be played at variable simulation framerate while still preserving game feel.
	- Even more daring: variable framerate during runtime. Eg. in situations where too many units get spawned framerate could be automatically reduced to account for more CPU work.

### Cool building/structure ideas

- Buildings that block FoW. So that enemies don't peek into your base.
- Neutral destructible bridges. Want to delay your enemy? Destroy the bridge. Would mean that in early game (while neithe rplayer has army big enough to efficiently destroy bridge) the map would be effectively smaller than after the bridge is destroyed and all the distances become greter.
- Flying buildings from like SC terran.
- Mech buildings (think elves from WC3).
- Players can build neutral buildings (they produce them, but buildings provide no vision).
- Buildings that can teleport themselves and units/structures around them. (This one is totally badass. If this allows structures to be teleported on top of each other, whis would result in impressive fireworks with structure with more hp&armour surviving).

### Totally awesome unit ideas

- SC2 Colossus type unit. No ground collisions, but can be targeted by both ground and anti air.
- Jetpack upgrade for infantry units. To allow moving through small cliffs.
- Infantry XOR upgrade for either movement speed or range.
- Units that transform from flying (no weapons) into ground (some weapons). I think SC2 has something like this.
- Unit that summons randomly placed non-expiring neutral buildings.
- SC2 broodlord type units (where attack spawns temporary unit) **must** happen. They are just amazing.
- Heavily armoured, light weapon unit that has active AOE damage ability that: is centered around caster, has friendly fire, multiple casts in short duration increase radius and damage.
- Artillery units with delay between fire and impact. They target ground so attacks can be dodged.
- Artillery units with homing projectile that can still miss.
- Melee units with MASSIVE (long and narrow) cone of cleave (think that sword from NOiSE and used by knights in end of BLAME!)
- Big Mechs with multiple weapons. Think Dante from ZeroK.
- Carriers from SC.
- Ling equivalent unit. Fast and numerous. 
- Temp unit spawning units. Like those turtles from SC2.
- Production buildings that irreversibly transform into powerful but expiring units.
- XXI century era tanks. Because ancient technology.

### Random awesome ideas:

- At the game end the units of winning player start dabbing and flossing.

### Ideas to "steal" from ZeroK

So in last few weeks I grinded quite some Zero-K and even participated in one 1v1 torunament. It was not my first time playing this game, but this time I took a deep dive and besides playing also watched competitive match commentaries and even analysed replays of my own games. I know that a lot of mechanics of ZK are taken from other games, but since I first encountered them in ZK, I will refer to them so.

Things I really liked:

- **Reclaim** mechanics. (Basically dead units can be reclaimed for up to 40% of their resource value). Apart from adding stategic component on the location of battles, this is also a comeback mechanic. Basically a failed push leaves a lot of resources in enemy base which makes their comeback much easier even if the push devastated a lot of their infrastructure. Overall I see this as an amazing** anti-snowball mechanic**. 
- **Flat tech tree**. Very beginner friendly and I personally don't really like complicated tech trees (I always forget some important building). I could see one faction in my game having a flat tech tree like that.
- **Deep tech tree.** Probably a wrong way to call this, but idea is that there are units that cost 25 metal and there are unis that cost 30k metal. While I personally don't really enjoy playing with and against mega units and structures, I am convinced that it is a **must-have** feature for casual team play. *A task for game design/balance would be to discourage the use of megaunits in 1v1 competitive, while making it a viable option in large team games.*
- **Massive team games (6-16 players per team)**.  This is very popular. Seems like another must-have to keep game community alive and satisfied. However, mid-late game in these matches get so laggy it becomes unbearable for me.
- **Clear unit role distinctions.** Probbaly necessary given the number of units in ZK. But makes the game very noob-friendly. Eg. if I see a unit for a first time I just hover over it to see that it's Riot (slow, short range, high dps) and I will know what to expect from it even if I have never seen this unit in a game before. However, I am not a fan of how these roles *hard-counter* each other in rock paper scissors manner: `Raid < Riot < Skirmsher < Raid`. 

Things I didn't like:

- **Most (99%) of units don't have stop-to-shoot.** This results in abundance of units that hard-counter each other. Basically if one unit has *speed and range* advantage over another then it becomes a **hard**-counter (because it can kite endlessly). Given the fact that ZK has over 100 units, this happens pretty often. And somewhere in all of these rants I have already mentioned that hard-counters suck.
- **Static defences are very strong**. I have counted that there exists 22 defence structures. Some of them can barely protect you from more than 3 raider units, others are a powerful offensive tools. Overall from games I played I get impression that it is very hard to push into fortified opponent. Upside is that this way players divide the map and for the whole duration there is some artillery action on the front lines. However, pushing into these fortifications is really hard and as a result I find that my 1v1 matches often take a really long time. (though this problem usually doesn't happen to pro players. However, is is generally true in any competitive game that high skill matches take shorter time than noob matches). I would much rather prefer the SC formula of 1-2 static defences per faction.
- **Unit AI**. In ZK you can enable unit AI which will make unit keep the optimal distance to his opponents (eg. rushing in if raider, keeping safe distance if artillery) as well as moving in zig-zags to avoid projectiles. This is hard to explain, but very often I found that I was greatly annoyed by this feature while still feeling that I would be putting myself in disadvantage if I didn't use it. Overall it has a heavy synergy with *no-stop-to-shoot *comabt mechanics, so if my game has *stop-to-shoot*, unit AI would be much less relevant.
- **Sea units & maps**. I like robots and I don't like boats.

Things I have a mixed feelings about:

- **3D simulation (terrain, movement, projectiles)**. I believe this has taken an impressive amout of effort from zk and SpringRTS devs. And it is definetely entertaining to take these things into consideration during a match. However, this can also be annoying (eg. when your phantom one-shots your another phantom because  it moves in front of it's slow projectile, or when hard to notice hill makes you lose a raider fight). Overall I believe that this is a great feature that makes SpringRTS games unique, while also believing that it is not necessary for a good game. 
- **Terraforming.** Very fun, not very effective. Gets quite a limited use in competitive matches, mostly used to lift the grounds for defence structures or siege units (crabs) or lower the ground to protect economy structures. Once again, cool feature, but not necessary. Also incompatible with 2D simulation in AutumnRTS.
- **Economy format.** Basically as long as you don't lose infrastructure your economy just grows. Metal resources never get depleted, and energy is created from "nothing". I have nothing against this in ZK context, but in my own game I would prefer to see resources get depleted from use.
