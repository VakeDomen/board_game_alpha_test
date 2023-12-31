

pub fn get_placers() -> HashMap<StructureSelector, Option<Box<dyn Placable>>> {
    let mut hm: HashMap<StructureSelector, Option<Box<dyn ActiveAbility>>> = HashMap::new();
    hm.insert(StructureSelector::BugBase1, None);
    hm.insert(StructureSelector::BugBase2, None);
    hm.insert(StructureSelector::BugBase3, None);
    hm.insert(StructureSelector::TechBase, None);
    hm.insert(StructureSelector::TechRoad, None);
    hm.insert(StructureSelector::TechMine1, None);
    hm.insert(StructureSelector::TechMine2, None);
    hm.insert(StructureSelector::TechRefinery1, None);
    hm.insert(StructureSelector::TechRefinery2, None);
    hm.insert(StructureSelector::TechMarket, None);
    hm.insert(StructureSelector::TechTurret1, None);
    hm.insert(StructureSelector::TechTurret2, None);
    hm.insert(StructureSelector::TechArtillery1, None);
    hm.insert(StructureSelector::TechArtillery2, None);
    hm.insert(StructureSelector::TechWall1, None);
    hm.insert(StructureSelector::TechNuke, None);
    hm
}