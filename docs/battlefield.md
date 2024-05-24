# The Battlefield

The battlefield consists of hexagonal tiles with varying properties. Skills and units may interact with tiles in order
to trigger specific effects. The size limit for Battlefields will depend on average unit movement as well as performance
metrics. Aside from built-in battlefields, additional battlefields may be created by users through the map editor.

## Tiles

Tiles themselves are mostly for visuals, but serve as the foundation for further objects. They also have an individual
height, which allows them to serve as natural walls. Tiles cannot be destroyed and their base height cannot be altered
through skills. Only one unit may be in place on a certain tile.

- Earth: No further effect.
- Grass: No further effect.
- Scorched Earth: Grass and Earth may turn into scorched earth.
- Sand: No further effect.
- Stone: No further Effect.

## Fluids

Fluids can be placed on top of tiles. Depending on their depth, they may stop or slow movement. Some skills might be
stronger or weaker when used while the user or target stands on top of a fluid. A fluid might also stick to a unit after
leaving it. Only one fluid may be in present on a tile at once. Additionally, fluids are static and cannot spread.

#### Water

- Increase Lightning damage
- Decrease Fire Damage

#### Lava

- Deals high amounts of damage when a unit comes into contact or walks through.

#### Slime

- Slows down movement significantly
- Sticks to a unit, slowing them down for a turn after leaving the fluid.

## Void

Tiles with height 0 are hidden and count as void. Only flying units can move over them, and no units can end their
movement on them. Void may hold fluids, which would then count as a "deep" fluid that's just as impassable.

## Props

Props are objects which can be placed on top of tiles. Multiple props can be placed on one tile, and a prop can be big
enough to cover multiple tiles, and certain skills might spawn new props on the map. They may be destructible or
interact with skills in other ways. As such, they are highly modular. For the sake of simplicity, skills will always
target the entire tile, and, as such, will interact with all props at once.

#### Possible Components:

- Destructible: The prop has HP and resistances to certain damage types. Getting destroyed by different damage types
  might trigger different effects and spawn another prop to visualize the remains.
    - Example: Flowers burning down when struck by fire, grass getting cut through slashing damage.
- Blocking: The tile beneath this prop cannot be walked onto.
    - Example: A Barricade.
- Extra Height: The tile beneath this prop can be walked onto, but the prop increases the tile's height.
    - Multiple props can be stacked on top of one another that way. Additional props which don't increase the height
      will be placed on top of the uppermost prop.
    - If these props are destroyed, anything placed on top will fall down, potentially receiving falling damage.
    - Example: a table.
- Interactable: A unit standing next to this prop might use it to trigger a certain action.
    - Example: A closed door or a lever.
- 
