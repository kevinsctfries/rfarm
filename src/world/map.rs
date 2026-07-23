use super::feature::Feature;
use super::geometry::point::Point;
use super::land_parcel::LandParcel;

pub struct Map {
    pub width: u32,
    pub height: u32,

    pub features: Vec<Box<dyn Feature>>,

    pub parcels: Vec<LandParcel>,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Map {
        Map {
            width,
            height,
            features: Vec::new(),
            parcels: Vec::new(),
        }
    }

    pub fn add_feature(&mut self, feature: Box<dyn Feature>) {
        self.features.push(feature);
    }

    pub fn add_parcel(&mut self, parcel: LandParcel) {
        self.parcels.push(parcel);
    }

    pub fn is_occupied(&self, point: Point) -> bool {
        for feature in &self.features {
            if feature.contains(point) {
                return true;
            }
        }

        false
    }

    pub fn parcel_at(&self, point: Point) -> Option<u64> {
        for parcel in &self.parcels {
            if parcel.contains(point) {
                return Some(parcel.id);
            }
        }

        None
    }
}
