// TODO: add normalizing gain to output legend
use chirp::BitModulator;
use plotters::prelude::*;

/// Simulate a 2nd-order “speaker” system (mass–spring–damper) at discrete timestep `dt`.
/// Inputs:
///   - `input`: reference to sampled input signal
///   - `f0_hz`: resonance frequency in Hz
///   - `zeta`: damping ratio (dimensionless)
///   - `dt`: timestep in seconds (1 / sampling rate)
/// Returns:
///   - filtered output samples as a Vec<f32>
pub fn speaker_response(input: &[f64], f0_hz: f64, zeta: f64, dt: f64) -> Vec<f64> {
    let dt = dt.max(1e-9);
    let fs = 1.0 / dt;
    let nyquist = 0.5 * fs;
    let f0 = f0_hz.clamp(1.0, 0.99 * nyquist);
    let zeta = zeta.max(1e-4);

    // pre-warp analog ω₀ so digital resonance matches f0
    let w0 = (2.0 / dt) * (std::f64::consts::PI * f0 * dt).tan();

    let c = 2.0 / dt;
    let c2 = c * c;
    let w0_2 = w0 * w0;

    // denominator coefficients before normalization
    let d0 = c2 + 2.0 * zeta * w0 * c + w0_2;
    let d1 = -2.0 * c2 + 2.0 * w0_2;
    let d2 = c2 - 2.0 * zeta * w0 * c + w0_2;

    // numerator (w0² * (1 + z⁻¹)²)
    let n0 = w0_2;
    let n1 = 2.0 * w0_2;
    let n2 = w0_2;

    // normalize so a0 = 1
    let a0 = d0;
    let b0 = n0 / a0;
    let b1 = n1 / a0;
    let b2 = n2 / a0;
    let a1 = d1 / a0;
    let a2 = d2 / a0;

    // process
    let mut x1 = 0.0;
    let mut x2 = 0.0;
    let mut y1 = 0.0;
    let mut y2 = 0.0;
    let mut out = Vec::with_capacity(input.len());

    for &x0 in input {
        // difference equation
        let y0 = b0 * x0 + b1 * x1 + b2 * x2 - a1 * y1 - a2 * y2;
        out.push(y0);
        x2 = x1;
        x1 = x0;
        y2 = y1;
        y1 = y0;
    }

    out
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all("target/example-graphs")?;
    let path = "target/example-graphs/sim-plot.svg";

    const SAMPLES: usize = 32;
    const TIMESTEP: f64 = 1.0 / 48_000.0;
    const DURATION: f64 = SAMPLES as f64 * TIMESTEP;
    let input: Vec<_> = BitModulator::default()
        .modulate(true)
        .iter()
        .map(|x| *x as f64)
        .collect();
    let upsampled_input: Vec<_> = input.iter().flat_map(|x| [*x; 10]).collect();
    let output = speaker_response(&upsampled_input, 1_000.0, 0.7, TIMESTEP / 10.0);
    let max_output = output.iter().cloned().reduce(f64::max).unwrap();
    let norm_output: Vec<_> = output.into_iter().map(|x| x / max_output).collect();
    let input_time: Vec<_> = (0..input.len()).map(|i| i as f64 * TIMESTEP).collect();
    let output_time: Vec<_> = (0..norm_output.len())
        .map(|i| (i as f64 * TIMESTEP) / 10.0)
        .collect();

    let root = SVGBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Speaker Model (2nd Order)", ("sans-serif", 28))
        .margin(20)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(0.0..DURATION, -1.2..1.2f64)?;

    chart
        .configure_mesh()
        .x_desc("Time (s)")
        .y_desc("Normalized Amplitude (y/x)")
        .label_style(("sans-serif", 16))
        .draw()?;

    let input_series: Vec<(f64, f64)> = input_time
        .iter()
        .cloned()
        .zip(input.iter().cloned())
        .collect();
    let output_series: Vec<(f64, f64)> = output_time
        .iter()
        .cloned()
        .zip(norm_output.iter().cloned())
        .collect();

    // zero order hold - input
    chart
        .draw_series(input_series.windows(2).flat_map(|w| {
            let (x0, y0) = w[0];
            let (x1, y1) = w[1];
            vec![
                PathElement::new(vec![(x0, y0), (x1, y0)], BLUE),
                PathElement::new(vec![(x1, y0), (x1, y1)], BLUE),
            ]
        }))?
        .label("driver input")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    // simulated speaker - output
    chart
        .draw_series(LineSeries::new(output_series, &RED))?
        .label(format!("speaker output: {:.1}x", (1.0 / max_output)))
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    // draw legend
    chart
        .configure_series_labels()
        .border_style(BLACK)
        .background_style(WHITE.mix(0.8))
        .label_font(("sans-serif", 16))
        .draw()?;

    root.present()?;
    println!("Saved {}", path);
    Ok(())
}
