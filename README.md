# ConfigFileSwitcher
My first project that I would actually use


# Usage

## Help menu '-h'
It shows you (almost) the exact same text you're reading right now!
Example: cfs -h


## Saving files '-s'
To load a file, you actually have to save a file (obviously)
You have to specify the file, where it should be loaded at, and what name it should have
In this example, we save a file called 'somefile.txt' as mainhyprlandconfig so it gets loaded at '/home/user/.config/hypr/hyprland.conf':

cfs -s hypr/hyprland.conf somefile.txt mainhyprlandconfig


## Loading files '-l'
After you have saved a file, you can load it at anytime by just doing:

cfs -l mainhyprlandconfig


## Delete saved files '-r'
After you had enough fun, you can do this to delete the file:

cfs -r mainhyprlandconfig


## Toggle between 2 files '-t'
Now this one is interesting
You specify 2 files, that's it
It automatically detects the current one, and replaces it with the other one
You can do:

cfs -t mainhyprlandconfig 2ndconfig

Special cases:
- The files have to be for the same config ("/home/user/.config/hypr/hyprland.conf" in this example)
- When none of those files is loaded, it loads the second one
 

## List saved files '-ls'
It's the only command with more than one character after the dash...
But it's still nothing special:

cfs -ls
