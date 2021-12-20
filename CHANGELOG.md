# Schotter Change Log

#### 20 December 2021
Added schotter3a and schotter4a, alternate implementations of schotter3 and schotter4 that use egui instead of Conrod to implement the control panel.

#### 16 December 2021
I just discovered that you can set the loop mode the the app builder. That seems more logical than setting it in view() or model(), so I changed the code to use that method.

#### 21 November 2021
Update to Nannou version 0.18. The `nannou::ui` module was refactored to a `nannou_conrod` crate. This required changing the code for setting up the control panel (though the actual widget code is the same).

I also updated the Rust edition to "2021" just to get the latest. No code changes were required for this.

#### 22 June 2021
Update to Nannou version 0.17. The random number generator gen_range function changed from using two arguments for the range to using a range. For example, `rng.gen_range(-0.5, 0.5)` changed to `rng.gen_range(-0.5..0.5)`.

#### 1 June 2021
Editorial updates to improve descriptions and make the code and descriptions match each other.

#### 21 May 2021
Initial version
