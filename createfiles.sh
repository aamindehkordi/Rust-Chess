#!/bin/bash

# Create directories
mkdir -p src/model/pieces
mkdir -p src/view
mkdir -p src/controller

# Create files
touch src/model/mod.rs
touch src/model/game.rs
touch src/model/board.rs
touch src/model/pieces/mod.rs
touch src/model/pieces/piece.rs
touch src/model/pieces/pawn.rs
touch src/model/pieces/rook.rs
touch src/model/pieces/knight.rs
touch src/model/pieces/bishop.rs
touch src/model/pieces/queen.rs
touch src/model/pieces/king.rs
touch src/view/mod.rs
touch src/view/console_view.rs
touch src/controller/mod.rs
touch src/controller/game_controller.rs
