# About

A naive [spinlock](https://en.wikipedia.org/wiki/Spinlock)

## Problems with it

- Not fair
- Wastes cpu cycles

## Don't want to use atomics?

Check out [Peterson's algorithm](https://github.com/PoorlyDefinedBehaviour/petersons_mutual_exclusion_algorithm)
