# WordleClone
Own implementation of Wordle as a CLI app.

## Preview
![During](src/images/during.png "Sample Image 1")
![Won](src/images/won.png "Sample Image 2")
![Lost](src/images/lost.png "Sample Image 3")

## Note
This program doesn't display properly in Window's Command Prompt due to the terminal's 
lack of native color features. If you're in Windows, you can run this smoothly in the 
built-in Powershell or in Git Bash if you have it.

## How to Build
```
cargo install wordle_clone
```

## How to Play
* Run `wordle_clone` in the command-line.
* If you're not familiar with the game, 
  Press `H` at the start of the game to view the game mechanics.

## What's new with version 0.1.4
* Added a high score feature.

## Dependencies
The _colored_ library for providing convenience function for printing colored
text in the terminal.
