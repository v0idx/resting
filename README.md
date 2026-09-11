# resting

## A simple command line utility to make http requests

### Description

resting aims to be an all in one command line utility to make http requests, and then view the responses. On top of being functional, I also hope to further my knowledge of Rust and it's crates through this project.

#### `dev` Branch Usage

Upon making some updates (3 years after initially starting this project) I became aware of a limitation within the reqwests crate that the project previously relied upon, namely being unable to make requests to explicit IP addresses (at least that I could see!). To fix this, this branch is aiming to refactor and redevelop the codebase to rely upon the `rustls` crate instead, lower level in nature, this should allow further customisability within the utility.

#### TODO

- [ ] Create connection-specific functionality
- [ ] Redesign cli implementation with `clap`
- [ ] Implement HEAD and GET methods to reach feature parity with current `master` branch.
