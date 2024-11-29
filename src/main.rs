use fmi::{CoSimulationInstance, FmiModelDescription};
use gtk::prelude::*;
use std::path::Path;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize GTK for the UI
    gtk::init()?;

    // Load the FMU
    let fmu_path = Path::new("path/to/your/model.fmu");
    let model_description = FmiModelDescription::from_fmu(&fmu_path)?;

    // Extract co-simulation details
    let instance = CoSimulationInstance::new(&model_description, &fmu_path)?;

    // Create a GTK window with a slider to interact with the model
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    window.set_title("Interactive Simulation");
    window.set_default_size(400, 200);

    let slider = gtk::Scale::new_with_range(gtk::Orientation::Horizontal, 0.0, 10.0, 0.1);
    slider.set_value(5.0); // Default value
    window.add(&slider);

    // Start the simulation and handle real-time interaction
    let start_time = 0.0;
    let stop_time = 10.0; // Simulate for 10 seconds
    let mut current_time = start_time;
    let step_size = 0.1; // 100ms step size

    // Setup FMU for co-simulation
    instance.setup_experiment(Some(start_time), Some(stop_time), None)?;
    instance.enter_initialization_mode()?;
    instance.exit_initialization_mode()?;

    let real_time_start = Instant::now();

    // Update the FMU during the simulation based on slider interaction
    slider.connect_value_changed(move |slider| {
        let slider_value = slider.get_value();
        // Update FMU inputs with the new value (assuming index 0 is the input)
        if let Err(e) = instance.set_real(0, slider_value) {
            eprintln!("Error setting input: {}", e);
        }
    });

    // Real-time simulation loop
    while current_time < stop_time {
        let elapsed = real_time_start.elapsed().as_secs_f64();
        if elapsed < current_time {
            std::thread::sleep(Duration::from_secs_f64(current_time - elapsed));
        }

        // Step the simulation
        instance.do_step(current_time, step_size, true)?;
        current_time += step_size;

        // Print or update simulation outputs (for instance, output with index 0)
        let output_value = instance.get_real(0)?;
        println!("Time: {:.2}, Output: {:.4}", current_time, output_value);

        // Process GTK events
        gtk::main_iteration();
    }

    instance.terminate()?;

    Ok(())
}
