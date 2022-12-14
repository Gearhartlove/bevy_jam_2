use std::collections::VecDeque;
use bevy::prelude::*;
use bevy::text::Text2dBounds;
use bevy::utils::HashMap;
use bevy::utils::tracing::event;
use crate::boss_fight::{SetupBossFightEvent, ToggleBossTimerEvent, WinGameEvent};
use crate::audio::{SayEvent, MusicTrack, MusicChangeEvent};
use crate::element::Element;
use crate::game::GameManager;
use crate::npc::{Npc, NpcClickEvent, NPCDropEvent, NpcKind, NpcSprite, NpcText, Say};
use crate::ui::{CraftType, ElementCraftedEvent, InsertElementEvent, LoadFurnaceEvent, LoadMixerEvent, LoadSlicerEvent, NPC_LEVEL};

pub struct GameflowPlugin;

impl Plugin for GameflowPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Gameflow>()
            .init_resource::<GameManager>()
            //.add_startup_system(start_gameflow)
            .add_system_to_stage(CoreStage::PostUpdate, update_gameflow);
    }
}

//==================================================================================================
//                  GameFlow
//==================================================================================================

pub struct Gameflow {
    segments: Vec<Box<dyn Segment + Send + Sync>>,
    current: u32,
    last : u32
}

impl Gameflow {
    pub fn add_segment<T>(&mut self, segment: T) -> &mut Self where T: Segment + Send + Sync + 'static {
        self.segments.push(Box::new(segment));
        return self;
    }

    pub fn advance(&mut self) {
        self.current += 1;
    }
}

pub struct EventCaller {
    pub insert_element_event : Option<InsertElementEvent>,
    pub load_mixer_event : Option<LoadMixerEvent>,
    pub load_slicer_event : Option<LoadSlicerEvent>,
    pub load_furnace_event : Option<LoadFurnaceEvent>,
    pub say_event: Option<SayEvent>,
    pub setup_boss_event : Option<SetupBossFightEvent>,
    pub toggle_timer_event : Option<ToggleBossTimerEvent>,
    pub win_game_event : Option<WinGameEvent>,
    pub music_change_event: Option<MusicChangeEvent>,
}

impl Default for EventCaller {
    fn default() -> Self {
        EventCaller {
            insert_element_event : None,
            say_event: None,
            load_mixer_event : None,
            load_slicer_event : None,
            load_furnace_event: None,
            setup_boss_event : None,
            toggle_timer_event : None,
            win_game_event : None,
            music_change_event: None,
        }
    }
}

fn start_gameflow(
    mut gameflow: ResMut<Gameflow>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameManager>,
) {
    let i = gameflow.current as usize;
    let current = gameflow.segments.get_mut(i).unwrap();
    current.on_segment_start(&mut commands, &asset_server, &mut game, &mut EventCaller::default());
}

