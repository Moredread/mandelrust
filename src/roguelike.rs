// Roguelike game implementation

use rand::Rng;

/// A position on the game map
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    /// Create a new position
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    /// Add a delta to this position, returning a new position
    pub fn add(&self, delta: &Position) -> Position {
        Position {
            x: self.x + delta.x,
            y: self.y + delta.y,
        }
    }

    /// Calculate Chebyshev distance (for 8-directional movement)
    /// This is the maximum of the absolute differences in x and y
    pub fn distance(&self, other: &Position) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx.max(dy)
    }

    /// Get all 8 neighboring positions
    pub fn neighbors(&self) -> Vec<Position> {
        let mut result = Vec::with_capacity(8);
        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                result.push(Position {
                    x: self.x + dx,
                    y: self.y + dy,
                });
            }
        }
        result
    }
}

/// Types of tiles in the dungeon
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    /// A wall that blocks movement and sight
    Wall,
    /// An open floor tile
    Floor,
    /// Stairs leading down to the next level
    StairsDown,
}

impl Tile {
    /// Returns true if this tile can be walked on
    pub fn is_walkable(&self) -> bool {
        match self {
            Tile::Floor | Tile::StairsDown => true,
            Tile::Wall => false,
        }
    }

    /// Returns true if this tile blocks line of sight
    pub fn blocks_sight(&self) -> bool {
        match self {
            Tile::Wall => true,
            Tile::Floor | Tile::StairsDown => false,
        }
    }

    /// Convert tile to ASCII character for display
    pub fn to_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::StairsDown => '>',
        }
    }
}

/// A dungeon map containing tiles
#[derive(Debug, Clone)]
pub struct Map {
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    /// Create a new map filled with walls
    pub fn new(width: i32, height: i32) -> Self {
        let size = (width * height) as usize;
        Map {
            width,
            height,
            tiles: vec![Tile::Wall; size],
        }
    }

    /// Get the width of the map
    pub fn width(&self) -> i32 {
        self.width
    }

    /// Get the height of the map
    pub fn height(&self) -> i32 {
        self.height
    }

    /// Check if a position is within map bounds
    pub fn in_bounds(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.x < self.width && pos.y >= 0 && pos.y < self.height
    }

    /// Convert position to index in the tiles vector
    fn pos_to_index(&self, pos: &Position) -> Option<usize> {
        if self.in_bounds(pos) {
            Some((pos.y * self.width + pos.x) as usize)
        } else {
            None
        }
    }

    /// Get the tile at a position
    pub fn get_tile(&self, pos: &Position) -> Option<Tile> {
        self.pos_to_index(pos).map(|idx| self.tiles[idx])
    }

    /// Set the tile at a position
    pub fn set_tile(&mut self, pos: &Position, tile: Tile) {
        if let Some(idx) = self.pos_to_index(pos) {
            self.tiles[idx] = tile;
        }
    }

    /// Check if a position is walkable
    pub fn is_walkable(&self, pos: &Position) -> bool {
        self.get_tile(pos).map_or(false, |tile| tile.is_walkable())
    }

    /// Generate a dungeon using cellular automata
    pub fn generate_dungeon<R: rand::Rng>(width: i32, height: i32, rng: &mut R) -> Self {
        let mut map = Map::new(width, height);

        // Step 1: Fill with random walls/floors (45% chance of floor)
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if rng.gen::<f32>() < 0.45 {
                    map.set_tile(&Position::new(x, y), Tile::Floor);
                }
            }
        }

        // Step 2: Apply cellular automata rules 4 times
        for _ in 0..4 {
            map = map.apply_cellular_automata();
        }

        map
    }

    /// Apply cellular automata rules to smooth the map
    fn apply_cellular_automata(&self) -> Self {
        let mut new_map = self.clone();

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                let pos = Position::new(x, y);
                let wall_count = self.count_walls_around(&pos);

                // If 5 or more neighbors are walls, become a wall
                // Otherwise, become a floor
                if wall_count >= 5 {
                    new_map.set_tile(&pos, Tile::Wall);
                } else {
                    new_map.set_tile(&pos, Tile::Floor);
                }
            }
        }

        new_map
    }

    /// Count walls in the 8 positions around a position
    fn count_walls_around(&self, pos: &Position) -> i32 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let check_pos = Position::new(pos.x + dx, pos.y + dy);
                if let Some(tile) = self.get_tile(&check_pos) {
                    if tile == Tile::Wall {
                        count += 1;
                    }
                } else {
                    // Out of bounds counts as wall
                    count += 1;
                }
            }
        }
        count
    }

    /// Find a random floor position on the map
    pub fn find_random_floor_position<R: rand::Rng>(&self, rng: &mut R) -> Option<Position> {
        let mut attempts = 0;
        while attempts < 1000 {
            let x = rng.gen_range(1..self.width - 1);
            let y = rng.gen_range(1..self.height - 1);
            let pos = Position::new(x, y);

            if self.get_tile(&pos) == Some(Tile::Floor) {
                return Some(pos);
            }
            attempts += 1;
        }
        None
    }
}

