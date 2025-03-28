# CS128HonorsProject
This is the CS128 Honors project in Rust. Our group is Sean Patel, Shrest Das, and Harsheet Bansal. We are attempting to build a multiplayer Snake game, with our preliminary target (Minimum Viable Product) being to create a single-player Snake game first.

Goals and Objectives: 

  Implement the correct logic for incrementing the snake's length when it eats an apple 
  
  Implement the correct logic for the snake hitting the boundary or another snake (multiplayer)
  
  Correctly spawn apples in a random space that is NOT occupied by a snake at the instant the apple is spawned 
  
  Track current and high scores for single or multiplayers
  
  Implement a winning condition (all spaces occupied - single player; most apples - multiplayer when snakes crash)
  
  Potential features: more than one apple, different snake speeds, different board size (be able to choose these settings as a way to adjust difficulty)
  
  Create a visualization (UI) for the player to be able to see the board
  
  Different shades at each length of the snake to determine start, middle, and end of snake
  
Technical Description:

  Create a data structure (linked list) to visualize and modify occupied spaces
  
  Shading algorithm that colors the snake based on its size and number of apples eaten
  
  Optimizations are necessary to ensure that game runs smoothly in real time
  
  Variables that track and modify current and high scores
  
  Conditions that are consistently checked to see if the game is over
  
  The snake must be represented by a linked list with Nodes that represent the coordinates at which the snake is presently occupying
  
Checkpoint Objectives:

  First Checkpoint: Build visualization to ensure smoother debugging
  
  Second Checkpoint: Complete implementation of single player Snake game
  
  Final Deadline: Implement multi-functionality and game modes
  
Challenges:

  Difficult to create visualizations
  
  Multiplayer: synchronizing processes and networking for two players to play at the same time but also ensuring that each player's movements are independent and do not affect the other player's movement (each snake has control of its own movements)
  
  Making sure that all processes are fast. Our algorithm may be slow which can affect ease of playing
  
  Spawning apples in an open spot unoccupied by snake part
  
References:

  Google Snake game and its various settings
