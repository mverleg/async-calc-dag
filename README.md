# Async calculation graph (DAG)

This repo is just for prototyping. The intention is to use a productionalized version of this in a compiler.

This intends to implement an efficient computational graph 'framework' with a few important properties:

* Leafs are external inputs (IO). They can become outdated.
* Any non-leaf graph nodes are calculations. They are "pure" in that they don't take external inputs or touch external state.
* However it's not fully pure, since results of steps can be cached. More than 1 old version per node can be cached.
* Invalidation can happen from leaf nodes when their external input changes, which will propagate through the graph.
* It is also possible to start from roots and work until reaching leafs, checking cache at each step.
* Calculations don't change; invalidation only ever originates at leafs (but propagates through nodes).
* Dependencies are not declared or known up-front. Any calculation can request any other calculation or leaf in the middle of calculating.
* The system automatically tracks dependencies that were used, and uses that for subsequent partial computations (even though they could have changed).
* Any errors that happen only reach the root if their result was actually needed. E.g. during partial computation.
* Steps run in parallel using multi-threaded Rust async (approximately thread-per-core with work stealing).
* There has to be some kind of cycle detection.