/// An entity in the game (player or monster)
#[derive(Debug, Clone)]
pub struct Entity {
    pub pos: Position,
    pub glyph: char,
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
}

impl Entity {
    /// Create a new entity
    pub fn new(pos: Position, glyph: char, name: &str, max_hp: i32, attack: i32) -> Self {
        Entity {
            pos,
            glyph,
            name: name.to_string(),
            hp: max_hp,
            max_hp,
            attack,
        }
    }

    /// Check if the entity is alive
    pub fn is_alive(&self) -> bool {
        self.hp > 0
    }

    /// Take damage
    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    /// Heal
    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }
}

/// The main game state
#[derive(Debug, Clone)]
pub struct GameState {
    pub map: Map,
    pub player: Entity,
    pub enemies: Vec<Entity>,
    pub dungeon_level: i32,
    pub messages: Vec<String>,
}

impl GameState {
    /// Create a new game
    pub fn new<R: rand::Rng>(rng: &mut R) -> Self {
        let map = Map::generate_dungeon(80, 45, rng);

        // Find a random floor position for the player
        let player_pos = map
            .find_random_floor_position(rng)
            .expect("Could not find floor position for player");

        let player = Entity::new(player_pos, '@', "Player", 30, 5);

        // Spawn some enemies
        let mut enemies = Vec::new();
        for _ in 0..10 {
            if let Some(pos) = map.find_random_floor_position(rng) {
                let enemy_type = rng.gen_range(0..3);
                let enemy = match enemy_type {
                    0 => Entity::new(pos, 'g', "Goblin", 10, 3),
                    1 => Entity::new(pos, 'o', "Orc", 16, 4),
                    _ => Entity::new(pos, 'T', "Troll", 24, 6),
                };
                enemies.push(enemy);
            }
        }

        GameState {
            map,
            player,
            enemies,
            dungeon_level: 1,
            messages: vec!["Welcome to the Dungeon of the Fallen King!".to_string()],
        }
    }

    /// Try to move the player by a delta
    pub fn try_move_player(&mut self, delta: &Position) {
        let new_pos = self.player.pos.add(delta);

        // Check for enemy at target position
        if let Some(enemy_idx) = self.find_enemy_at(&new_pos) {
            // Attack the enemy
            self.attack_enemy(enemy_idx);
        } else if self.map.is_walkable(&new_pos) {
            // Move the player
            self.player.pos = new_pos;
        }
    }

    /// Find an enemy at a position
    fn find_enemy_at(&self, pos: &Position) -> Option<usize> {
        self.enemies
            .iter()
            .position(|e| e.is_alive() && e.pos == *pos)
    }

    /// Player attacks an enemy
    fn attack_enemy(&mut self, enemy_idx: usize) {
        let damage = self.player.attack;
        self.enemies[enemy_idx].take_damage(damage);

        let enemy_name = self.enemies[enemy_idx].name.clone();

        if self.enemies[enemy_idx].is_alive() {
            self.messages.push(format!(
                "You hit the {} for {} damage!",
                enemy_name, damage
            ));
        } else {
            self.messages
                .push(format!("You killed the {}!", enemy_name));
        }
    }

    /// Enemy attacks player
    fn enemy_attacks_player(&mut self, enemy_idx: usize) {
        let damage = self.enemies[enemy_idx].attack;
        self.player.take_damage(damage);

        let enemy_name = &self.enemies[enemy_idx].name;
        self.messages
            .push(format!("The {} hits you for {} damage!", enemy_name, damage));
    }

    /// Run enemy AI turns
    pub fn enemy_turns(&mut self) {
        for i in 0..self.enemies.len() {
            if !self.enemies[i].is_alive() {
                continue;
            }

            let enemy_pos = self.enemies[i].pos;
            let player_pos = self.player.pos;

            // Simple AI: move towards player if close
            if enemy_pos.distance(&player_pos) <= 10 {
                let dx = (player_pos.x - enemy_pos.x).signum();
                let dy = (player_pos.y - enemy_pos.y).signum();
                let delta = Position::new(dx, dy);
                let new_pos = enemy_pos.add(&delta);

                // Check if at player position
                if new_pos == player_pos {
                    // Attack player
                    self.enemy_attacks_player(i);
                } else if self.map.is_walkable(&new_pos) {
                    // Check no other enemy is there
                    if self.find_enemy_at(&new_pos).is_none() {
                        self.enemies[i].pos = new_pos;
                    }
                }
            }
        }

        // Remove dead enemies
        self.enemies.retain(|e| e.is_alive());
    }

