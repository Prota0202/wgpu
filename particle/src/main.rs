mod particles_app2;

use std::sync::Arc;

use crate::particles_app2::ParticleApp;
use wgpu_bootstrap::{egui, Runner};

fn main() {
    let mut runner = Runner::new(
        "Gui App",
        800,
        600,
        egui::Color32::from_rgb(245, 245, 245),
        32,
        0,
        Box::new(|context| Arc::new(ParticleApp::new(context))),
    );
    runner.run();
}