# wug-hunt-rust
An implementation of the wug hunt game in Rust.

## How to use
Download and save the latest version from the releases section. Now, navigate through a command line application to the downloaded file's location. If on Linux, type:
```
chmod +x wug-hunt-linux  
./wug-hunt-linux
```
For Windows, type:
```
.\wug-hunt-windows.exe
```

## The game
In the game Wug Hunt, three _wugs_ will be hidden on a number somewhere between 1 and 100. It's your task to find all three wugs as quick as possible. For every round, you will be prompted a left and right boundary. Next, if one of the boundaries happen to be the location of a wug, this wug will be _found_ and removed from the game. Now you will be told how many wugs are enclosed in the interval defined by the boundaries. Once all three wugs have been found, the game is over and the number of tries needed will be displayed. 