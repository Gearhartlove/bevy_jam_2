use crate::element::Element;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SlicerRecipe {
    pub object : Element,
    pub result : Element,
}

impl SlicerRecipe {
    const SHAVED_ICE: SlicerRecipe = SlicerRecipe::new(Element::GLACIER_ICE, Element::SHAVED_ICE);
    const PEPPER_FLAKES: SlicerRecipe = SlicerRecipe::new(Element::MAGMA_PEPPER, Element::PEPPER_FLAKES);
    const DICED_CROUTONS: SlicerRecipe = SlicerRecipe::new(Element::ELVEN_TOAST, Element::DICED_CROUTONS);
    const CUT_SANDWICH: SlicerRecipe = SlicerRecipe::new(Element::SANDWICH, Element::CUT_SANDWICH);
    const BONE_CHOPSTICK: SlicerRecipe = SlicerRecipe::new(Element::BONE, Element::BONE_CHOPSTICK);
    const RAW_BACON: SlicerRecipe = SlicerRecipe::new(Element::RAW_PORK, Element::RAW_BACON);
    const CHASHU: SlicerRecipe = SlicerRecipe::new(Element::COOKED_PORK, Element::CHASHU);
    const RAMEN_NOODLES: SlicerRecipe = SlicerRecipe::new(Element::NOODLE_DOUGH, Element::RAMEN_NOODLES);

    pub const RECIPES: [SlicerRecipe; 8] = [
        SlicerRecipe::SHAVED_ICE,
        SlicerRecipe::PEPPER_FLAKES,
        SlicerRecipe::DICED_CROUTONS,
        SlicerRecipe::CUT_SANDWICH,
        SlicerRecipe::BONE_CHOPSTICK,
        SlicerRecipe::RAW_BACON,
        SlicerRecipe::CHASHU,
        SlicerRecipe::RAMEN_NOODLES
    ];

    pub const fn new(object: Element, result: Element) -> Self {
        Self {
            object,
            result,
        }
    }

    pub fn id(&self) -> String {
        let id = format!("{}_{}", self.object.id, self.result.id);
        return id;
    }
}