    /// Check if the game is over
    pub fn is_game_over(&self) -> bool {
        !self.player.is_alive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for Position structure
    #[test]
    fn test_position_creation() {
        let pos = Position { x: 5, y: 10 };
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
    }

    #[test]
    fn test_position_equality() {
        let pos1 = Position { x: 3, y: 4 };
        let pos2 = Position { x: 3, y: 4 };
        let pos3 = Position { x: 5, y: 4 };

        assert_eq!(pos1, pos2);
        assert_ne!(pos1, pos3);
    }

    #[test]
    fn test_position_add() {
        let pos = Position { x: 5, y: 10 };
        let delta = Position { x: 2, y: -3 };
        let result = pos.add(&delta);

        assert_eq!(result.x, 7);
        assert_eq!(result.y, 7);
    }

    #[test]
    fn test_position_distance() {
        let pos1 = Position { x: 0, y: 0 };
        let pos2 = Position { x: 3, y: 4 };

        // Using Chebyshev distance (max of abs differences) - used for 8-directional movement
        assert_eq!(pos1.distance(&pos2), 4);

        let pos3 = Position { x: 5, y: 2 };
        assert_eq!(pos1.distance(&pos3), 5);
    }

    #[test]
    fn test_position_neighbors() {
        let pos = Position { x: 5, y: 5 };
        let neighbors = pos.neighbors();

        // Should return all 8 adjacent positions
        assert_eq!(neighbors.len(), 8);
        assert!(neighbors.contains(&Position { x: 4, y: 4 })); // NW
        assert!(neighbors.contains(&Position { x: 5, y: 4 })); // N
        assert!(neighbors.contains(&Position { x: 6, y: 4 })); // NE
        assert!(neighbors.contains(&Position { x: 4, y: 5 })); // W
        assert!(neighbors.contains(&Position { x: 6, y: 5 })); // E
        assert!(neighbors.contains(&Position { x: 4, y: 6 })); // SW
        assert!(neighbors.contains(&Position { x: 5, y: 6 })); // S
        assert!(neighbors.contains(&Position { x: 6, y: 6 })); // SE
    }

    // Tests for Tile enum
    #[test]
    fn test_tile_is_walkable() {
        assert!(Tile::Floor.is_walkable());
        assert!(Tile::StairsDown.is_walkable());
        assert!(!Tile::Wall.is_walkable());
    }

    #[test]
    fn test_tile_blocks_sight() {
        assert!(Tile::Wall.blocks_sight());
        assert!(!Tile::Floor.blocks_sight());
        assert!(!Tile::StairsDown.blocks_sight());
    }

    #[test]
    fn test_tile_to_char() {
        assert_eq!(Tile::Wall.to_char(), '#');
        assert_eq!(Tile::Floor.to_char(), '.');
        assert_eq!(Tile::StairsDown.to_char(), '>');
    }

    // Tests for Map structure
    #[test]
    fn test_map_creation() {
        let map = Map::new(10, 8);
        assert_eq!(map.width(), 10);
        assert_eq!(map.height(), 8);
    }

    #[test]
    fn test_map_in_bounds() {
        let map = Map::new(10, 8);
        assert!(map.in_bounds(&Position::new(0, 0)));
        assert!(map.in_bounds(&Position::new(9, 7)));
        assert!(!map.in_bounds(&Position::new(10, 7)));
        assert!(!map.in_bounds(&Position::new(5, 8)));
        assert!(!map.in_bounds(&Position::new(-1, 5)));
    }

    #[test]
    fn test_map_get_set_tile() {
        let mut map = Map::new(10, 8);
        map.set_tile(&Position::new(5, 3), Tile::Wall);
        assert_eq!(map.get_tile(&Position::new(5, 3)), Some(Tile::Wall));

        map.set_tile(&Position::new(2, 1), Tile::Floor);
        assert_eq!(map.get_tile(&Position::new(2, 1)), Some(Tile::Floor));

        // Out of bounds returns None
        assert_eq!(map.get_tile(&Position::new(20, 20)), None);
    }

    #[test]
    fn test_map_is_walkable() {
        let mut map = Map::new(10, 8);
        map.set_tile(&Position::new(5, 3), Tile::Wall);
        map.set_tile(&Position::new(2, 1), Tile::Floor);

        assert!(!map.is_walkable(&Position::new(5, 3)));
        assert!(map.is_walkable(&Position::new(2, 1)));
        assert!(!map.is_walkable(&Position::new(20, 20))); // out of bounds
    }

    // Tests for dungeon generation
    #[test]
    fn test_generate_dungeon_has_floor() {
        let mut rng = rand::thread_rng();
        let map = Map::generate_dungeon(40, 30, &mut rng);

        // Count floor tiles
        let mut floor_count = 0;
        for y in 0..map.height() {
            for x in 0..map.width() {
                if let Some(Tile::Floor) = map.get_tile(&Position::new(x, y)) {
                    floor_count += 1;
                }
            }
        }

        // Should have at least some floor tiles
        assert!(floor_count > 100, "Dungeon should have floor tiles");
    }

    #[test]
    fn test_generate_dungeon_has_borders() {
        let mut rng = rand::thread_rng();
        let map = Map::generate_dungeon(40, 30, &mut rng);

        // Check all borders are walls
        for x in 0..map.width() {
            assert_eq!(map.get_tile(&Position::new(x, 0)), Some(Tile::Wall));
            assert_eq!(
                map.get_tile(&Position::new(x, map.height() - 1)),
                Some(Tile::Wall)
            );
        }
        for y in 0..map.height() {
            assert_eq!(map.get_tile(&Position::new(0, y)), Some(Tile::Wall));
            assert_eq!(
                map.get_tile(&Position::new(map.width() - 1, y)),
                Some(Tile::Wall)
            );
        }
    }

    #[test]
    fn test_find_random_floor_position() {
        let mut rng = rand::thread_rng();
        let map = Map::generate_dungeon(40, 30, &mut rng);

        let pos = map.find_random_floor_position(&mut rng);
        assert!(pos.is_some());

        if let Some(p) = pos {
            assert_eq!(map.get_tile(&p), Some(Tile::Floor));
        }
    }

    // Tests for Entity
    #[test]
    fn test_entity_creation() {
        let player = Entity::new(Position::new(5, 5), '@', "Player", 30, 5);
        assert_eq!(player.pos, Position::new(5, 5));
        assert_eq!(player.glyph, '@');
        assert_eq!(player.name, "Player");
        assert_eq!(player.hp, 30);
        assert_eq!(player.max_hp, 30);
        assert_eq!(player.attack, 5);
    }

    #[test]
    fn test_entity_is_alive() {
        let mut entity = Entity::new(Position::new(0, 0), 'g', "Goblin", 10, 3);
        assert!(entity.is_alive());

        entity.hp = 0;
        assert!(!entity.is_alive());

        entity.hp = -5;
        assert!(!entity.is_alive());
    }

    #[test]
    fn test_entity_take_damage() {
        let mut entity = Entity::new(Position::new(0, 0), 'o', "Orc", 20, 5);

        entity.take_damage(5);
        assert_eq!(entity.hp, 15);

        entity.take_damage(20);
        assert_eq!(entity.hp, -5);
        assert!(!entity.is_alive());
    }

    #[test]
    fn test_entity_heal() {
        let mut entity = Entity::new(Position::new(0, 0), '@', "Player", 30, 5);
        entity.hp = 10;

        entity.heal(5);
        assert_eq!(entity.hp, 15);

        entity.heal(100);
        assert_eq!(entity.hp, 30); // capped at max_hp
    }

    // Tests for GameState
    #[test]
    fn test_gamestate_creation() {
        let mut rng = rand::thread_rng();
        let game = GameState::new(&mut rng);

        assert!(game.player.is_alive());
        assert!(game.enemies.len() > 0);
        assert_eq!(game.dungeon_level, 1);
    }

    #[test]
    fn test_gamestate_player_move() {
        let mut rng = rand::thread_rng();
        let mut game = GameState::new(&mut rng);

        let start_pos = game.player.pos;
        let delta = Position::new(1, 0);
        let new_pos = start_pos.add(&delta);

        // Set new position to floor to ensure move is valid
        game.map.set_tile(&new_pos, Tile::Floor);

        game.try_move_player(&delta);

        // Player should have moved if the tile was walkable
        if game.map.is_walkable(&new_pos) {
            assert_eq!(game.player.pos, new_pos);
        }
    }

    #[test]
    fn test_gamestate_combat() {
        let mut game = GameState {
            map: Map::new(20, 20),
            player: Entity::new(Position::new(5, 5), '@', "Player", 30, 10),
            enemies: vec![Entity::new(Position::new(6, 5), 'g', "Goblin", 10, 3)],
            dungeon_level: 1,
            messages: Vec::new(),
        };

        // Set tiles to floor
        game.map.set_tile(&Position::new(5, 5), Tile::Floor);
        game.map.set_tile(&Position::new(6, 5), Tile::Floor);

        let initial_goblin_hp = game.enemies[0].hp;

        // Try to move into goblin (should trigger combat)
        game.try_move_player(&Position::new(1, 0));

        // Goblin should have taken damage
        assert!(game.enemies[0].hp < initial_goblin_hp);
    }
}
