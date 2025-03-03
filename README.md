# globe

[![Continuos Integration](https://github.com/hectormrc/globe/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/hectormrc/globe/actions/workflows/ci.yml)
[![Code Coverage](https://codecov.io/github/hectormrc/globe/coverage.svg?branch=main&token=)](https://codecov.io/gh/hectormrc/globe)
[![Crates.io: globe-rs](https://img.shields.io/crates/v/globe-rs.svg)](https://crates.io/crates/globe-rs)

A bridge between geographic and cartesian coordinates.

## About
This library provides a simple way to manipulate geographic coordinates while maintaining consistent values. Two coordinates systems are available: the [geographic coordinate system](https://en.wikipedia.org/wiki/Geographic_coordinate_system), which is made up of latitude, longitude, and altitude. And the [Cartesian coordinate system](https://en.wikipedia.org/wiki/Cartesian_coordinate_system), which is the regular one for representing arbitrary points in a three-dimensional space. Both of them can be converted from one to the other without restrictions but assuming a precision error.
