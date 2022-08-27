#[derive(Eq, PartialEq, Debug, Clone, Default, Hash)]
pub struct Element {
    pub name: &'static str,
    pub id: &'static str,
    pub desc: &'static str,
}

impl Element {
    #[allow(dead_code)]
    pub const ELEMENT_PATH: &'static str = "sprites/";

    //Stage 1
    pub const FROZEN_DRAGON_SCALE: Element = Element::new("Frost Scale", "frost_dragon_scale", "A scale from a dragon that is cold to the touch. It will freeze you if you arent careful.");
    pub const YETI_WATER: Element = Element::new("Yeti Water", "yeti_water", "A hydrating liquid with a strange stench. You think this is just normal water, but something is off... ");
    pub const GLACIER_ICE: Element = Element::new("Glacier Ice", "glacier_ice", "Your tongue is drawn to the frosty surface...");
    pub const LEGEND_DAIRY: Element = Element::new("Legend Dairy", "legend_dairy", "Utterly Delicious. Legend speaks of the cow from which this heavenly cream comes from");
    pub const SHAVED_ICE: Element = Element::new("Shaved Ice", "shaved_ice", "The most clean shaven ice youve ever seen. To bad you cant shave yourself like that.");
    pub const UTTER_ICE_CREAM: Element = Element::new("Utter Ice Cream", "utter_ice_cream", "Utterly delicious ice cream that comes in a cute little cone! No, I dont know where I found the cone.");

    //Stage 2
    pub const FANTASY_FLOUR: Element = Element::new("Fantasy Flour", "fantasy_flour", "Flour, but fantasy flavored. Pun intended. This can be used to make everything from bread to pasta.");
    pub const MAGMA_PEPPER: Element = Element::new("Magma Pepper", "magma_pepper", "Really, really, REALLY hot. These are grown on the rim of an active volcano. Used for dragon kibble.");
    pub const BREAD_DOUGH : Element = Element::new("Bread Dough", "bread_dough", "Bread dough that is slightly sticky. You dont know how it rose so fast, best not think about it to hard.");
    pub const ELVEN_BREAD : Element = Element::new("Elven Bread", "elven_bread", "Bread that is cooked in the traditional elven way. With a pepper as heat. Tastes good!");
    pub const PEPPER_FLAKES : Element = Element::new("Pepper Flakes", "pepper_flakes", "Hot flakes that sizzle when you touch them. This will make any dish spicy.");
    pub const ICE_CREAM_SANDWICH : Element = Element::new("Ice Cream Sandwich", "ice_cream_sandwich", "Though normal bread isnt typically used, is it fairly yummy.");

    //Stage 3
    pub const ELVEN_TOAST: Element = Element::new("Elven Toast", "elven_toast", "Finest toast with the confines of this tavern. Its crazy how many things are just better when cooked.");
    pub const GRIFFON_EGG: Element = Element::new("Griffon Egg", "griffon_egg", "An egg as big as your head! Serves 5. Taken from a griffons nest not to long ago.");
    pub const SIREN_SEAWEED: Element = Element::new("Siren Seaweed", "siren_seaweed", "The name is misleading, this isnt seaweed harvested from sirens. No, just around where they are. Cant help but like em!");
    pub const DICED_CROUTONS: Element = Element::new("Diced Croutons", "diced_croutons", "Youre on a roll! After these croutons youll be in para-dice.");
    pub const RANCH: Element = Element::new("Ranch", "ranch", "Ranch ironically made off of a ranch. Creamy and delicious, why not smother it of everything?");
    pub const SALAD_TOPPING: Element = Element::new("Salad Topping", "salad_topping", "A mixture of the croutons and the ranch. If only you had something to put this on...");
    pub const SALAD: Element = Element::new("Salad", "salad", "A (sorta) healthy and tasty meal! Wilbur is sure to love this.");

    //pub const GRIFFON_EGGS: Element = Element::new("Griffon Eggs", "griffon_eggs", "Eggs bigger than you head, serves five");

    // note update the number when new elements are created
    #[allow(dead_code)]
    pub const ELEMENTS: [Element; 19] = [
        Element::FROZEN_DRAGON_SCALE,
        Element::YETI_WATER,
        Element::GLACIER_ICE,
        Element::LEGEND_DAIRY,
        Element::SHAVED_ICE,
        Element::UTTER_ICE_CREAM,
        Element::FANTASY_FLOUR,
        Element::MAGMA_PEPPER,
        Element::BREAD_DOUGH,
        Element::ELVEN_BREAD,
        Element::PEPPER_FLAKES,
        Element::ICE_CREAM_SANDWICH,
        Element::ELVEN_TOAST,
        Element::GRIFFON_EGG,
        Element::SIREN_SEAWEED,
        Element::DICED_CROUTONS,
        Element::RANCH,
        Element::SALAD_TOPPING,
        Element::SALAD
    ];

    pub const fn new(name: &'static str, id: &'static str, desc: &'static str) -> Self {
        Self {
            name,
            id,
            desc,
        }
    }

    pub fn sprite_file_path(&self) -> String {
        let path: String = format!("{}{}.png", Element::ELEMENT_PATH, self.id);
        path
    }
}