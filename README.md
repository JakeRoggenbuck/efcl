# Efficiency First Color Library (EFCL)
The most simple, small, and fast terminal color text library.

## Priorities
- Runtime speed
- Dependency size

## Why?

Here is a flamegraph of the [auto-clock-speed](https://github.com/JakeRoggenbuck/auto-clock-speed) project. In green is the time it takes for colored text to render. That is just over 9% of runtime cpu is taken by color formatting.

![image](https://user-images.githubusercontent.com/35516367/201438554-a3c7bd63-2810-4140-a457-da8eff267d21.png)

![EFCL](https://user-images.githubusercontent.com/35516367/223891514-462f831d-19d9-4289-89e5-14fe0ad2c940.png)

This is way too long for what we need it for. In auto-clock-speed, color is used sparingly and only a few default colors on top of that.

![image](https://user-images.githubusercontent.com/35516367/201438673-56254428-515b-4e18-a918-c557703e936e.png)

This library is for the most basic text coloring for the terminal and only includes the default terminal colors, no background color or text styles other than bold. This library is for speed.
