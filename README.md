# Chromosome Toolkit

Chromosome Toolkit is a collection of tools for creating modifications for games based on the Genome Engine.
As of now, the focus is on the version used for Risen 1.

This has no relationship with THQ Nordic or any of the original companies involved with the games (e.g., Piranha Bytes or Deep Silver).
Some names used may be trademarks of their respective owners and are only used to accurately refer to (parts of) the corresponding software;
there is no endorsement of this project by the owners.

# Usage

## File converters (crates in crates/bins)
Most of the tools included convert between the data formats used by the game and formats which can be edited using 'normal' tools.

To convert a file, run the executable with the filename of the input as it's only parameter (e.g. by dragging the file onto the executable using Explorer).
If you want to convert multiple files, you can instead provide multiple paths via cli, or point the executable to a folder, which will be recursivly searched for files with the correct file extension.

## Scripts
Mods that compile to DLLs to be loaded by the game engine to modify behaviour

## Formats crate
Implements file-formats and in-memory structures of the game

## Attach crate 
Implements the basic functionality for attaching to in-memory C++ constructs

# License
The code in this repository is licensed under the MIT License, see the `LICENSE` file for details

# Acknowledgements

Providing great resources to learn how Risen works in the World of Players forums:

- NicoDE
- Baltram
- Galrath434
