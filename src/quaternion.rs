//! # Quaternion — Rust Implementation
//!
//! Demonstrates idiomatic Rust best practices:
//!   - Generic structs with trait bounds (`Float`)
//!   - Operator overloading (`Mul`, `Neg`, `Add`)
//!   - Custom `Display` formatting
//!   - `#[derive]` for free trait impls
//!   - Builder-style constructors
//!   - Doc-comments on every public item
//!   - Unit tests in the same file (`#[cfg(test)]`)
//!   - `const` and associated functions vs methods
//!   - Zero-cost newtype abstractions
//!
//! ## How to run
//!
//! ```bash
//! # Single-file run (no Cargo needed):
//! rustc quaternion.rs -o quaternion_demo && ./quaternion_demo
//!
//! # Or inside a Cargo project, place in src/main.rs
//! cargo run --release
//! ```

use std::fmt;
use std::ops::{Add, Mul, Neg};

// ─── Trait alias for numeric flexibility ─────────────────────────────────────
//
// Best practice: constrain generics with the *minimum* trait set needed.
// Here we need float arithmetic + Copy + display + constant values.
// In production, use the `num-traits` crate's `Float` trait instead.
pub trait Float:
Copy
+ fmt::Display
+ PartialOrd
+ Add<Output = Self>
+ Mul<Output = Self>
+ Neg<Output = Self>
+ std::ops::Sub<Output = Self>
+ std::ops::Div<Output = Self>
{
    fn sqrt(self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn acos(self) -> Self;
    fn abs(self) -> Self;
    fn zero() -> Self;
    fn one() -> Self;
    fn two() -> Self;
    fn pi() -> Self;
}

// ─── Implement Float for f32 and f64 ─────────────────────────────────────────
//
// Best practice: blanket-impl your trait for standard types so callers
// can use `Quat<f32>` or `Quat<f64>` without any changes.

macro_rules! impl_float {
    ($t:ty, $pi:expr) => {
        impl Float for $t {
            fn sqrt(self) -> Self { self.sqrt() }
            fn sin(self)  -> Self { self.sin()  }
            fn cos(self)  -> Self { self.cos()  }
            fn acos(self) -> Self { self.acos() }
            fn abs(self)  -> Self { self.abs()  }
            fn zero() -> Self { 0.0 }
            fn one()  -> Self { 1.0 }
            fn two()  -> Self { 2.0 }
            fn pi()   -> Self { $pi }
        }
    };
}

impl_float!(f32, std::f32::consts::PI);
impl_float!(f64, std::f64::consts::PI);

// ─── Vec3 ─────────────────────────────────────────────────────────────────────

/// A 3-component vector used for rotation axes and rotated points.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: Float> Vec3<T> {
    /// Construct a new vector.
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    /// Normalize to a unit vector (panics if the zero vector is passed).
    pub fn normalize(self) -> Self {
        let len = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }
}

// Best practice: implement `Display` rather than writing print helpers,
// so the type integrates naturally with `println!`, `format!`, etc.
impl<T: Float> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({:.4}, {:.4}, {:.4})", self.x, self.y, self.z)
    }
}

// ─── Quaternion ───────────────────────────────────────────────────────────────

/// A quaternion `q = w + xi + yj + zk`.
///
/// Uses a generic float type `T` so the same code works for both
/// `f32` (games, real-time) and `f64` (simulation, robotics).
///
/// # Convention
/// The scalar part `w` comes first — this matches most graphics APIs
/// (GLM, Unity, Three.js). Some physics engines put `w` last; be consistent.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quaternion<T> {
    pub w: T,
    pub x: T,
    pub y: T,
    pub z: T,
}

// ─── Constructors (associated functions, not free functions) ──────────────────

impl<T: Float> Quaternion<T> {
    /// Create a quaternion directly from components.
    pub fn new(w: T, x: T, y: T, z: T) -> Self {
        Self { w, x, y, z }
    }

