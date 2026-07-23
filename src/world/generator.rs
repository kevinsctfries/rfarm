use rand::RngExt;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

use super::map::Map;
use super::seed::Seed;

use super::terrain::river::River;

use super::roads::generator::RoadGenerator;

use super::parcel_generator::ParcelGenerator;
use super::region::LandRegion;

const MIN_WORLD_PARCELS: usize = 6;
const MAX_WORLD_PARCELS: usize = 8;

pub struct WorldGenerator;

impl WorldGenerator {
    pub fn generate(width: u32, height: u32, seed: Seed) -> Map {
        let mut rng = ChaCha8Rng::seed_from_u64(seed.0);

        // Generate terrain
        let river = River::generate(width, height, &mut rng);

        // Generate arterial roads
        let roads = RoadGenerator::generate(width, height, &mut rng);

        let mut map = Map::new(width, height);

        // Add world features
        // Priority determines rendering order.
        // Roads will overwrite rivers.
        map.add_feature(Box::new(river));

        let regions = LandRegion::generate(&map);

        map.add_feature(Box::new(roads));

        // Total farms in the entire world
        let mut remaining_parcels = rng.random_range(MIN_WORLD_PARCELS..=MAX_WORLD_PARCELS);

        let mut next_parcel_id = 0;

        for region in &regions {
            if remaining_parcels == 0 {
                break;
            }

            // Give this region some farms
            let region_parcels = remaining_parcels.min(rng.random_range(1..=remaining_parcels));

            let parcels =
                ParcelGenerator::generate(region, &mut rng, &mut next_parcel_id, region_parcels);

            remaining_parcels = remaining_parcels.saturating_sub(parcels.len());

            for parcel in parcels {
                map.add_parcel(parcel);
            }
        }

        map
    }
}
