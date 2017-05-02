pub enum GameEvent {
    LevelStart,
    PlayerDead { player_id: i8 },
    EnemyKilled { enemy_id: i8 },
    PlayerReachedTop,
}

/*
LevelStart: The game starts by showing off the mountain (camera pans up). Once the camera
has reached the top of the mountain, the LevelStart event is pushed. This signals each
system to begin normal gameplay operations.

PlayerDead: Signifies a player has died. "player_id" will tell which player died.

EnemyKilled: Signifies an enemy has been killed. The enemy id tells which type of
enemy was killed.

PlayerReachedTop: Thrown when the player reaches the top. Control will be taken from
the player and the game will display a "congrats" before moving to the next mountain.
*/
