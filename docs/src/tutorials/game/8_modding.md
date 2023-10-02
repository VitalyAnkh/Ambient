# Chapter 8: Modding

In this final chapter, we'll look at modding. All games are moddable by default with Ambient.

## Adding the mod manager UI

We'll start by adding the Mod manager UI to your game, so that we can list and enable mods
easily in your game. Start by adding the followng to your
`ambient.toml`'s dependencies:

```toml
package_manager = { deployment = "4hEHArTmKMprisqnQPNxLK" }
```

(To find the latest deployment, [go to the package mananger on the website](https://ambient-733e7.web.app/packages/hr4pxz7kfhzgimicoyh65ydel3aehuhk))

Then add this to the top of your `server.rs`'s main function:

```rust
entity::add_component(
    package_manager::entity(),
    package_manager::components::mod_manager_for(),
    packages::this::entity(),
);
```

Launch your game, and then press F4 to open the Mod manager. From here, you can enable and disable mods.

> **Note**: You can build your own mod manager UI if you want to. You can see the source code for the
> default one [here](https://github.com/AmbientRun/Ambient/tree/main/guest/rust/packages/tools/package_manager).

## Creating a mod

To create a mod for your game, simply type `ambient new my_mod`. Then update the `ambient.toml` with this:

```toml
# Note: There will already be another content field in the toml. Replace it with this:
content = { type = "Mod", for_playables = ["cyjzy4nxfwdpzh34g3ozntjkypd7f5ot"] }

[dependencies]
# Note: This line will make it possible to run the mod locally, as it will pull in your game as a dependency.
# Replace LATEST_DEPLOYMENT_ID with the latest deployment id of your game
# When you want to deploy the mod, comment this line out first
my_game = { deployment = "LATEST_DEPLOYMENT_ID" }
```

You can now edit the code in `src/` like you would normally, and run it as you would normally. Once you're
happy with your mod you can deploy it with `ambient deploy` (just like we did with the game). Remember to comment
out the `my_game = ..` line before deploying.