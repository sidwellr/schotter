# Schotter Change Log

#### 21 May 2021
Initial version

#### 1 June 2021
Editorial updates to improve descriptions and make the code and descriptions match each other.

#### 22 June 2021
Update to Nannou version 0.17. The random number generator gen_range function changed from using two arguments for the range to using a range. For example, `rng.gen_range(-0.5, 0.5)` changed to `rng.gen_range(-0.5..0.5)`.

#### 21 November 2021
Update to Nannou version 0.18. The `nannou::ui` module was refactored to a `nannou_conrod` crate. This required changing the code for setting up the control panel (though the actual widget code is the same).

I also updated the Rust edition to "2021" just to get the latest. No code changes were required for this.
