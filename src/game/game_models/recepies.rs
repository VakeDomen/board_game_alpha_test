use std::collections::HashMap;

use super::{structure::{StructureSelector, StructureRecepie, StructureStats}, resource::Resouce};



pub fn get_recepies() -> HashMap<StructureSelector, StructureRecepie> {
    let mut recepies: HashMap<StructureSelector, StructureRecepie> = HashMap::new();
    recepies.insert(StructureSelector::BugBase, StructureRecepie {
        cost: vec![],
        footprint: vec![
            vec![true],
        ],
        stats: StructureStats {
            hp: 1,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![],
    });

    recepies.insert(StructureSelector::TechBase, StructureRecepie {
        cost: vec![],
        footprint: vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, true, true],
        ],
        stats: StructureStats {
            hp: 10,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![],
    });

    recepies.insert(StructureSelector::TechMine1, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
        ],
        footprint: vec![
            vec![true, true],
            vec![true, true],
        ],
        stats: StructureStats {
            hp: 2,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![
            vec![Resouce::Metal]
        ],
    });
    recepies.insert(StructureSelector::TechMine2, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
            Resouce::Metal,
        ],
        footprint: vec![
            vec![true, true],
            vec![true, true],
        ],
        stats: StructureStats {
            hp: 2,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![],
    });

    recepies.insert(StructureSelector::TechRefinery1, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
        ],
        footprint: vec![
            vec![true, true],
            vec![true, true],
        ],
        stats: StructureStats {
            hp: 2,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![
            vec![Resouce::Gold],
            vec![Resouce::Metal],
        ],
    });
    recepies.insert(StructureSelector::TechRefinery2, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
            Resouce::Metal,
        ],
        footprint: vec![
            vec![true, true],
            vec![true, true],
        ],
        stats: StructureStats {
            hp: 2,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![
            vec![Resouce::Gold]
        ],
    });
    recepies.insert(StructureSelector::TechMarket, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
            Resouce::Metal,
        ],
        footprint: vec![
            vec![true, true],
            vec![true, true],
        ],
        stats: StructureStats {
            hp: 2,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![
            vec![Resouce::Gold],
            vec![Resouce::Metal],
        ],
    });
    recepies.insert(StructureSelector::TechTurret1, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
            Resouce::Metal,
        ],
        footprint: vec![
            vec![true],
        ],
        stats: StructureStats {
            hp: 3,
            attack: 3,
            range: 2,
        },
        activated_costs: vec![
            vec![Resouce::Metal],
        ],
    });
    recepies.insert(StructureSelector::TechTurret2, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
            Resouce::Metal,
            Resouce::Metal,
        ],
        footprint: vec![
            vec![true],
        ],
        stats: StructureStats {
            hp: 3,
            attack: 5,
            range: 3,
        },
        activated_costs: vec![],
    });
    recepies.insert(StructureSelector::TechArtillery1, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
            Resouce::Metal,
        ],
        footprint: vec![
            vec![true],
        ],
        stats: StructureStats {
            hp: 3,
            attack: 1,
            range: 7,
        },
        activated_costs: vec![
            vec![Resouce::Metal],
        ],
    });
    recepies.insert(StructureSelector::TechArtillery2, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
            Resouce::Gold, 
            Resouce::Gold,
            Resouce::Metal,
            Resouce::Metal,
        ],
        footprint: vec![
            vec![true],
        ],
        stats: StructureStats {
            hp: 3,
            attack: 2,
            range: 8,
        },
        activated_costs: vec![],
    });
    recepies.insert(StructureSelector::TechWall1, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
        ],
        footprint: vec![
            vec![true],
        ],
        stats: StructureStats {
            hp: 2,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![],
    });
    recepies.insert(StructureSelector::TechRoad, StructureRecepie {
        cost: vec![
            Resouce::Gold, 
        ],
        footprint: vec![
            vec![true],
        ],
        stats: StructureStats {
            hp: 1,
            attack: 0,
            range: 0,
        },
        activated_costs: vec![],
    });
    
    recepies
}