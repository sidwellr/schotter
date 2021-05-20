# Schotter Four Ways

*Schotter* (German for gravel) is a piece by computer art pioneer Georg Nees. It consists of a grid of squares 12 across and 22 down with random rotation and displacement that increases towards the bottom.

![](images/schotter.jpg)

The original was created using a pen plotter. I don't know if Dr. Nees had access to a graphics terminal (fairly rare at the time) to preview the result or if he ran the program multiple time to get different results before choosing this as the final.

*Schotter* is fairly simple to code; at least it is today with modern graphics libraries. Several other people have written tutorials for how to code this using various platforms. I'm adding to this collection with a series of tutorials for coding it in Rust using the Nannou library.

This tutorial assumes you are familiar with the Rust language and, if you will be following along (which I certainly encourage), access to a working Rust coding environment. We will be using a Rust "workspace", which allows grouping multiple related programs together. (You can learn more about workspaces in chapters of [*The Rust Programming Language*](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) or [*The Cargo Book*](https://doc.rust-lang.org/cargo/reference/workspaces.html).)

Here are the steps I use for creating a Rust workspace:

1. Create a new directory for the workspace.
2. Create a new file named "Cargo.toml" in that directory with the following contents:
    ```
    [workspace]
    members=[

    ]
    ```
3. Optionally initialize configuration management for that directory. I like to use git for this. When I get something I like (and that works), I commit the files in git. Then as I make changes, I can easily compare what I have done to that version, and I can easily revert back to that version if I really mess things up and want to start again. It also make it easy to publish the result online (for example, on GitHub). There are two steps:
    * Initialize the repository. This can be done with the command line (```git init```). Alternatively, many editors have a method to do this, for example in VS Code, click the Source Control button on the left and click Initialize Repository.

    * Create a file ".gitignore" containing the files that should be managed by git. For Rust programs, there are two, so the file contains their names:
        ```
        /target
        Cargo.lock
        ```

4. Optionally create standard information files in the directory:
    * LICENSE.txt, containing the license you want to use. You can pick one from https://choosealicense.com/; I like the permissive MIT license for most of my generative art code. Just copy the license text, paste it into LICENSE.txt, and fill in the copyright information (year and your name).
    * README.md, containing general information about the project. If you publish it to GitHub, this file will be displayed on the main repository page.

We'll be coding *Schotter* four different ways, each building on the previous version. Click a link for a short tutorial; they are designed to be done in order. You can find the end result for each tutorial in the appropriate repository directory.

[Schotter1](schotter1.md): Using the Nannou "sketch" facility to code a simple version with no frills.

[Schotter2](schotter2.md): Changing the code to a Nannou "app" with persistent data and some simple keyboard commands to adjust some parameters.

[Schotter3](schotter3.md): Adding a control panel to make it easier to adjust parameters on the fly.

[Schotter4](schotter4.md): Animating the squares, so they move from the starting grid to their displaced positions, and from there to new positions.

I may add some variations later; after all, that's what creative coding is all about! They may or may not be accompanied by full tutorials. But that's it for now.
