//! Ecological Entity & Specimen Inspection Module
//! Bridges 3D proximity in the primary vista with companion tablet inspection loupes.

pub mod loupe;

pub use loupe::{
    BotanicalDevelopmentalStage, DevelopmentalStage, GhostGumOrganSubmesh, GhostGumPhenology,
    GhostGumSpecimenLoupe,
};
