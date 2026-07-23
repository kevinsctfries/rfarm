use super::farm_names::FarmNameGenerator;
use super::geometry::point::Point;
use super::land_parcel::LandParcel;
use super::region::LandRegion;

use rand::RngExt;
use std::collections::{HashMap, HashSet};

const MIN_PARCEL_WIDTH: i32 = 10;
const MIN_PARCEL_HEIGHT: i32 = 10;

const RELAXATION_STEPS: usize = 5;

pub struct ParcelGenerator;

impl ParcelGenerator {
    pub fn generate(
        region: &LandRegion,
        rng: &mut impl RngExt,
        next_id: &mut u64,
        parcel_count: usize,
        farm_names: &FarmNameGenerator,
    ) -> Vec<LandParcel> {
        let mut parcels = Vec::new();

        if region.tiles.is_empty() {
            return parcels;
        }

        let mut candidates = region.tiles.clone();

        let mut seeds = Vec::new();

        let target_count = parcel_count.min(candidates.len());

        for _ in 0..target_count {
            let index = rng.random_range(0..candidates.len());

            seeds.push(candidates.swap_remove(index));
        }

        // Lloyd relaxation
        for _ in 0..RELAXATION_STEPS {
            let assignments = Self::assign_tiles(&region.tiles, &seeds);

            for (index, seed) in seeds.iter_mut().enumerate() {
                let Some(tiles) = assignments.get(&index) else {
                    continue;
                };

                if tiles.is_empty() {
                    continue;
                }

                let center = Point {
                    x: tiles.iter().map(|p| p.x).sum::<i32>() / tiles.len() as i32,
                    y: tiles.iter().map(|p| p.y).sum::<i32>() / tiles.len() as i32,
                };

                if region.tiles.contains(&center) {
                    *seed = center;
                }
            }
        }

        // Voronoi assignment
        let assignments = Self::assign_tiles(&region.tiles, &seeds);

        let mut raw_parcels: Vec<Vec<Point>> = assignments.into_values().collect();

        Self::perturb_borders(&mut raw_parcels, rng);

        // Merge invalid parcels
        loop {
            let invalid = raw_parcels
                .iter()
                .enumerate()
                .find(|(_, tiles)| !Self::is_large_enough(tiles));

            let Some((index, _)) = invalid else {
                break;
            };

            let neighbors = Self::find_bordering_parcels(index, &raw_parcels);

            let target = neighbors
                .into_iter()
                .filter(|i| !raw_parcels[*i].is_empty())
                .max_by_key(|i| raw_parcels[*i].len());

            match target {
                Some(target) => {
                    let tiles = std::mem::take(&mut raw_parcels[index]);

                    raw_parcels[target].extend(tiles);
                }

                None => break,
            }
        }

        // Create parcels
        for tiles in raw_parcels {
            if tiles.is_empty() {
                continue;
            }

            let id = *next_id;

            *next_id += 1;

            let tile_set: HashSet<Point> = tiles.into_iter().collect();

            let parcel_seed = rng.random::<u64>();

            // Generate names until one can be placed.
            loop {
                let farm_name = farm_names.generate(rng);

                let parcel = LandParcel::new(id, tile_set.clone(), farm_name, parcel_seed);

                if parcel.label_position.is_some() {
                    parcels.push(parcel);
                    break;
                }
            }
        }

        parcels
    }

    // Assign tiles to nearest seed
    fn assign_tiles(tiles: &[Point], seeds: &[Point]) -> HashMap<usize, Vec<Point>> {
        let mut assignments = HashMap::new();

        for tile in tiles {
            let mut closest = 0;

            let mut closest_distance = i32::MAX;

            for (index, seed) in seeds.iter().enumerate() {
                let dx = tile.x - seed.x;
                let dy = tile.y - seed.y;

                let distance = dx * dx + dy * dy;

                if distance < closest_distance {
                    closest_distance = distance;
                    closest = index;
                }
            }

            assignments
                .entry(closest)
                .or_insert_with(Vec::new)
                .push(*tile);
        }

        assignments
    }

    // Slightly alter borders without deleting land
    fn perturb_borders(parcels: &mut Vec<Vec<Point>>, rng: &mut impl RngExt) {
        let mut ownership = HashMap::<Point, usize>::new();

        for (index, parcel) in parcels.iter().enumerate() {
            for tile in parcel {
                ownership.insert(*tile, index);
            }
        }

        let mut moves = Vec::<(usize, usize, Point)>::new();

        for (index, parcel) in parcels.iter().enumerate() {
            for tile in parcel {
                for neighbor in tile.orthogonal_neighbors() {
                    let Some(owner) = ownership.get(&neighbor) else {
                        continue;
                    };

                    if *owner != index && rng.random_range(0..100) < 5 {
                        moves.push((index, *owner, *tile));
                    }
                }
            }
        }

        for (from, to, tile) in moves {
            if let Some(position) = parcels[from].iter().position(|x| *x == tile) {
                parcels[from].remove(position);
                parcels[to].push(tile);
            }
        }
    }

    fn find_bordering_parcels(parcel_index: usize, parcels: &[Vec<Point>]) -> Vec<usize> {
        let mut neighbors = HashSet::new();

        for tile in &parcels[parcel_index] {
            let adjacent = tile.orthogonal_neighbors();

            for (index, other) in parcels.iter().enumerate() {
                if index == parcel_index {
                    continue;
                }

                if other.iter().any(|p| adjacent.contains(p)) {
                    neighbors.insert(index);
                }
            }
        }

        neighbors.into_iter().collect()
    }

    fn is_large_enough(tiles: &[Point]) -> bool {
        if tiles.is_empty() {
            return false;
        }

        let min_x = tiles.iter().map(|p| p.x).min().unwrap();
        let max_x = tiles.iter().map(|p| p.x).max().unwrap();

        let min_y = tiles.iter().map(|p| p.y).min().unwrap();
        let max_y = tiles.iter().map(|p| p.y).max().unwrap();

        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        width >= MIN_PARCEL_WIDTH && height >= MIN_PARCEL_HEIGHT
    }
}
