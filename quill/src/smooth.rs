// Interpolation: Bezier
// Simulation: Spring

// SwiftUI spring:
//   response: Double = 0.55,
//   dampingFraction: Double = 0.825,
//   blendDuration: Double = 0

// mass = 1
// stiffness = pow(2 * .pi / frequencyResponse, 2) * mass = 130
// damping = 4 * .pi * dampingRatio * mass / frequencyResponse = 18.85
// damping = dampingRatio * 2 * sqrt(mass * stiffness)

// f(t) = x
// f'(t) = dx

type Duration = f64;
type Time = f64;

type FrequencyResponse = f64;
type DampingFactor = f64;

enum Smooth {
    Constant(f64),
    Lerp(Time, Duration, Box<Smooth>, Box<Smooth>),
    // Curve(Time, Duration, [f64; 4], Box<Smooth>, Box<Smooth>),
    Spring(Time, FrequencyResponse, DampingFactor, Box<Smooth>, f64),
}

impl Smooth {
    fn terminal(&self) -> f64 {
        match self {
            Smooth::Constant(val) => *val,
            Smooth::Lerp(_time, _dur, _from, to) => to.terminal(),
            Smooth::Spring(_time, _freq, _damp, _from, to) => *to,
        }
    }
}
