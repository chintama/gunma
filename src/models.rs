struct Team(u64);

const WORLD_WIDTH: f32 = 100000.0;
const WORLD_HEIGHT: f32 = 100000.0;

struct World {
    size: Vector,
}

struct Screen {}

trait Sprite {
    fn rect(&self) -> &Rectangle;
    fn image(&self) -> &Image;
    fn transform(&self) -> &Transform;
}

trait Item: Sprite {}

struct Player {
    rect: Rectangle,
    image: Image,
    transform: Transform,
    team: Team,
    item: Option<Box<Item>>,
}

struct Item {
    rect: Rectangle,
    image: Image,
    transform: Transform,
    team: Team,
}

struct Landmark {
    rect: Rectangle,
    image: Image,
    transform: Transform,
    team: Team,
}

struct Weapon {
    rect: Rectangle,
    image: Image,
    transform: Transform,
    team: Team,
}

enum Action {
    Left,
    Rigt,
    Jump,
    Take,
    Use,
    Drop,
}
