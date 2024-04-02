# hexagon-tactics

## Project Structure

### game-client

The window which opens when a player launches the game. Graphics and Audio, using bevy. This is where the ECS lives.

### game-common

General game logic, shared between server and client.

### game-server

Handle connections between Clients and AI, both local or via network.

# Licence

GNU AGPL 3
In case anything here seems useful enough to upstream back into bevy or any of the other crates this project uses which
are licenses under Apache or MIT, feel free to poke me.
