pub mod ui;
pub mod automata;
pub mod automatas;
pub mod ca;
pub mod nca;
pub mod dnca;
pub mod config;
pub mod filter;

pub use ui::UI;
pub use automata::Automata;
pub use ca::CellularAutomata;
pub use nca::NeuralCellularAutomata;
pub use dnca::DeepNeuralCellularAutomata;
pub use filter::Filter;