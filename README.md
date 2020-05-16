# Gossip-rs: Rusty Implementation of gossip protocols

## Implementation

### Event Loops:

There are a few event loops used within this library:
- Network Listeners
  - Reliable Transport Server
    - Implemented with TCP for now, in the future switch/augment with QUIC based approach
  - Unreliable Transport Server
    - Implemented with UDP, and should assume to not guarantee delivery
- Gossip Generators