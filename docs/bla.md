# Kingdom Building
- Hexagonal World Map
- Every Tile produces Resources
- Buildings can be placed onto Tiles to increase resource production or for other effects 
- Resources can be used to unlock new Skills, Equipment and Recruit new Fighters
- Player chooses one Tile to attack each round, which will initiate combat.
  - When victorious, tile will be conquered for the kingdom
- Likewise, owned Tiles will regularly be attacked by Monsters and other adversaries. 
  - Players can deploy units to defend their lands, or initiate a fight themselves.

-> Start from a small settlement to a globe-spanning empire!
- Tiles owned

## Tiles and Tile Effects
- Grasslands: Food Production Bonus
- Woodlands: Timber Production Bonus
- Mountains: Metal Production Bonus
- River: Food Production Bonus, might allow special building interactions
- Ocean: Cannot be settled, natural boundary

## Resources
- Quarters: Directly limit the army size. Every Unit occupies a spot, and when this is maxed out, no more units can be recruited
- Food: Needed to keep your population happy
- Money: Used to hire Mercenaries

## Buildings
Buildings can be built onto Tiles to acquire extra effects, such as resource refinement or extra production.
- Smith: Builds Weapons & Armor

# World Map
Tiles will show a preview of what they can expect on a tile: Monsters, Bandits, An Army...
Players can assembled squads of their units and have to pick one of them to engage in Combat.

# Combat
Battles take place on a hexagonal battlefield. Turn-Based, but one turn for each individual unit rather than for each player.

New combatants can either be trained (costing resources) or hired randomly after each encounter (costing money). Maybe the player can also attempt to tame wild monsters instead of defeating them.

## Turn Actions
- Move: How many tiles a unit can move depends on the units class. A Unit can move, interact, and move again.

One Interaction (additional interactions might be available to certain classes)
- Attack: A default Attack
- Class Skill: Use of a class skill or spell.
- Item: Use of an Item.

## Turn Order
Unit Turn order is decided by the speed value: `[Some constant Value] + Speed` - this allows faster units to act more often.

Let's say we have two units, A and B, with 50 and 75 Speed respectively. 
50 - Unit A
75 - Unit B
100 - Unit A
150 - Unit A & Unit B
200 - Unit A
225 - Unit B

In case of a Tie, the unit with the higher speed value goes first.
In case both units have the same speed:
- If it's the units of two different players:
  - Alternate who's unit goes first: Player A, then Player B. Then Player A again. 
- If both units belong to the same player:
  - Create a popup to let the player decide / go alphabetically / roll a die for each.

# Units
## Stats
Let's use well-established D&D Style stats. Different classes will have their damage numbers be based on different attributes, so these here are just rough ideas.
- Strength: How strong physical attacks are
- Dexterity: Unit Speed, Evasion Chance (Or just general damage reduction to avoid RNG)
- Constitution: Health pool and defense against physical attacks
- Intelligence: Damaging Magic, Magical Defense
- Wisdom: Healing Magic, Magical Defense

### Sub-Stats
- Health: 10 * Constitution
- Mana: Intelligence * X + Wisdom * X
- Speed: 50 + X * Dex
- Happiness: Flexible -15% to +15% stat boost depending on a variety of factors: Food, Housing, recent events...

(Mana is kinda necessary, to limit how many super-strong spells can be used in quick succession, especially with multiclassing?)

## Classes
Every Class has a signature weapon assigned to them. Character classes can be changed at will, but stats are persistent and raise depending on the active class on level up (?)

- Berserker: Axes. Strong focus on single target melee damage.
- Knight: Swords. Well balanced defense & damage.
- Fencer: Rapiers. Increased damage after getting hit.
- Shieldbearer: Shields. Support.

#### "Medium Armor", for a lack of a better term
- Rogue: Daggers. Fast moving, high damage glass cannons.
- Monk: Bracers. Physical based magic, mix of STR and WIS.
- Ranger: Bows. Can apply debuffs or boost damage by aiming at certain body parts.

#### Mages
- White Mage: Staves. Support. Heal focused.
- Black Mage: Wands. Magical Damage.
- Blood Mage: Claws. Instead of Mana, their spells use HP as a resource. Vampire-inspired.

### Skills
Skills are unlocked by purchasing them for resources.
For now, random wacky ideas go here:
- Sacrifical Dagger: Deal damage this round. Target heals twice the amount at the end of next turn.

### Reactions
Every Unit can have one Skill set as their Reaction. This will be triggered automatically whenever the condition is met, but only once. It recharges at the end of their turn.

- Arrow Catcher: Dodge all incoming arrows.
- Retaliate: When attacked with a physical attack, retaliate with an attack of your own, if in range.
- Counterspell: When attacked with a spell, retaliate with a spell, if in range. (This can come in multiple variations for different spells, or the players can select which spell should be used.)
- Opportunity Attack: Whenever an enemy unit leaves your attack range, attack it.
- Quicken: When receiving damage, cast haste on yourself until the end of your next turn.
- Well Prepared: First hit in a fight receives damage bonus.
- Faster than Light: First direct hit in a fight will be evaded.

### Multiclassing
Every Unit can have the skills of one other class, but without access to their equipment. This means a black mage could use white magic, or a warrior could use a lancer's skills. Thus, we don't necessarily need hybrid classes like traditional red mages, players can just build those themselves, but still having them might be more interesting.

## Horoscopes
A random set of Units and/or Skills will receive Buffs and Debuffs every match. These effects will last for the entirety of the encounter.

# Multiplayer
If we separate Game Logic into a separate crate and have the client just be a visual representation of game data, multiplayer would be very easy to add, both Co-Op and PvP. As would adding extra AIs be.

# Basic Modding Support
By parsing all game data from external files at game launch, certain aspects like classes, skills, monsters and events will be easy to change or extend.
