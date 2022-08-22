use crate::element::Element;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct SlicerRecipe {
    pub object : Element,
    pub result : Element,
}

impl SlicerRecipe {
    const SHAVED_ICE: SlicerRecipe = SlicerRecipe::new(Element::GLACIER_ICE, Element::SHAVED_ICE);

    pub const RECIPES: [SlicerRecipe; 1] = [
        SlicerRecipe::SHAVED_ICE,
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