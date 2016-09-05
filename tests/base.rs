extern crate mersh;

fn compute_triangle_area(p0: &mersh::base::Pnt2d, p1: &mersh::base::Pnt2d, p2: &mersh::base::Pnt2d) -> f64
{
    let v01 = p0.to(&p1);
    let v02 = p0.to(&p2);
    0.5 * (v01.coords.x * v02.coords.y - v01.coords.y * v02.coords.x).abs()
}

#[test]
fn area_triangle() {

    // Defining points of triangle
    let p0 = mersh::base::Pnt2d::new(0., 0.);
    let p1 = mersh::base::Pnt2d::new(1., 0.);
    let p2 = mersh::base::Pnt2d::new(0., 1.);

    // Computing triangle area.
    assert!((compute_triangle_area(&p0, &p1, &p2) - 0.5).abs() < 1e-10);

    // Translating points using random vector.
    let u = mersh::base::Vec2d::new(3.4, -9.6);

    // Recomputing area.
    let q0 = p0.add(&u);
    let q1 = p1.add(&u);
    let q2 = p2.add(&u);
    assert!((compute_triangle_area(&q0, &q1, &q2) - 0.5).abs() < 1e-10);
}
