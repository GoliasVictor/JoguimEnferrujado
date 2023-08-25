//use crate::{
//    components::Position,
//    nodes::EntitieData,
//    prelude::*,
//    System,
//};
//pub type CollisionReaction = Box<dyn Fn(&EntitieData, &EntitieData, &mut World) -> ()>;
//
//pub struct Collision {
//    pub collide_with: CollisionReaction,
//}
//struct Collisionable<'a> {
//    pub entitie: EntitieData<'a>,
//    pub sides_positions: SidesPositions,
//    pub collision_data: &'a mut Collision,
//}
//
//const QT_DIVISIONS: i32 = 2;
//struct ColisionPair<'a> {
//    pub a: &'a Collisionable<'a>,
//    pub b: &'a Collisionable<'a>,
//}
//impl<'a> ColisionPair<'a> {
//    fn new(a: &'a Collisionable, b: &'a Collisionable) -> Self {
//        Self { a, b }
//    }
//}
//struct Chunk<'a> {
//    pub entities: Vec<&'a Collisionable<'a>>,
//    pub sides: SidesPositions,
//}
//fn is_coliding(a: &SidesPositions, b: &SidesPositions) -> bool {
//    return a.right > b.left && a.top < b.bottom && b.right > a.left && b.top < a.bottom;
//}
//fn collide(a: &mut Collisionable, b: &mut Collisionable, world: &mut World) {
//    (a.collision_data.collide_with)(&a.entitie, &b.entitie, world);
//    (b.collision_data.collide_with)(&b.entitie, &a.entitie, world);
//}
//fn generate_pairs<'a>(entities: &'a Vec<&'a Collisionable>) -> Vec<ColisionPair<'a>> {
//    let chunks = create_chunks(entities);
//    let mut pairs: Vec<ColisionPair<'a>> = vec![];
//    for chunk in chunks.iter() {
//        let len = chunk.entities.len();
//        for i in 0..len {
//            let a = chunk.entities[i];
//            for j in i..len {
//                let b = chunk.entities[j];
//                if is_coliding(&a.sides_positions, &b.sides_positions) {
//                    pairs.push(ColisionPair::new(a, b));
//                }
//            }
//        }
//    }
//    return pairs;
//}
//
//fn this_shouldnt_be_used<T>( a : &T) -> &mut T{
//    return unsafe {  &mut *(( a as *const T ) as *mut T) }
//
//}
//
//fn collide_vec<'a>(entities: Vec<Collisionable>, world: &'a mut World) {
//    let aux = entities
//        .iter()
//        .collect();
//    let pairs = generate_pairs(&aux);
//    for pair in pairs {
//        let a = this_shouldnt_be_used(pair.a);
//        let b = this_shouldnt_be_used(pair.b);
//        collide(a, b, world);
//    }
//}
//
//fn create_chunks<'a>(entities: &'a Vec<&Collisionable>) -> Vec<Chunk<'a>> {
//    const MAX_ENTITIES: usize = 10;
//
//    let mut final_chunks: Vec<Chunk> = vec![];
//    let mut chunks: Vec<Chunk> = vec![Chunk {
//        entities: entities.clone(),
//        sides: SidesPositions {
//            left: entities
//                .iter()
//                .map(|e| e.sides_positions.left)
//                .reduce(f32::min)
//                .unwrap(),
//            right: entities
//                .iter()
//                .map(|e| e.sides_positions.right)
//                .reduce(f32::max)
//                .unwrap(),
//            top: entities
//                .iter()
//                .map(|e| e.sides_positions.top)
//                .reduce(f32::min)
//                .unwrap(),
//            bottom: entities
//                .iter()
//                .map(|e| e.sides_positions.bottom)
//                .reduce(f32::max)
//                .unwrap(),
//        },
//    }];
//    while let Some(chunk) = chunks.pop() {
//        let sides = chunk.sides;
//        let width = (sides.right - sides.left).abs();
//        let height = (sides.bottom - sides.top).abs();
//        if width == 0. || height == 0. {
//            continue;
//        }
//        let chunks_width: f32 = width / QT_DIVISIONS as f32;
//        let chunks_height: f32 = height / QT_DIVISIONS as f32;
//        for i in 0..QT_DIVISIONS {
//            for j in 0..QT_DIVISIONS {
//                let x = sides.left + chunks_width * i as f32;
//                let y = sides.top + chunks_height * j as f32;
//
//                let mut subchunk = Chunk {
//                    sides: SidesPositions {
//                        left: x,
//                        right: x + chunks_width,
//                        top: y,
//                        bottom: y + chunks_height,
//                    },
//                    entities: vec![],
//                };
//                subchunk.entities = chunk
//                    .entities
//                    .clone()
//                    .into_iter()
//                    .filter(|e| is_coliding(&e.sides_positions, &sides))
//                    .collect();
//                if subchunk.entities.len() > MAX_ENTITIES {
//                    chunks.push(subchunk);
//                } else {
//                    final_chunks.push(subchunk);
//                }
//            }
//        }
//    }
//
//    return final_chunks;
//}
//
//pub struct CollisionSystem {}
//impl System for CollisionSystem {
//    fn run(&self, world: &mut World) {
//        let entities: Vec<Collisionable> = this_shouldnt_be_used(&*world)
//            .get_entities()
//            .into_iter()
//            .filter_map(|e| {
//
//                let collision_data = this_shouldnt_be_used(e.get_component_ref::<Collision>()?);
//                let position = e.get_component::<Position>()?.0;
//                let size = e.get_component::<Size>()?;
//
//                let sides_positions = SidesPositions::sides_pos_size(position, size);
//                return Some(Collisionable {
//                    collision_data,
//                    sides_positions,
//                    entitie: e
//                });
//            })
//            .collect();
//        collide_vec(entities, this_shouldnt_be_used(&*world));
//    }
//}
