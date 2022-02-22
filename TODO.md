<!--
 Copyright (c) 2022 Tony Barbitta
 
 This Source Code Form is subject to the terms of the Mozilla Public
 License, v. 2.0. If a copy of the MPL was not distributed with this
 file, You can obtain one at http://mozilla.org/MPL/2.0/.
-->

So far here is the listing of what is done and what needs doing.

- [ ] :heavy_exclamation_mark::heavy_exclamation_mark::heavy_exclamation_mark: **Organize this list by section / category.** :heavy_exclamation_mark::heavy_exclamation_mark::heavy_exclamation_mark:
- [x] [ComputeFunction], [ComputeRequest], [ComputeResponse], i.e. basic plugin structure
- [x] [ComputeFunctionManager] has been written but not run or tested at all
  - [ ] Make sure it actually works.
  - [ ] See what can be done better.
- [ ] Write all the tests.
- [ ] Figure out how to test the more involved components of the project, like the route handlers, the plugin manager, and the places that they intersect.
- [ ] Start building the CLI. It should be simple but I want to make sure that it is easy to send and receive in the format that the app expects. **I don't want all of my weird enums and wrappers to interfere with usage.**
- [ ] Ensure code actually runs / works. I've finally gotten the errors to disappear but as we all know with interior mutability, that just means it will probably fail at runtime instead of yelling at me at compile-time.  
- [ ] Expand the [ComputeRequest] and [ComputeResponse] types, as they are currently mostly just a placeholder.
- [ ] Split up any files that contain too much crap. I've moved the [error](./src/core/types/error/mod.rs) file into it's own sub-module, but it's all still sitting in mod.rs instead of separate files as it should be. I'd rather have lots of files than huge files.
  - [x] core/types/error
- [ ] Expand [TargetComputeFunc] to better handle namespaced invocations and possibly path parameters or queries.
- [ ] Start writing some basic functions. As always, I think you get the best idea of how a library works by implementing / using it. So as I start to build some basic (and hopefully not-too-basic) [ComputeFunction]s I'll get a better idea of what needs to be changed.
- [ ] Hand-in-hand with previous entry, start assembling a library of various utility functions that will come in handy with implementing [ComputeFunction]s. Apparently compiler version has to be strictly synced between a rust binary and dynamic libraries, so I don't know if this is going to effect external dependency usage in implementation libraries. Either way I'm sure I'll come across a handful of necessary functionalities while writing some built-in functions.
- [ ] Read more about `tokio::Mutex` vs `std::sync::Mutex` vs `std::sync::RwLock`, figure out which one should be used. `RwLock` *seems* like a better choice since it has writer-prioritization built in (or maybe I have that backwards?) and that seems to be the kind of priority that should be applied to this situation. Once this project is built out a bit more, I'm imagining the typical use-case will be a single server handling a bunch of requests, so it seems very easy for a writer to be stalled for long periods of time waiting for a write-lock. On the other hand, once things are more stable it seems like plugin additions and removals will become pretty rare. Ideally I'd like to have a workflow that looks like this: You write your web-app or whatever that uses some cloud function/lambda functionality, so in its start-up process it sends a request to the already running local-compute instance to make sure whatever plugins it needs are already loaded or will be loaded. In this case, forcing a writer to wait will make the start-up time for that web-app much longer than necessary (then again what are the odds you'll have a second web-app project you're working on already running? It's not like there's a ton of context switching in the web-dev world right?)
- [ ] Try out other server implementations. Find a way to compare performance maybe?
  - [x] Axum
    - [ ] Currently I have functions written using both [tokio::Mutex] & [std::sync::RwLock], it's going to be stupid to do this for all of the other frameworks and in future stuff, make a decision
  - [ ] Hyper (raw)
  - [x] Warp
  - [ ] Gotham?

Server Notes: Having written a (very bare-bones) implementation for [warp] and [axum], I think I like [warp]s style better. It's composable by nature so many small functions can be combined into larger endpoints, and for me at least it's a little easier to wrap my head around.

[ComputeFunction]: ./src/core/types/func.rs
[ComputeFunctionManager]: ./src/core/manager/cfm.rs
[ComputeRequest]: ./src/core/types/req.rs
[ComputeResponse]: ./src/core/types/resp.rs
[TargetComputeFunc]: ./src/core/types/req.rs
