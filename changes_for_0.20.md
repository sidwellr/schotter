# Changes Made for Nannou 0.20

The release of nannou 0.20.0 was a major change. Nannou now runs on the Bevy game engine, which will allow developers to focus more on the creative coding aspects of nannou. But this required quite a few changes to the API, which are reflected in the latest version of the schotter programs.

The main tutorial is meant for new users, so doesn't highlight the changes needed to migrate to nannou 0.20. For users who have used previous nannou versions, here is a list of the changes that were needed for the latest release as well as some quirks in nannou 0.20 that I had to work around.

* Pressing Esc no longer exits nannou programs. I added this functionality to all schotter versions that accept keypresses.

* The construct `loop_mode(LoopMode::loop_once())` in the sketch builder is no longer supported. The nearest replacement I could find is `set_update_mode`, but it isn't available in the sketch builder. I added `app.set_update_mode(UpdateMode::freeze());` to the beginning of the view function as a workaround, but it generates a new random frame when you move the mouse.

A fix has been added to the nannou code base which will be in the next released version. Put `.loop_once()` in the sketch builder for schotter1. For the other variations, `app.set_update_mode()` is needed, but this can't be in the app/sketch builder; it needs to be added to the `view` function.

* For all variations, two changes are needed to the `view` function. First, the `Frame` argument is no longer used, so needs to be removed. Second, the last line of the function, `draw.to_frame(app, &frame).unwrap();` needs to be removed. Rendering the frame is now done automatically.

* For all variations except schotter1, a change is needed to the `update` function: The `Update` argument is no longer used, so needs to be removed.

* For all variations except schotter1, when new windows are created in the `model` function, the final `.unwrap()` needs to be removed.

* For all variations except schotter1, keyboard event handling now uses the Bevy `KeyCode` instead of `Key`. So the last parameter of `key_pressed` is now type `KeyCode`, and the match statement patterns change from the form `Key::R` to `KeyCode::KeyR`.

* For all variations except schotter1, `capture_frame` is replaced with `save_screenshot`, which works the same.

* For schotter2 and schotter3, there are changes to the seedable random number generator. The method `gen_range` is replace with `random_range`. Also, the use statement for `nannou::rand::Rng` is replaced with `nannou::rand::RngExt`.

The rest of the items pertain to egui, so are applicable only to schotter3 and schotter4.

* Egui is now a nannou feature and needs to be enabled in the Cargo.toml file. In the [dependencies] section, remove the `nannou_egui` line and add `features = ["egui"]` to the `nannou` line. If versions are specified in the individual project Cargo.toml files, this might look like: 
```
nannou = { version = "0.20", features = ["egui"] }
```

But since I changed schotter to use workspace dependencies by adding
```
[workspace.dependencies]
nannou = "0.20"
```
to the workspace Cargo.toml file, adding the egui feature looks like this:
```
nannou = { workspace = true, features = ["egui"] }
```

* In addition, the `use nannou_egui` statement needs to be removed.

* The `app.main_window()` method now works correctly so there is no need to save the main window in the model, so I removed it. But I added `.primary()` to the `new_window` builder to make `app.main_window()` identify it. And I changed the "S" keypress code to use it (the code is now the same as in schotter2).

* The `update_ui` function now needs the ui window instead of an Egui state. So I removed `ui: Egui,` from the model and added `ui_window: Entity`. (The previous `WindowId` is replaced by `Entity`.) I made appropriate changes to the `model` function to set this to the ui window.

* Getting the egui context is now done using the `app.egui_for_window()` method. So I added an `App` parameter to `update_ui` and added the app when it is called in `update`. The first line of `update_ui` is now:
```
    let ctx = app.egui_for_window(model.ui_window);
```

* For some reason, an egui panel will only work in the first window that was created. So in `model`, I moved the ui_window creation to before the main window creation. I suspect this is a bug that will be fixed in a future version, but that workaround is fine for now.

* I noticed that closing the ui window causes lots of warnings to be logged to the console complaining that the window is missing. To avoid this, I added some code to the beginning of `update` to quit the program if either window is closed:

```
    if app.window_count() < 2 {
        app.quit();
    }
```
