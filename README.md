## Mancala solving
Mancala is an entirely deterministic game, so I decided to make a program that would simulate a game and determine the best possible moves.

## TODO
- [] Get the game itself built so there's a framework to play it on.
   - This includes being able to give the game moves, recognizing when a move ends you turn and when it doesn't, and inputting a whole game state in case you want to start using it in the middle of a game without replaying moves to reach that state (as well as for ease of testing).
- [] Make a system for scoring specific moves.
- [] Automate the testing of moves and determining which is the best (keeping in mind that some turns you'll be able to make multiple moves so it might be a sequence of moves as opposed to a single one).
   1. First I'll probably just judge the best turn as the one that gets you the highest score at the end.
   2. Then I can try judging it by also including a  consideration for what move limits the opponent the most
      - For example, if you have one move sequence that makes slightly more than another, but the other one stops your opponent from scoring anything on their turn, then that sequence is almost like 2 turns for you so it's probably better
      - Could also do some testing to determine the specifics, run the program with different weights for you own score vs limiting theirs and maybe have them play against each other to see which one wins the most often.
- [] Grab a 2d graphics crate and pull the game out of the command line.