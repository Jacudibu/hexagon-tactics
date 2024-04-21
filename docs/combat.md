# Combat

Battles take place on a hexagonal battlefield. Turn-Based, but one turn for each individual unit rather than for each
player.

New combatants can either be trained (costing resources) or hired randomly after each encounter (costing money). Maybe
the player can also attempt to tame wild monsters instead of defeating them.

## Turn Actions

- Move: How many tiles a unit can move depends on the units class. A Unit can move, interact, and move again.

One Interaction (additional interactions might be available to certain classes)

- Attack: A default Attack
- Class Skill: Use of a class skill or spell.
- Item: Use of an Item.

## Turn Order

Unit Turn order is decided by the speed value: `[Some constant Value] + Speed` - this allows faster units to act more
often.

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
