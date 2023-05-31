use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Race {
    SpaceMarines,
    ChaosSpaceMarines,
    Orks,
    Eldar,
    OrdoMalleus,
    Tyranids,
}
