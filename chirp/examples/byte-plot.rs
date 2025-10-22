use chirp::message::modulate;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("target/example-graphs")?;
    let path = "target/example-graphs/message-plot.svg";

    // modulate a single byte
    const SAMPLES: usize = 32 * 8;
    let mut buffer = vec![0.0; SAMPLES];
    modulate(&[0b01010101], &mut buffer).ok();

    let root = SVGBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Modulated Byte (LSB)", ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0f64..(SAMPLES as f64), -1.2f64..1.2f64)?;

    chart
        .configure_mesh()
        .x_desc("Samples")
        .y_desc("Amplitude")
        .label_style(("sans-serif", 16))
        .draw()?;

    // Sampled data: exponentially damped sine
    let samples: Vec<(f64, f64)> = buffer
        .into_iter()
        .enumerate()
        .map(|(i, v)| (i as f64, v as f64))
        .collect();

    // zero order hold
    chart.draw_series(samples.windows(2).flat_map(|w| {
        let (x0, y0) = w[0];
        let (x1, y1) = w[1];
        vec![
            PathElement::new(vec![(x0, y0), (x1, y0)], BLUE),
            PathElement::new(vec![(x1, y0), (x1, y1)], BLUE),
        ]
    }))?;

    root.present()?;
    println!("Saved {}", path);
    Ok(())
}
