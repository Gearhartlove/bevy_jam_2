#[derive(Eq, PartialEq, Debug, Clone, Default, Hash)]
pub struct Element {
    pub name: &'static str,
    pub id: &'static str,
    pub desc: &'static str,
}

impl Element {
    #[allow(dead_code)]
    pub const ELEMENT_PATH: &'static str = "sprites/elements/";

    pub const FROST_BOTTLE: Element = Element::new("Frost Bottle", "frost_bottle", "Cold to the touch");
    pub const YETI_WATER: Element = Element::new("Yeti Water", "yeti_water", "A hydrating liquid with a strange stench");
    pub const GLACIER_ICE: Element = Element::new("Glacier Ice", "glacier_ice", "Your tongue is drawn to the frosty surface");
    pub const LEGEND_DAIRY: Element = Element::new("Legend Dairy", "legend_dairy", "Utterly Delicious. Legend speaks of the cow from which this heavenly cream comes from");
    pub const SHAVED_ICE: Element = Element::new("Shaved Ice", "shaved_ice", "The most clean shaven ice you've ever seen");
    pub const UTTER_ICE_CREAM: Element = Element::new("Utter Ice Cream", "utter_ice_cream", "Utterly delicious");

    // note update the number when new elements are created
    #[allow(dead_code)]
    pub const ELEMENTS: [Element; 5] = [
        Element::FROST_BOTTLE,
        Element::YETI_WATER,
        Element::GLACIER_ICE,
        Element::LEGEND_DAIRY,
        Element::SHAVED_ICE,
    ];

    pub const fn new(name: &'static str, id: &'static str, desc: &'static str) -> Self {
        Self {
            name,
            id,
            desc,
        }
    }

    pub fn path(&self) -> String {
        let path: String = format!("{}{}.png", Element::ELEMENT_PATH, self.id);
        path
    }
}