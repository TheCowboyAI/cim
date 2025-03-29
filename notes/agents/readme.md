# Agents
Agents refer to a automaton that is operated by a user.

We do not allow automotons to run unchecked, they ALWAYS have what you might think of as a supervising human.

This means that a person will be responsible for what the automoton does and will need access to the stream of Events the automoton provides.

Most of this is simply done for you by NATS, but we do need to establish how we communicate and not get overwhelmed with notifications from 100 agents.

Agents are mostly in the form of WASM Modules.
These WASM Modules communicate through NATS, and have the notion of being an "actor".

