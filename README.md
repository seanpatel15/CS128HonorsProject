# CS128HonorsProject
This is the CS128 Honors project in Rust. Our group is Sean Patel, Shrest Das, and Harsheet Bansal. We are attempting to build a multiplayer Snake game, with our preliminary target (Minimum Viable Product) being to create a single-player Snake game first.

**Goals and Objectives:** 

  &emsp; Implement the correct logic for incrementing the snake's length when it eats an apple 
  
  &emsp; Implement the correct logic for the snake hitting the boundary or another snake (multiplayer)
  
  &emsp; Correctly spawn apples in a random space that is NOT occupied by a snake at the instant the apple is spawned 
  
  &emsp; Track current and high scores for single or multiplayers
  
  &emsp; Implement a winning condition (all spaces occupied - single player; most apples - multiplayer when snakes crash)
  
  &emsp; Potential features: more than one apple, different snake speeds, different board size (be able to choose these settings as a way to adjust difficulty)
  
  &emsp; Create a visualization (UI) for the player to be able to see the board
  
  &emsp; Different shades at each length of the snake to determine start, middle, and end of snake
  
**Technical Description:**

  &emsp; Create a data structure (linked list) to visualize and modify occupied spaces
  
  &emsp; Shading algorithm that colors the snake based on its size and number of apples eaten
  
  &emsp; Optimizations are necessary to ensure that game runs smoothly in real time
  
  &emsp; Variables that track and modify current and high scores
  
  &emsp; Conditions that are consistently checked to see if the game is over
  
  &emsp; The snake must be represented by a linked list with Nodes that represent the coordinates at which the snake is presently occupying
  
**Checkpoint Objectives:**

  &emsp; **First Checkpoint:** Build visualization to ensure smoother debugging
  
  &emsp; **Second Checkpoint:** Complete implementation of single player Snake game
  
  &emsp; **Final Deadline:** Implement multi-functionality and game modes
  
**Challenges:**

  &emsp; Difficult to create visualizations
  
  &emsp; Multiplayer: synchronizing processes and networking for two players to play at the same time but also ensuring that each player's movements are independent and do not affect the other player's movement (each snake has control of its own movements)
  
  &emsp; Making sure that all processes are fast. Our algorithm may be slow which can affect ease of playing
  
  &emsp; Spawning apples in an open spot unoccupied by snake part
  
**References:**

  &emsp; Google Snake game and its various settings
