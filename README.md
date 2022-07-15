# Antenna Pattern Generator library

There are plenty of resources for learning about antennas and antenna
patterns. A couple of those links are below.
What is an [antenna](https://en.wikipedia.org/wiki/Antenna_(radio))?
What is an [antenna pattern](https://en.wikipedia.org/wiki/Radiation_pattern)?

And now that you know what an antenna pattern __IS__, you might be
asking yourself __WHY__ would you want to make one. There are a few
reasons below:

0. Learning: are you are interested in [phased arrays](https://en.wikipedia.org/wiki/Phased_array)?
0. Simulation: for projects that use antennas and want to simulate performance
0. Making pretty pictures: it might just be me but that looks pretty cool

(TODO: insert picture)

## How do I use it?

I have taken the liberty of creating some integration tests in the
[tests](tests/) directory. Each one shows how this library would be used
and provides a helper [function](tests/support/mod.rs) to write the
resulting pattern to an H5 file to the `tests/output/` directory which
is created when `cargo test` is executed.
