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
    pub const MAYO: Element = Element::new("Mayo", "mayo", "A creamy spread made by whipping two eggs together. A classic addition for sammys!");
    pub const SALAD_TOPPING: Element = Element::new("Salad Topping", "salad_topping", "A mixture of the croutons and the ranch. If only you had something to put this on...");
    pub const SALAD: Element = Element::new("Salad", "salad", "A (sorta) healthy and tasty meal! Wilbur is sure to love this.");


    //Stage 4
    pub const SCRAMBLED_EGG : Element = Element::new("Scrambled Egg", "scrambled_egg", "Nice and fluffy egg, cooked an scrambled to perfect. Perfect for a hearty breakfast sandwich.");
    pub const RAW_PORK : Element = Element::new("Raw Pork", "raw_pork", "Meat straight from the pig! Did you know that english is one of the only languages that has different words for an animal and the meat it produces?");
    pub const RAW_BACON : Element = Element::new("Raw Bacon", "raw_bacon", "It's bacon! Of course we had to put it in the game, its bacon! Gotta cook it first though.");
    pub const BACON : Element = Element::new("Bacon", "bacon", "You baked some bacon! Yummy and greasy sweet meat that is such a treat.");
    pub const SPICY_SPREAD : Element = Element::new("Spicy Spread", "spicy_spread", "This is a nice spread made from mayo and pepper flakes, adds a pleasant kick to whatever you spread it on.");
    pub const SPICY_TOAST : Element = Element::new("Spicy Toast", "spicy_toast", "Toast with a spicy spread on it. You could probably eat this on your own and be happy, but you should add more...");
    pub const SANDWICH_FILLINGS : Element = Element::new("Sandwich Fillings", "sandwich_filling", "A mixture of egg and bacon that is by itself a hearty meal, however it could use something else...");
    pub const SANDWICH : Element = Element::new("Sandwich", "sandwich", "The culmination of your work for Sir. Conrad, however I think there maybe one more step before we can truly call it a sandwich.");
    pub const CUT_SANDWICH : Element = Element::new("Cut Sandwich", "cut_sandwich", "This is it! A beautiful sandwich ready to be eaton! Make sure Sir Conrad gets some.");

    //Stage 5
    pub const BOILING_WATER : Element = Element::new("Boiling Water", "boiling_water", "Alright now your cooking! Some");
    pub const BONE : Element = Element::new("Bone", "bone", "A bone you got while boiling the pork to make the broth. Wonder what you can do with this?");
    pub const BONE_CHOPSTICK : Element = Element::new("Bone Chopstick", "bone_chopstick", "Perfect for eating ramen with! Although it is missing a pair.");
    pub const BONE_CHOPSTICKS : Element = Element::new("Bone Chopsticks", "bone_chopsticks", "Now that they are paired up, It is read to use to eat something!");
    pub const PORK_BROTH : Element = Element::new("Pork Broth", "pork_broth", "The water turned into broth after boiling that pork. Now you have a delicious base for a soup of some kind.");
    pub const DRIED_SEAWEED : Element = Element::new("Dried Seaweed", "dired_seaweed", "The smell isn't great, but the flavor is great! Used in eastern style recipes.");
    pub const HARD_BOILED_EGG : Element = Element::new("Hard Boiled Egg", "hard_boiled_egg", "An egg that has been boiled. Amazing topping for any dish. Well not any dish but you get it.");
    pub const NOODLE_DOUGH : Element = Element::new("Noodle Dough", "noodle_dough", "Dough for making noodles! I wonder what to next...");
    pub const RAMEN_NOODLES : Element = Element::new("Ramen Noodles", "ramen_noodles", "Noodle perfectly made for the ultimate soup : Ramen. Just the right size and shape, this should make any ramen fan happy.");
    pub const COOKED_PORK : Element = Element::new("Cooked Pork", "cooked_pork", "Smells so good! This meat is probably the most tasty thing you have cooked so far.");
    pub const CHASHU : Element = Element::new("Chashu", "chashu", "Boiled and cut pork made in the traditional style. This meat is perfectly fatty and sweet for a stew of some sort.");

    //pub const GRIFFON_EGGS: Element = Element::new("Griffon Eggs", "griffon_eggs", "Eggs bigger than you head, serves five");

    // note update the number when new elements are created
    #[allow(dead_code)]
    pub const ELEMENTS: [Element; 39] = [
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
        Element::SALAD,
        Element::SCRAMBLED_EGG,
        Element::RAW_PORK,
        Element::RAW_BACON,
        Element::BACON,
        Element::SPICY_SPREAD,
        Element::SPICY_TOAST,
        Element::SANDWICH_FILLINGS,
        Element::SANDWICH,
        Element::CUT_SANDWICH,
        Element::BOILING_WATER,
        Element::BONE,
        Element::BONE_CHOPSTICK,
        Element::BONE_CHOPSTICKS,
        Element::PORK_BROTH,
        Element::DRIED_SEAWEED,
        Element::HARD_BOILED_EGG,
        Element::NOODLE_DOUGH,
        Element::RAMEN_NOODLES,
        Element::COOKED_PORK,
        Element::CHASHU
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