fn update_gameflow(
    mut gameflow: ResMut<Gameflow>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<GameManager>,

    //Events Listeners
    mut on_npc_click: EventReader<NpcClickEvent>,
    mut on_item_craft: EventReader<ElementCraftedEvent>,
    mut on_npc_drop : EventReader<NPCDropEvent>,

    //Event Writers
    mut insert_element_event_writer: EventWriter<InsertElementEvent>,
    mut load_furnace_event_writer : EventWriter<LoadFurnaceEvent>,
    mut load_mixer_event_writer : EventWriter<LoadMixerEvent>,
    mut load_slicer_event_writer : EventWriter<LoadSlicerEvent>,
    mut say_event_writer : EventWriter<SayEvent>,
    mut setup_boss_fight : EventWriter<SetupBossFightEvent>,
    mut toggle_boss_timer_event : EventWriter<ToggleBossTimerEvent>,
    mut win_game_event : EventWriter<WinGameEvent>,
    mut music_event_writer: EventWriter<MusicChangeEvent>,
) {

    //println!("{} | {}", gameflow.current, gameflow.segments.len());
    let mut event_caller = EventCaller::default();

    let mut should_init = false;

    if gameflow.current != gameflow.last {
        gameflow.last = gameflow.current;
        should_init = true;
    }

    let i = gameflow.current as usize;
    if let Some(mut current) = gameflow.segments.get_mut(i) {
        if should_init {
            current.on_segment_start(&mut commands, &asset_server, &mut game, &mut event_caller)
        }

        for event in on_npc_click.iter() {
            current.on_npc_click(&mut commands, &asset_server, &mut game, &mut event_caller);
        }

        for event in on_item_craft.iter() {
            current.on_item_crafted(&mut commands, &asset_server, &mut game, &mut event_caller, event.0.clone());
        }

        for event in on_npc_drop.iter() {
            current.on_npc_drop(&mut commands, &asset_server, &mut game, &mut event_caller, event.0.clone())
        }

        if current.is_complete() {
            current.on_segment_end(&mut commands, &asset_server, &mut game, &mut event_caller);
            gameflow.advance();
        }
    }

    if let Some(event) = event_caller.insert_element_event {
        insert_element_event_writer.send(event)
    }

    if let Some(event) = event_caller.load_furnace_event {
        load_furnace_event_writer.send(event)
    }

    if let Some(event) = event_caller.load_mixer_event {
        load_mixer_event_writer.send(event)
    }

    if let Some(event) = event_caller.load_slicer_event {
        load_slicer_event_writer.send(event)
    }

    if let Some(event) = event_caller.say_event {
        say_event_writer.send(event)
    }

    if let Some(event) = event_caller.setup_boss_event {
        setup_boss_fight.send(event)
    }

    if let Some(event) = event_caller.toggle_timer_event {
        toggle_boss_timer_event.send(event)
    }

    if let Some(event) = event_caller.music_change_event {
        music_event_writer.send(event)
    }
}

