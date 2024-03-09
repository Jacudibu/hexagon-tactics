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

## Resources
- Quarters: Directly limit the army size. Every Unit occupies a spot, and when this is maxed out, no more units can be recruited
- Food: Needed to keep your population happy
- Money: Used to hire Mercenaries

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
In case of a tie there:
- If it's the units of two different players:
  - Alternate who's unit goes first: Player A, then Player B. Then Player A again. 
- If both units belong to the same player::
  - Create a popup to let the player decide, go alphabetically or roll a die for each.


## Classes 

### Skills
Skills are unlocked by purchasing them for resources.
For now, random wacky ideas go here:
- Sacrifical Dagger: Deal damage this round. Target heals twice the amount at the end of next turn.

### Reactions
Every Unit can have one Skill set as their Reaction. This will be triggered automatically whenever the condition is met, but only once. It recharges at the end of their turn.

- Arrow Catcher: Dodge all incoming arrows.
- Retaliate: When attacked with a physical attack, retaliate with an attack of your own.
- Counterspell: When attacked with a spell, retaliate with a spell. (This can come in multiple variations for different spells.)
- Opportunity Attack: Whenever an enemy unit leaves your attack range, attack it.
- Quicken: When receiving damage, cast haste on yourself until the end of your next turn.

### Multiclassing
Every Unit can have the skills of one other class, but without access to their equipment.

## Horoscopes
A random set of Units and/or Skills will receive Buffs and Debuffs every match. These effects will last for the entirety of the encounter.
