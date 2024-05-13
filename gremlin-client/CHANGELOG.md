# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.8.10](https://github.com/wolf4ood/gremlin-rs/compare/v0.8.9...v0.8.10) - 2024-05-13

### Other
- Expose connection pool's get timeout via ConnectionOptionsBuilder ([#212](https://github.com/wolf4ood/gremlin-rs/pull/212))

## [0.8.9](https://github.com/wolf4ood/gremlin-rs/compare/v0.8.8...v0.8.9) - 2024-04-24

### Other
- Map explicitly null response status message to empty string ([#210](https://github.com/wolf4ood/gremlin-rs/pull/210))

## [0.8.8](https://github.com/wolf4ood/gremlin-rs/compare/v0.8.7...v0.8.8) - 2024-04-19

### Other
- Default message to empty string for ResponseStatus ([#207](https://github.com/wolf4ood/gremlin-rs/pull/207))

## [0.8.7](https://github.com/wolf4ood/gremlin-rs/compare/v0.8.6...v0.8.7) - 2024-04-18

### Other
- Include GermlinError::Generic's contained string in thisserror's message ([#204](https://github.com/wolf4ood/gremlin-rs/pull/204))

## [0.8.6](https://github.com/wolf4ood/gremlin-rs/compare/gremlin-client-v0.8.5...gremlin-client-v0.8.6) - 2023-10-19

### Fixed
- fix from tungstenite error ([#198](https://github.com/wolf4ood/gremlin-rs/pull/198))
- fix compilation issue

### Other
- update tungstenite
- update webpki
- Update rustls requirement from 0.19 to 0.20 in /gremlin-client ([#144](https://github.com/wolf4ood/gremlin-rs/pull/144))
- Update base64 requirement from 0.13.1 to 0.21.4 in /gremlin-client ([#195](https://github.com/wolf4ood/gremlin-rs/pull/195))

### Added

### Fixed

### Changed


## [0.8.4] - 2023-05-20

### Added

- [#188](https://github.com/wolf4ood/gremlin-rs/pull/188)
- [#189](https://github.com/wolf4ood/gremlin-rs/pull/189)
- [#182](https://github.com/wolf4ood/gremlin-rs/pull/182)
- [#185](https://github.com/wolf4ood/gremlin-rs/pull/185) 

### Fixed

- Add Labels type in `add_e` [#174](https://github.com/wolf4ood/gremlin-rs/issues/174)

### Changed

- Removed `GraphSON` v1 [#177](https://github.com/wolf4ood/gremlin-rs/issues/177)

## [0.8.3] - 2023-02-06

### Added

### Fixed

- Remove Websocket & Time crates #172 

## [0.8.2] - 2021-05-09

### Added

### Fixed

- Fix connection not closing properly

## [0.8.0] - 2021-05-09

### Added

- [129](https://github.com/wolf4ood/gremlin-rs/pull/129) Added Option support for: String, i32, i64, f32, f64, uuid, date, and bool
- [132](https://github.com/wolf4ood/gremlin-rs/pull/131) Added SET and LIST cardinality support 

### Fixed

### Changed

- [#128](https://github.com/wolf4ood/gremlin-rs/issues/128) Fixed Date serialization precision

## [0.7.1] - 2021-03-03

### Added

- [#116](https://github.com/wolf4ood/gremlin-rs/pull/116) Added support for Session
### Fixed

## [0.7.0] - 2021-02-05

### Added

- [#122](https://github.com/wolf4ood/gremlin-rs/issues/122) Exposed AsyncTerminator
- Updated to Tokio v1
### Fixed

## [0.6.2] - 2020-11-16

### Added

- [#109](https://github.com/wolf4ood/gremlin-rs/pull/109) Added repeat, until, emit steps
- [#102](https://github.com/wolf4ood/gremlin-rs/pull/102) Added property many 

### Fixed

## [0.6.1] - 2020-09-7

### Added

### Fixed

- [#97](https://github.com/wolf4ood/gremlin-rs/issues/97) Fixed issue on boolean deserialization

## [0.6.0] - 2020-07-03

### Added

- [#50](https://github.com/wolf4ood/gremlin-rs/issues/50) First impl of derive from GResult/Map

### Fixed

- [#86](https://github.com/wolf4ood/gremlin-rs/issues/86) Fixed option accept_invalid_certs with async

## [0.5.1] - 2020-06-05

### Added

- [#82](https://github.com/wolf4ood/gremlin-rs/pull/82) Added .project(), .constant() & .barrier() and more.

### Fixed

## [0.5.0] - 2020-05-11

### Added

- [#77](https://github.com/wolf4ood/gremlin-rs/pull/77) Added Iter and IntoIter impl.

### Fixed

## [0.4.0] - 2020-04-18

### Added

- [#74](https://github.com/wolf4ood/gremlin-rs/pull/74) Added support for GraphSONv1
- [#75](https://github.com/wolf4ood/gremlin-rs/issues/75) Added support for Tokio Runtime

### Fixed

## [0.3.2] - 2020-03-22

### Added

- [#67](https://github.com/wolf4ood/gremlin-rs/issues/67) Implemented coalesce 
- [#66](https://github.com/wolf4ood/gremlin-rs/pull/66)  Added anonymous steps (add_v,property) and traversal steps (choose,value)

### Fixed

- [#69](https://github.com/wolf4ood/gremlin-rs/issues/69) Fixed issue with pong messages.

## [0.3.1] - 2020-02-10

### Added

- [#62](https://github.com/wolf4ood/gremlin-rs/issues/62) Added support for GraphSONv2


### Fixed

## [0.3.0] - 2020-01-06

### Added

- [#15](https://github.com/wolf4ood/gremlin-rs/issues/15) Async support
- [#51](https://github.com/wolf4ood/gremlin-rs/pull/51)  Repeat, until, simplePath, sample, loops and local
- [#47](https://github.com/wolf4ood/gremlin-rs/pull/47) Implements Pop enum for .select() and .v() 
- [#48](https://github.com/wolf4ood/gremlin-rs/pull/48) Implements basic with_side_effect
- [#55](https://github.com/wolf4ood/gremlin-rs/pull/55) Added out_e

### Fixed


## [0.2.2] - 2019-11-06

### Added

- [#41](https://github.com/wolf4ood/gremlin-rs/issues/8) Added traversal input for From/To step
- [#31](https://github.com/wolf4ood/gremlin-rs/issues/1) Implemented TextP

### Fixed

## [0.2.1] - 2019-09-13

### Added

- [#8](https://github.com/wolf4ood/gremlin-rs/issues/8) SSL Support
- [#1](https://github.com/wolf4ood/gremlin-rs/issues/1) Implemented SASL Authentication

### Fixed


## [0.2.0] - 2019-06-14

### Added
- [#12](https://github.com/wolf4ood/gremlin-rs/issues/12) GLV support (Base impl)
- [#16](https://github.com/wolf4ood/gremlin-rs/issues/16) Implemented addV Step
- [#17](https://github.com/wolf4ood/gremlin-rs/issues/17) Implemented property Step
- [#20](https://github.com/wolf4ood/gremlin-rs/issues/20) Implemented as Step
- [#18](https://github.com/wolf4ood/gremlin-rs/issues/18) AddEdge Step
- [#21](https://github.com/wolf4ood/gremlin-rs/issues/21) Implemented Remaining Vertex/Edge Step
- [#19](https://github.com/wolf4ood/gremlin-rs/issues/19) properties + propertyMap Step

### Fixed

- [#14](https://github.com/wolf4ood/gremlin-rs/issues/14) Fixed support for nested metrics

## [0.1.2] - 2019-04-04

### Added

- [#11](https://github.com/wolf4ood/gremlin-rs/issues/11) Support for V and E as keys in `Map`.
- [#2](https://github.com/wolf4ood/gremlin-rs/issues/10) Added support for other types as keys in `Map`.

### Changed

- [#13](https://github.com/wolf4ood/gremlin-rs/issues/13) Refactor List/Set in their own types.

## [0.1.1] - 2019-03-27

### Added

- [#2](https://github.com/wolf4ood/gremlin-rs/issues/2) Implemented alias support.

### Fixed

- [#4](https://github.com/wolf4ood/gremlin-rs/issues/4) Fixed traversal metrics eg. `g.V().profile()`
- [#3](https://github.com/wolf4ood/gremlin-rs/issues/3) Fixed traversal exxplanation eg. `g.V().explain()`

## [0.1.0] - 2019-03-18

### Added
- First release