impl Default for Gameflow {
    fn default() -> Self {
        let mut game_flow = Gameflow {
            segments: vec![],
            current: 0,
            last: u32::MAX
        };

        game_flow
            // chapter 1
            .add_segment(NpcDialogueSegment::new()
                .with_line("Barkeep! Over here! Click on me to talk to me!")
                .with_line("Hey! Who are you? You arent the usual chef! Where is Gyome?")
                .with_line("Oh, my boss Gordon will not be pleased, not pleased at all!")
                .with_line("Good thing I came by to check first, he would have sauteed you with dung fruit!")
                .with_line("Do you even know how to cook? It doesnt look like it...")
                .with_line("Ill teach you how, just so gordon doesnt go ballistic.")
                .with_line("Lets try to make something simple.. something like ice cream!")
            )

            .add_segment(GiveElementSegment::new(Element::YETI_WATER))
            .add_segment(GiveElementSegment::new(Element::FROZEN_DRAGON_SCALE)
                .with_line("Take these. Youll need them.")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("To see what an item is, you can mouse over it. Right clicking will show its page in the fantastical cook book.")
                .with_line("If you ever forget a recipe, I would check there.")
                .with_line("Lets see, first thing you need for ice cream is, well, ice.")
            )

            .add_segment(LoadToolSegment::new(CraftType::FURNACE))

            .add_segment(CraftingSegment::new(Element::GLACIER_ICE.clone(), true)
                .with_hint("Go ahead and try to make ice! If you click on me I will give hints.")
                .with_hint("You can drag items around and put them into the tools in the middle.")
                .with_hint("Youll want to use the furnace for this. If you put something cold in the bottom slot, the item on top will freeze!")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("Wow you did it. Maybe you will taste His Wrath. That is his specialty dish.")
                .with_line("Now we need to shave that ice into smaller pieces. You have a knife dont you?")
            )

            .add_segment(LoadToolSegment::new(CraftType::SLICER)
                .with_line("Oh, its over there.")
            )

            .add_segment(CraftingSegment::new(Element::SHAVED_ICE.clone(), true)
                .with_hint("Alright, go a head and make some shaved ice.")
                .with_hint("You shouldnt need a hint for this one.")
                .with_hint("Really?")
                .with_hint("Fine. Put the ice on the cutting board.")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("Cool. now the last step, you need to mix the shaved ice with some cream.")
            )

            .add_segment(GiveElementSegment::new(Element::LEGEND_DAIRY)
                .with_line("Here is the cream.")
            )

            .add_segment(LoadToolSegment::new(CraftType::MIXER)
                .with_line("And here is the mixer.")
            )

            .add_segment(CraftingSegment::new(Element::UTTER_ICE_CREAM.clone(), false)
                .with_hint("Now make that ice cream.")
                .with_hint("It takes two ingredients.")
                .with_hint("You also need to use the mixing bowl.")
                .with_hint("Put the shaved ice and legend dairy into the mixing bowl.")
                .with_comment(&Element::UTTER_ICE_CREAM, "Nice. Now give that to me!")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("You did it. Now that you kinda know how to cook, hopefully you can make gordon something that he likes.")
                .with_line("If he doesnt, boy I am done for. The last guy that was in my shoes got cooked into a real nice roast.")
                .with_line("Honestly, not a bad way to go.")
                .with_line("Anyways, I have to go and check the other places Gordon is going to today. But before I go, Im going to give you some ingredients that you may need.")
            )

            .add_segment(GiveElementSegment::new(Element::MAGMA_PEPPER)
                .with_line("Take this to heat your dishes.")
            )

            .add_segment(GiveElementSegment::new(Element::FANTASY_FLOUR)
                .with_line("And this because every kitchen needs some.")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("Now I gotta run! If I dont I might not make it.")
            )

            .add_segment(TransitionSegment::new(
              vec![
                 "Good luck... you will need it...".to_string(),
                 "And remember that book of yours, you can never know too much about the ingredients you cook with.".to_string()
              ],
                vec![
                    "Hello! My name is Sir Connrad and I am in desperate need of adventuring food.".to_string()
                ]
            ))

            //Stage 2
            .add_segment(NpcDialogueSegment::new()
                .with_line("Ho there barkeep! I am in dire need of provisioning")
                .with_line("As a knight of this realm, I must see to my duties outside of the city.")
                .with_line("And my duties today take me to the Dunes of Teveldia, to hunt the witches that lives there.")
                .with_line("But to do this quest I must travel. Teveldia is far, far away ..")
                .with_line("... one whole hour away ... ")
                .with_line("And because of that, Ill need some food that I can bring with me on my journey!")
                .with_line("Now, what better to hunt sand witches with than sandwiches!")
                .with_line("That is what I am here for! One of your best sandwiches!")
            )

            .add_segment(CraftingSegment::new(Element::ICE_CREAM_SANDWICH, false)
                .with_hint("So please make me a sandwich of some sort!")
                .with_hint("My favorite part of any sandwich is the bread. Good bread is necessary for a good sandwich.")
                .with_hint("Ill take any type of sandwich, really!")
                .with_comment(&Element::ELVEN_BREAD, "Yes! Any good sandwich needs some bread!")
                .with_comment(&Element::BREAD_DOUGH, "A step in the right direction! You could be a knight yourself with intuition like that!")
                .with_comment(&Element::ICE_CREAM_SANDWICH, "That is what I require! Hand it over at once!")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("Ah yes! A sandwich! Thank you good fellow, I will eat be hearty knowing that your skill in cook craft is paramount!")
            )

            .add_segment(GiveElementSegment::new(Element::GRIFFON_EGG)
                .with_line("As payment, please accept this egg. It will lend you aid in these trying times.")
            )

            .add_segment(TransitionSegment::new(
                vec![
                    "Huzuh! I am off, for glory!".to_string()
                ],
                vec![
                    "... Hi, I would like some food ...".to_string()
                ]
            ))

            //Stage 3
            .add_segment(NpcDialogueSegment::new()
                .with_line("... I took my time coming to order ... that last guy was loud ... ")
                .with_line("... My name is Wilbur, but everybody calls me pumpkin head. I am a pig farmer from around here ... ")
                .with_line("... please dont ask about the pumpkin, itll make me shy ... ")
            )

            .add_segment(GiveElementSegment::new(Element::SIREN_SEAWEED)
                .with_line("... I would like a salad with this seaweed ... ")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("... I would also like it with some toppings ... ")
                .with_line("... something creamy and something crunchy ... ")
            )

            .add_segment(CraftingSegment::new(Element::SALAD.clone(), false)
                .with_hint("... Could you please make me one now? ...")
                .with_hint("... a salad with a creamy and crunchy topping ...")
                .with_hint("... I want those crunchy things ... crawdads ... cruonans ... ahh, something like that ...")
                .with_hint("... could you please hurry? I need to get back to the RANCH ...")
                .with_hint("... I like the toppings mixed together ...")
                .with_comment(&Element::MAYO, "... that seems creamy, but to solid for a salad ...")
                .with_comment(&Element::ELVEN_TOAST, "... that smells good, but that wont fit on my salad ...")
                .with_comment(&Element::DICED_CROUTONS, "... those would add the most perfect crunch to my salad ...")
                .with_comment(&Element::RANCH, "... that seems yummy ... perfect for my salad ...")
                .with_comment(&Element::SALAD, "... oh wow that looks so good ... can I please have it?")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("... thanks ... this salad looks really good ... ")
                .with_line("... I am going to go home now ... I have been in public for far too long ...")
            )

            .add_segment(GiveElementSegment::new(Element::RAW_PORK)
                .with_line("... here is something from my pig farm as payment ...")
            )

            .add_segment(TransitionSegment::new(
                vec![
                    "... enjoy yourself ...".to_string()
                ],
                vec![
                    "Huzuh! I am back from the fray!".to_string()
                ]
            ))

            //Stage 4
            .add_segment(NpcDialogueSegment::new()
                .with_line("Though it is earlier than expected, I am back none the less!")
                .with_line("You see, the last sandwich you gave me started to melt as soon as I left the city gates.")
                .with_line("These provisions must stay solid until I make it to the dunes. This was a problem you see.")
                .with_line("But this problem was no match for the valiant Sir Conrad! I turn problems into mincemeat!")
                .with_line("My solution being thus ... eat the sandwich given prier and come back for another, more substantial morsel.")
                .with_line("So my request is as follows ... I would like another sandwich. This one I want to be more meaty.")
                .with_line("Specifically I would like a breakfast sandwich with a spicy spread.")
            )

            .add_segment(CraftingSegment::new(Element::CUT_SANDWICH.clone(), false)
                .with_hint("I am in need of a breakfast sandwich with a little bit of heat.")
                .with_hint("Mhhmm. That cut of pork looks mighty delicious!")
                .with_comment(&Element::PEPPER_FLAKES, "Yes not the whole pepper, just a bit of it. However, I still think those flakes are going to be hard to sallow...")
                .with_comment(&Element::SCRAMBLED_EGG, "What a good filling for a breakfast sandwich! I think it is missing a protein though.")
                .with_comment(&Element::RAW_BACON, "Now that looks intriguing! Sliced pork? How novel.")
                .with_comment(&Element::BACON, "Listen to that sizzle, music to my ears and ambrosia for my nose!")
                .with_comment(&Element::SANDWICH_FILLINGS, "The perfect mix of filling fillings I have ever seen.")
                .with_comment(&Element::SPICY_SPREAD, "That will be the perfect amount of heat! Put it on the sandwich!")
                .with_comment(&Element::SPICY_TOAST, "Now all that needs is the filling!")
                .with_comment(&Element::SANDWICH, "That is a legendary sandwich, but you need to do one more thing to make it perfect...")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("There it is! The breakfast sandwich I have been dreaming of!")
                .with_line("Thank you fine citizen, for without your help my quest would be a wash.")
                .with_line("Take head of your skill, for you deserve the recognition!")
            )

            .add_segment(TransitionSegment::new(
                vec![
                    "Huzuh and good morrow my fiend of food. Huzuh!".to_string()
                ],
                vec![
                    "So, you are the one that will be cooking for me tonight?".to_string()
                ]
            ))

            .add_segment(MusicChangeSegment::change_too(MusicTrack::Boss))


            .add_segment(NpcDialogueSegment::new()
                .with_line("I was expecting gyome but I guess I cant expect for a perfect meal every night.")
                .with_line("Well, If I said that you were my first disappointment of the night I would be lying.")
                .with_line("On the way here I saw a pedestrian eating Weef Bellington with ketchup. KETCHUP!!")
                .with_line("That is a sin worse than war, murder, and out of taste clothing. I can tell based on your wardrobe that you are a sinner.")
                .with_line("Where is your menu? Or is the only you serve here bad service and the black plague.")
                .with_line("Disgusting. I hope that your food comes cooked and not BLOODY RAW!")
                .with_line("Hmph. Now, I think I have berated you enough to work up an appetite. I will order my food now.")
                .with_line("I want you to make your best ramen. A true mark of any great chef is to incorporate foreign dished into your repertoire.")
                .with_line("I will see your skill by eating a dish that is uncommon. Also there is a twist.")
            )

            .add_segment(StartBossFightSegment)

            .add_segment(NpcDialogueSegment::new()
                .with_line("I will time you. You have 10 minutes to complete the dish.")
                .with_line("This ramen must be made with eight ingredients only. I will give you a description of what I want now.")
                .with_line("Listen close, because I wont repeat myself.")
                .with_line("Seriously, I WONT repeat myself.")
                .with_line("I want a spicy ramen with a delicious broth and traditionally cooked meat.")
                .with_line("The noodles better be made from scratch, I will be able to tell. Garnish with a green and a hearty ingredient.")
                .with_line("Once bowled, I want it to be served with utensils and an ice cube to cool it down.")
                .with_line("Do you understand? Good. Your time starts...")
            )

            .add_segment(ToggleBossTimerSegment)

            .add_segment(NpcDialogueSegment::new()
                .with_line("NOW!!!!")
            )

            .add_segment(CraftingSegment::new(Element::RAMEN, true)
                .with_hint("Make the ramen! What what are you waiting for?")
                .with_hint("I told you I wouldn't repeat myself.")
                .with_comment(&Element::PORK_BROTH, "Wow. Good use of your ingredients.")
                .with_comment(&Element::BONE_CHOPSTICK, "I see what you are doing there. Very smart.")
                .with_comment(&Element::BONE_CHOPSTICKS, "Perfect. I can eat with those.")
                .with_comment(&Element::DRIED_SEAWEED, "Yes, that will do nicely.")
                .with_comment(&Element::CHASHU, "Perfectly cooked and cut. A man class after all.")
                .with_comment(&Element::NOODLE_DOUGH, "Ah, interesting.")
                .with_comment(&Element::RAMEN_NOODLES, "That is a nice cut of noodles.")
                .with_comment(&Element::BOILING_WATER, "Interesting...")
                .with_comment(&Element::HARD_BOILED_EGG, "Perfect addition to my ramen.")
            )

            .add_segment(NpcDialogueSegment::new()
                .with_line("Wow... this... is... actually good.")
                .with_line("I dont say that often, but this ramen is actually very good.")
                .with_line("Full credits. This is a good dish. Well done!")
            )

            //.add_segment(MusicChangeSegment::change_too(MusicTrack::Background))
            .add_segment(WinGameSegment)
        ;

        return game_flow;
    }
}

