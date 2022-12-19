# hyprcompanion
A helper to achieve a more DWM-like workflow on Hyprland

# What it does
It helps create a more DWM like way of managing workspaces (Each monitor has tags 1-9). It will also create such workspaces for a newly connected monitor. 

It also helps with focusing monitors, having the ability to "loop" from the rightmost monitor to the leftmost and vice versa.

Final thing it does is provide waybar JSON for workspaces and for the one that is currently selected.

# Usage
Bind workspaces you want to monitors in this scheme: `wsbind=<monitor id><workspace number>,<output that has the same id>`. For a monitor ID of 0 just leave it out. Also set the default workspace for monitors appropriately.

That looks like this in practice:
```
workspace=DP-2,1
workspace=HDMI-A-2,11
workspace=HDMI-A-1,21

wsbind=1,DP-2
...
wsbind=9,DP-2

wsbind=11,HDMI-A-1
...
wsbind=19,HDMI-A-1

wsbind=21,HDMI-A-2
...
wsbind=29,HDMI-A-2
```

When Hyprland is set up to handle it set up your workspace binds to run hyprcompanion with the appropriate arguments. Same goes for the display functionality.

# Credits
[hyprland-rs](https://github.com/hyprland-community/hyprland-rs): The library for interfacing with Hyprland
[hyprsome](https://github.com/sopa0/hyprsome): Inspiration, made this due to some issues with it and for fun
