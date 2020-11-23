use bengine::gui::*;
use legion::*;
use nox_components::*;
use nox_raws::RAWS;
use parking_lot::RwLock;

struct BuildingInfo {
    id: usize,
    name: ImString,
    description: ImString,
    tag: String,
    contents: Vec<ImString>,
    complete: bool,
    reactions: Vec<BuildingReaction>
}

impl BuildingInfo {
    fn new() -> Self {
        Self{
            id: 0,
            name: ImString::new(String::new()),
            description: ImString::new(String::new()),
            tag: String::new(),
            contents: Vec::new(),
            complete: false,
            reactions: Vec::new(),
        }
    }
}

struct BuildingReaction {
    name: ImString,
    auto: bool,
    mode: usize,
    qty: i32
}

lazy_static! {
    static ref BUILDING_INFO: RwLock<BuildingInfo> = RwLock::new(BuildingInfo::new());
}

pub fn setup_building_info(id: usize, ecs: &World) {
    let mut bl = BUILDING_INFO.write();
    *bl = BuildingInfo::new(); // Clear it
    bl.id = id;

    let (name, description, building, entity, btag) = <(&IdentityTag, &Name, &Description, &Building, Entity, &Tag)>::query()
        .iter(ecs)
        .filter(|(bid, _, _, _, _, _)| bid.0 == id)
        .map(|(_, n, d, b, e, tag)| (ImString::new(&n.name), ImString::new(&d.desc), b, *e, tag.0.clone()))
        .nth(0)
        .unwrap();

    bl.name = name;
    bl.description = description;
    bl.tag = btag.clone();
    bl.complete = building.complete;

    // Check container contents
    <(Read<Name>, Read<Position>)>::query()
        .iter(ecs)
        .filter(|(_, store)| store.is_in_container(id))
        .for_each(|(name, _)| {
            bl.contents.push(ImString::new(&name.name));
        }
    );

    if let Ok(er) = ecs.entry_ref(entity) {
        if let Ok(_ws) = er.get_component::<Workshop>() {
            RAWS.read().reactions.reactions.iter()
                .filter(|r| r.workshop == btag)
                .for_each(|r| {
                    let br = BuildingReaction{
                        name : ImString::new(&r.name),
                        auto : r.automatic,
                        mode: 0,
                        qty: 0
                    };
                    bl.reactions.push(br);
                }
            );
        }
    }
}

pub fn show_building_info(imgui: &Ui, _ecs: &World, _id: &usize) {
    let reaction_modes = [
        im_str!("Make"),
        im_str!("Until You Have"),
    ];

    let mut bl = BUILDING_INFO.write();
    let tmp_name = bl.name.clone();
    let window = Window::new(&tmp_name);
    window
        .size(
            [600.0, 400.0],
            Condition::FirstUseEver,
        )
        .movable(true)
        .position([20.0, 20.0], Condition::FirstUseEver)
        .build(imgui, || {
            imgui.text_wrapped(&bl.description);
            if !bl.complete {
                imgui.text_colored([1.0, 0.0, 0.0, 1.0], im_str!("(Incomplete)"));
            }

            // Check container contents
            if !bl.contents.is_empty() {
                imgui.text_colored([1.0, 1.0, 0.0, 1.0], im_str!("Contains the following items:"));
                bl.contents.iter().for_each(|c| {
                    imgui.text(c);
                });
            }

            // Check for reactions
            if !bl.reactions.is_empty() {
                imgui.text_colored([1.0, 1.0, 0.0, 1.0], im_str!("Available Commands:"));
                bl.reactions.iter_mut().for_each(|r| {
                    imgui.text(&r.name);
                    imgui.set_next_item_width(250.0);
                    if r.auto {
                        imgui.same_line(0.0);
                        imgui.text(im_str!("(auto)"));
                    }
                    imgui.same_line(260.0);

                    imgui.set_next_item_width(100.0);
                    ComboBox::new(&ImString::new(&format!("##m{}", r.name))).build_simple_string(
                        &imgui,
                        &mut r.mode,
                        &reaction_modes,
                    );
                    imgui.same_line(0.0);
                    imgui.set_next_item_width(75.0);
                    imgui
                        .input_int(&ImString::new(&format!("##qty{}", r.name)), &mut r.qty)
                        .step(1)
                        .step_fast(1)
                        .build();
                    imgui.same_line(0.0);
                    imgui.button(im_str!("Queue"), [50.0, 20.0]);
                });
            }
        }
    );
}