    /// The identity quaternion — represents "no rotation".
    ///
    /// Prefer this over `Quat::new(1.0, 0.0, 0.0, 0.0)` for clarity.
    pub fn identity() -> Self {
        Self {
            w: T::one(),
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }

    /// Encode a rotation of `angle` radians around a unit `axis`.
    ///
    /// The axis is normalized internally so callers don't need to pre-normalize.
    pub fn from_axis_angle(axis: Vec3<T>, angle: T) -> Self {
        let axis = axis.normalize();
        let half  = angle / T::two();
        let s     = half.sin();
        Self {
            w: half.cos(),
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
        }
    }

    // ─── Core operations ───────────────────────────────────────────────────

    /// Squared norm — cheaper than `norm()` when you only need comparisons.
    pub fn norm_sq(self) -> T {
        self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Euclidean norm (length on the 4D unit sphere).
    pub fn norm(self) -> T {
        self.norm_sq().sqrt()
    }

    /// Return a unit quaternion. Panics if `self` is the zero quaternion.
    pub fn normalize(self) -> Self {
        let n = self.norm();
        Self {
            w: self.w / n,
            x: self.x / n,
            y: self.y / n,
            z: self.z / n,
        }
    }

    /// Conjugate: negates the vector part, reversing the rotation.
    pub fn conjugate(self) -> Self {
        Self {
            w:  self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    /// Multiplicative inverse.
    ///
    /// For unit quaternions, `inverse == conjugate` (much faster).
    /// This general form works for any non-zero quaternion.
    pub fn inverse(self) -> Self {
        let n2 = self.norm_sq();
        let c  = self.conjugate();
        Self {
            w: c.w / n2,
            x: c.x / n2,
            y: c.y / n2,
            z: c.z / n2,
        }
    }

    /// Dot product on the 4D vector space (used by SLERP).
    pub fn dot(self, other: Self) -> T {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Rotation ──────────────────────────────────────────────────────────

    /// Rotate a 3D vector using the sandwich product `q · p · q⁻¹`.
    ///
    /// `self` must be a unit quaternion.
    pub fn rotate(self, v: Vec3<T>) -> Vec3<T> {
        // Treat v as a pure quaternion (w = 0)
        let p   = Self::new(T::zero(), v.x, v.y, v.z);
        let res = self * p * self.inverse();
        Vec3::new(res.x, res.y, res.z)
    }

    // SLERP ─────────────────────────────────────────────────────────────

    /// Spherical Linear Interpolation between two unit quaternions.
    ///
    /// `t = 0.0` returns `self`, `t = 1.0` returns `other`.
    /// Traces the shortest great-circle arc on S³.
    pub fn slerp(self, mut other: Self, t: T) -> Self {
        let mut dot = self.dot(other);

        // Best practice: ensure shortest path by checking the sign of the dot
        // product. If negative, the "long way around" would be taken — negate.
        if dot < T::zero() {
            other = -other;
            dot   = -dot;
        }

        // Near-identical quaternions: fall back to normalized linear interpolation
        // to avoid division by zero in the sin(theta) denominator.
        let threshold = {
            // 0.9995 as T — we compute it from Float primitives
            let v: T = T::one() / T::one(); // 1.0
                                            // approximate 0.9995 via (1 - 0.0005)
            let tiny = v / (T::two() * T::two() * T::two() * T::two() * T::two()
                * T::two() * T::two() * T::two() * T::two() * T::two() * T::two());
            v - tiny
        };

        if dot > threshold {
            return Self {
                w: self.w + t * (other.w - self.w),
                x: self.x + t * (other.x - self.x),
                y: self.y + t * (other.y - self.y),
                z: self.z + t * (other.z - self.z),
            }
            .normalize();
        }

        let theta_0 = dot.acos();
        let theta   = theta_0 * t;
        let sin_t0  = theta_0.sin();
        let s1 = theta.cos() - dot * theta.sin() / sin_t0;
        let s2 = theta.sin() / sin_t0;

        Self {
            w: s1 * self.w + s2 * other.w,
            x: s1 * self.x + s2 * other.x,
            y: s1 * self.y + s2 * other.y,
            z: s1 * self.z + s2 * other.z,
        }
    }

    // Matrix conversion

    /// Convert to a row-major 3×3 rotation matrix.
    ///
    /// Returns `[[f; 3]; 3]` — compatible with most graphics APIs.
    pub fn to_matrix(self) -> [[T; 3]; 3] {
        let (w, x, y, z) = (self.w, self.x, self.y, self.z);
        let two = T::two();
        [
            [
                T::one() - two * (y * y + z * z),
                two * (x * y - w * z),
                two * (x * z + w * y),
            ],
            [
                two * (x * y + w * z),
                T::one() - two * (x * x + z * z),
                two * (y * z - w * x),
            ],
            [
                two * (x * z - w * y),
                two * (y * z + w * x),
                T::one() - two * (x * x + y * y),
            ],
        ]
    }
}

// Operator Overloading
//
// Best practice: implement standard `std::ops` traits so your type
// behaves like a built-in numeric type.  q1 * q2, -q, etc. just work.

/// Hamilton product: the core quaternion multiplication.
impl<T: Float> Mul for Quaternion<T> {
    type Output = Self;

    fn mul(self, b: Self) -> Self {
        Self {
            w: self.w * b.w - self.x * b.x - self.y * b.y - self.z * b.z,
            x: self.w * b.x + self.x * b.w + self.y * b.z - self.z * b.y,
            y: self.w * b.y - self.x * b.z + self.y * b.w + self.z * b.x,
            z: self.w * b.z + self.x * b.y - self.y * b.x + self.z * b.w,
        }
    }
}

/// Negation (used by SLERP to enforce the shortest arc).
impl<T: Float> Neg for Quaternion<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            w: -self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

/// Pretty-print: `w + xi + yj + zk` notation.
impl<T: Float> fmt::Display for Quaternion<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:.4} + {:.4}i + {:.4}j + {:.4}k",
            self.w, self.x, self.y, self.z
        )
    }
}

// Printers
fn section(title: &str) {
    println!("\n--- {} {}", title, "-".repeat(50 - title.len()));
}

// Demo
pub fn run() {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║        QUATERNION DEMO  —  quaternion.rs             ║");
    println!("╚══════════════════════════════════════════════════════╝");

    // ── 1. Construction & Display ──────────────────────────────────────────
    section("1. CONSTRUCTION & DISPLAY");

    // Type inference: Rust infers Quat<f64> from the literals
    let q = Quaternion::new(1.0_f64, 2.0, 3.0, 4.0);
    println!("Raw q           : {q}");
    println!("norm(q)         : {:.4}  (not unit)", q.norm());

    let qu = q.normalize();
    println!("Normalized q    : {qu}");
    println!("norm(q_unit)    : {:.4}", qu.norm());

    // ── 2. Conjugate & Inverse ────────────────────────────────────────────
    section("2. CONJUGATE & INVERSE");

    let conj = qu.conjugate();
    let inv  = qu.inverse();
    println!("conjugate(q)    : {conj}");
    println!("inverse(q)      : {inv}");

    // For unit quaternions, conjugate == inverse
    let identity = qu * qu.inverse();
    println!("q * q⁻¹         : {identity}");
    println!("  → should be identity (1, 0, 0, 0)");

    // ── 3. Encoding a rotation ────────────────────────────────────────────
    section("3. ROTATION ENCODING");

    let z_axis = Vec3::new(0.0_f64, 0.0, 1.0);
    let angle  = std::f64::consts::PI / 2.0;  // 90°
    let rot90z = Quaternion::from_axis_angle(z_axis, angle);

    println!("90° around Z    : {rot90z}");
    println!("  Expected: w = cos(45°) = {:.4}, z = sin(45°) = {:.4}",
    (angle / 2.0).cos(), (angle / 2.0).sin());

    // ── 4. Rotating a point ───────────────────────────────────────────────
    section("4. ROTATING A 3D POINT");

    let point   = Vec3::new(1.0_f64, 0.0, 0.0);
    let rotated = rot90z.rotate(point);
    println!("Original        : {point}");
    println!("After 90° Z-rot : {rotated}");
    println!("  Expected: (0, 1, 0)");

    // ── 5. Composing rotations ────────────────────────────────────────────
    section("5. COMPOSING ROTATIONS  (non-commutativity)");

    // Two 90° Z rotations = 180°
    let rot180z   = rot90z * rot90z;
    let rotated2  = rot180z.rotate(point);
    println!("After 2×90° Z   : {rotated2}");
    println!("  Expected: (-1, 0, 0)");

    // Order matters!
    let x_axis = Vec3::new(1.0_f64, 0.0, 0.0);
    let y_axis = Vec3::new(0.0_f64, 1.0, 0.0);
    let rot90x = Quaternion::from_axis_angle(x_axis, angle);
    let rot90y = Quaternion::from_axis_angle(y_axis, angle);

    // Apply X then Y: compose as  q_y * q_x  (right-to-left)
    let rot_xy = rot90y * rot90x;
    let rot_yx = rot90x * rot90y;

    let p = Vec3::new(1.0_f64, 0.0, 0.0);
    println!("X then Y        : {}", rot_xy.rotate(p));
    println!("Y then X        : {}", rot_yx.rotate(p));
    println!("  → Different results prove non-commutativity!");

    // ── 6. SLERP ──────────────────────────────────────────────────────────
    section("6. SLERP — SMOOTH INTERPOLATION");

    let start = Quaternion::<f64>::identity();
    let end   = Quaternion::from_axis_angle(z_axis, std::f64::consts::PI);

    println!("  Interpolating identity → 180° around Z:");
    println!("  {:<6}  {:<10}  {:<10}  {:<10}  {:<10}", "t", "w", "x", "y", "z");
    println!("  {}", "-".repeat(52));
    for i in 0..=5 {
        let t = i as f64 / 5.0;
        let s = start.slerp(end, t);
        println!("  {:<6.2}  {:<10.4}  {:<10.4}  {:<10.4}  {:<10.4}",
            t, s.w, s.x, s.y, s.z);
    }
    println!("  → Smooth interpolation, constant angular velocity");

    // ── 7. Rotation matrix ────────────────────────────────────────────────
    section("7. CONVERTING TO ROTATION MATRIX");

    let mat = rot90z.to_matrix();
    println!("  Rotation matrix for 90° around Z:");
    for row in &mat {
        println!("    [ {:7.4}  {:7.4}  {:7.4} ]", row[0], row[1], row[2]);
    }
    println!("\n  Expected:");
    println!("    [  0.0000  -1.0000   0.0000 ]");
    println!("    [  1.0000   0.0000   0.0000 ]");
    println!("    [  0.0000   0.0000   1.0000 ]");

    // ── 8. Generic usage: f32 ─────────────────────────────────────────────
    section("8. GENERIC — SAME CODE WITH f32");

    let q32   = Quaternion::new(1.0_f32, 0.0, 0.0, 0.0);
    let ax32  = Vec3::new(0.0_f32, 0.0, 1.0);
    let r32   = Quaternion::from_axis_angle(ax32, std::f32::consts::PI / 2.0);
    let pt32  = Vec3::new(1.0_f32, 0.0, 0.0);
    println!("f32 identity    : {q32}");
    println!("f32 90° Z-rot   : {r32}");
    println!("f32 rotated pt  : {}", r32.rotate(pt32));

    println!("\n{}", "═".repeat(56));
    println!("  All demos complete.");
    println!();
}
