# hexagon-tactics

There really ain't much to see here right now, as I'm just toying around with bevy.
Attempting some kind of Final Fantasy Tactics Clone on a hexagonal map, with some roguelite elements sprinkled on top in
case I ever get that far.

## Project Structure

### game-client

The window which opens when a player launches the game, using bevy. This is where the ECS lives.

### game-common

General game logic, shared between server and client.

### game-server

Handle connections between Clients and AI, both local or via network.

# License

[GNU AGPL 3](./LICENSE)

In case anything here seems useful enough to upstream back into bevy or any of the other crates this project uses which
are using different licenses (Apache/MIT), feel free to poke me.
