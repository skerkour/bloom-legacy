# Tron

##[Live Link][link]

[link]: http://momajd.github.io/Tron/

Remake of the classic arcade game that involves maneuvering a 'light cycle' in an arena against an opponent. Tron was built using Javascript, jQuery, HTML, and CSS. Game can be played against a computer or with two players.

![Intro](/docs/intro.png)





![End](/docs/game_end.png)

## Game Architecture and Logic

- Object-oriented - classes are made for `bike`, `board`, `coord`, and `view`.

- The `view` renders the board and listens for key events in the browser. The board grid is constructed using `ul` and `li` tags. Each time the board is rendered, the CSS class selector of each `li` item is updated using jQuery to reflect the position of the bikes.

- The `board` class takes dimensions as arguments and checks if coordinates are within its boundaries.

- The `bike` holds the logic for turns, collisions, and the AI of the computer player.

## Features

- Collision detection

- Two-player option

- Simple AI that makes turning decisions based on the number of available steps in each direction. The computer is programmed to make an occasional random turn to avoid 'wall-hugging'.

- User can change difficulty of the computer player. The easiest computer makes lots of random moves which makes it likely to crash. The hardest computer makes no random moves.

## Future Features

- [ ] Improve AI by using a more advanced algorithm that is based on searching areas (instead of linear paths). Maybe something similar to Flood Fill

- [ ] Add websocket for multi-player games over server
