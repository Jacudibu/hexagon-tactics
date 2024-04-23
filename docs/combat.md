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

Unit Turn order is decided by the speed value, something along the lines of `[Some constant Value] + Speed`, maybe
getting active once a certain total threshold is reached, kind of
like [Charge Time](https://finalfantasy.fandom.com/wiki/Charge_Time#Final_Fantasy_Tactics).
The main goal should be to allow faster units to act more often, without making the speed stat alone too overpowered.
Proper values have to be figured out through gameplay testing, obviously.

To handle ties, each unit gets an initial tiebreaker value assigned to them, depending on their speed. The unit with the
higher tiebreaker goes first, but then these values are swapped. That swap should only occur when there are enough
units, otherwise they'd get two turns in a row. 
