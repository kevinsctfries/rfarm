use rand::RngExt;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::farm_names::FarmNameGenerator;
use super::map::Map;
use super::seed::Seed;

use super::roads::generator::RoadGenerator;
use super::terrain::river::River;

use super::parcel_generator::ParcelGenerator;
use super::region::LandRegion;
use super::vehicles::generator::VehicleGenerator;

const MIN_WORLD_PARCELS: usize = 6;
const MAX_WORLD_PARCELS: usize = 8;

pub struct WorldGenerator;

impl WorldGenerator {
    pub fn generate(width: u32, height: u32, seed: Seed) -> Map {
        let mut rng = ChaCha8Rng::seed_from_u64(seed.0);

        // Terrain first
        let river = River::generate(width, height, &mut rng);

        // Roads need terrain information
        let roads = RoadGenerator::generate(width, height, &river, &mut rng);

        // Vehicles need roads
        let vehicles = VehicleGenerator::generate(&roads, 5, &mut rng);

        let mut map = Map::new(width, height, seed.0);

        // Feature priority:
        // Roads overwrite rivers visually.
        map.add_feature(Box::new(river));

        let regions = LandRegion::generate(&map);

        map.add_feature(Box::new(roads));

        map.vehicles = vehicles;

        // Generate farms
        let farm_names = FarmNameGenerator::load();

        let mut remaining_parcels = rng.random_range(MIN_WORLD_PARCELS..=MAX_WORLD_PARCELS);

        let mut next_parcel_id = 0;

        for region in &regions {
            if remaining_parcels == 0 {
                break;
            }

            let region_parcels = remaining_parcels.min(rng.random_range(1..=remaining_parcels));

            let parcels = ParcelGenerator::generate(
                region,
                &mut rng,
                &mut next_parcel_id,
                region_parcels,
                &farm_names,
            );

            remaining_parcels = remaining_parcels.saturating_sub(parcels.len());

            for parcel in parcels {
                map.add_parcel(parcel);
            }
        }

        map
    }
}