// ########################################################################

pub trait Segment {
    fn is_complete(&self) -> bool;

    fn on_item_crafted(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller,
        element : Element
    ) {}

    fn on_npc_click(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {}

    fn on_npc_drop(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller,
        element : Element
    ) {}

    fn on_segment_start(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {}

    fn on_segment_end(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {}
}

// ################################################################################################################################################
// SqueeHelloPlayer
// ################################################################################################################################################

struct NpcDialogueSegment {
    phrases: VecDeque<String>,
    ready_to_advance : bool
}

impl NpcDialogueSegment {
    pub fn new() -> Self {
        Self {
            phrases: VecDeque::new(),
            ready_to_advance : false,
        }
    }

    pub fn do_next_phrase(&mut self, commands : &mut Commands, game : &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        let line = self.phrases.pop_front();
        if let Some(line) = line {
            let duration = game.npc_data.say(commands, line.as_str());
            event_caller.say_event = Some(SayEvent(duration));
        };
    }

    pub fn with_line(mut self, line : &str) -> Self {
        self.phrases.push_back(line.to_string());
        self
    }
}

impl Segment for NpcDialogueSegment {
    fn is_complete(&self) -> bool {
        self.ready_to_advance
    }

    fn on_npc_click(
        &mut self,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {
        if self.phrases.is_empty() {
            self.ready_to_advance = true;
        } else {
            self.do_next_phrase(commands, game, event_caller)
        }
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        self.do_next_phrase(commands, game, event_caller);
    }
}

//==================================================================================================
//                    Crafting Segment
//==================================================================================================

pub struct CraftingSegment {
    goal : Element,
    hints : Vec<String>,
    comments : HashMap<Element, String>,
    is_thing_crafted : bool,
    current_hint : usize,
    continue_on_craft : bool
}

impl CraftingSegment {

    pub fn new(element : Element, continue_on_craft : bool) -> Self {
        CraftingSegment {
            goal : element,
            hints : Vec::new(),
            comments : HashMap::new(),
            is_thing_crafted : false,
            current_hint : 0,
            continue_on_craft
        }
    }

    pub fn with_hint(mut self, hint : &str) -> CraftingSegment {
        self.hints.push(hint.to_string());
        self
    }

    pub fn with_comment(mut self, element : &'static Element, comment : &str) -> CraftingSegment {
        self.comments.insert(element.clone(), comment.to_string());
        self
    }

    pub fn cycle_hint(&mut self, commands : &mut Commands, game : &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        if self.current_hint >= self.hints.len() {
            self.current_hint = 0
        }
        let text = self.hints.get(self.current_hint).unwrap();
        let duration = game.npc_data.say(commands, text.as_str());
        event_caller.say_event = Some(SayEvent(duration));
        self.current_hint += 1;
    }
}

impl Segment for CraftingSegment {
    fn is_complete(&self) -> bool {
        self.is_thing_crafted
    }

    fn on_item_crafted(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller, element: Element) {
        if self.continue_on_craft {
            if element == self.goal {
                self.is_thing_crafted = true;
            }
        }

        if let Some(comment) = self.comments.get(&element) {
            let duration = game.npc_data.say(commands, comment.as_str());
            event_caller.say_event = Some(SayEvent(duration));
        }
    }

    fn on_npc_click(
        &mut self, commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller : &mut EventCaller
    ) {
        self.cycle_hint(commands, game, event_caller)
    }

    fn on_npc_drop(
        &mut self, commands: &mut Commands,
        asset_server: &Res<AssetServer>,
        game: &mut ResMut<GameManager>,
        event_caller: &mut EventCaller,
        element: Element
    ) {
        if !self.continue_on_craft {
            if element == self.goal {
                self.is_thing_crafted = true;
            }
        }
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        self.cycle_hint(commands, game, event_caller);
        game.can_use_ui = true;
    }

    fn on_segment_end(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        game.can_use_ui = false
    }
}

//==================================================================================================
//                    Give Element Segment
//==================================================================================================

pub struct GiveElementSegment {
    element : Element,
    optional_dialog : Option<String>,
    can_continue : bool
}

impl GiveElementSegment {
    pub fn new(element: Element) -> Self {
        Self {
            element,
            optional_dialog: None,
            can_continue : false
        }
    }

    pub fn with_line(mut self, line : &str) -> Self {
        self.optional_dialog = Some(line.to_string());
        self
    }
}

impl Segment for GiveElementSegment {
    fn is_complete(&self) -> bool {
        self.optional_dialog.is_none() || self.can_continue
    }

    fn on_npc_click(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        self.can_continue = true;
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        event_caller.insert_element_event = Some(InsertElementEvent(self.element.clone()));
        if let Some(dialog) = &self.optional_dialog {
            let duration = game.npc_data.say(commands, dialog.as_str());
            event_caller.say_event = Some(SayEvent(duration));
        }
    }
}

//==================================================================================================
//                    Setup Boss Fight
//==================================================================================================

pub struct StartBossFightSegment;

impl Segment for StartBossFightSegment {
    fn is_complete(&self) -> bool {
        true
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        event_caller.setup_boss_event = Some(SetupBossFightEvent)
    }
}

//==================================================================================================
//                    Toggle Boss Timer Segment
//==================================================================================================

pub struct ToggleBossTimerSegment;

impl Segment for ToggleBossTimerSegment {
    fn is_complete(&self) -> bool {
        true
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        event_caller.toggle_timer_event = Some(ToggleBossTimerEvent)
    }
}

//==================================================================================================
//                    Win Game Segment
//==================================================================================================

pub struct WinGameSegment;

impl Segment for WinGameSegment {
    fn is_complete(&self) -> bool {
        true
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        event_caller.win_game_event = Some(WinGameEvent)
    }
}

//==================================================================================================
//                    Load Tool Segment
//==================================================================================================

pub struct LoadToolSegment {
    craft_type : CraftType,
    optional_dialog : Option<String>,
    can_continue : bool
}

impl LoadToolSegment {
    pub fn new(craft_type : CraftType) -> Self {
        Self {
            craft_type,
            optional_dialog: None,
            can_continue : false
        }
    }

    pub fn with_line(mut self, line : &str) -> Self {
        self.optional_dialog = Some(line.to_string());
        self
    }
}

impl Segment for LoadToolSegment {
    fn is_complete(&self) -> bool {
        self.optional_dialog.is_none() || self.can_continue
    }

    fn on_npc_click(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        self.can_continue = true;
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        match self.craft_type {
            CraftType::SLICER => event_caller.load_slicer_event = Some(LoadSlicerEvent),
            CraftType::MIXER => event_caller.load_mixer_event = Some(LoadMixerEvent),
            CraftType::FURNACE => event_caller.load_furnace_event = Some(LoadFurnaceEvent),
        }
        if let Some(dialog) = &self.optional_dialog {
            let duration = game.npc_data.say(commands, dialog.as_str());
            event_caller.say_event = Some(SayEvent(duration));
        }
    }
}

//==================================================================================================
//                    Transition Segment
//==================================================================================================

pub struct TransitionSegment {
    leaving_phrases: Vec<String>,
    entering_phrases: Vec<String>,
    leaving_index: i32,
    entering_index: i32,
}

impl TransitionSegment {
    fn new(
        leaving_phrases: Vec<String>,
        entering_phrases: Vec<String>,
    ) -> Self {
        Self {
            leaving_phrases,
            entering_phrases,
            leaving_index: -1,
            entering_index: -1,
        }
    }

    pub fn is_old_npc_done(&self) -> bool {
        self.leaving_index >= (self.leaving_phrases.len() as i32) - 1
    }

    pub fn is_new_npc_done(&self) -> bool {
        self.entering_index >= (self.entering_phrases.len() as i32) - 1
    }

    pub fn get_next_phrase(&mut self) -> String {
        let get_phrase = |index: &mut i32, phrases: &Vec<String>| -> String {
            *index += 1;
            let i = *index as usize;
            return if let Some(s) = phrases.get(i as usize) {
                s.clone()
            } else {
                "Index Out of bounds".to_string()
            };
        };

        if self.is_old_npc_done() {
            get_phrase(&mut self.entering_index, &self.entering_phrases)
        } else {
            get_phrase(&mut self.leaving_index, &self.leaving_phrases)
        }
    }
}

impl Segment for TransitionSegment {
    fn is_complete(&self) -> bool {
        self.is_old_npc_done() && self.is_new_npc_done()
    }

    fn on_npc_click(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        if self.is_new_npc_done() {
            return;
        }
        if self.is_old_npc_done() && self.entering_index == -1 { // -1 because the 0 index of the dialogue Vec has not been said
            game.npc_data.spawn_next_npc()
        }
        let phrase = self.get_next_phrase();
        let duration = game.npc_data.say(commands, phrase.as_str());
        event_caller.say_event = Some(SayEvent(duration));
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        if self.is_new_npc_done() {
            return;
        }
        if self.is_old_npc_done() && self.entering_index == -1 { // -1 because the 0 index of the dialogue Vec has not been said
            game.npc_data.spawn_next_npc()
        }
        let phrase = self.get_next_phrase();
        let duration = game.npc_data.say(commands, phrase.as_str());
        event_caller.say_event = Some(SayEvent(duration));
    }
}

//==================================================================================================
//                    MusicTransitionSegment
//==================================================================================================
struct MusicChangeSegment {
    change_too: MusicTrack,
    music_changed: bool
}

impl MusicChangeSegment {
    fn change_too(track: MusicTrack) -> Self {
        Self {
            change_too: track,
            music_changed: false
        }
    }
}

impl Segment for MusicChangeSegment {
    fn is_complete(&self) -> bool {
        self.music_changed
    }

    fn on_segment_start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>, game: &mut ResMut<GameManager>, event_caller: &mut EventCaller) {
        event_caller.music_change_event = Some(MusicChangeEvent(self.change_too.clone()));
        self.music_changed = true;
    }
}