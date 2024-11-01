use std::collections::HashMap;
use std::ops::Mul;
use std::sync::{Arc, Mutex};
use sfml::cpp::FBox;
use sfml::graphics::{CircleShape, Color, CustomShape, CustomShapePoints, Drawable, RenderStates, RenderTarget, RenderWindow, Shape, Sprite, Texture, Transformable};
use sfml::system::Vector2f;

static PAW: Color = Color::rgba(255, 255, 255, 255);
static PAW_EDGE: Color = Color::rgba(0, 0, 0, 255);
static PAW_EDGE_SHADOW: Color = Color::rgba(0, 0, 0, 85);
static PINK: Color = Color::rgba(255, 192, 203, 255);

pub fn draw_mouse_arm (textures: &HashMap<&str, FBox<Texture>>, window: &mut RenderWindow) {
    let mouse = get_xy();
    let x = mouse.x;
    let y = mouse.y;

    // initializing pss and pss2 (kuvster's magic)
    let oof = 6;
    let mut pss: Vec<f32> = Vec::new();
    pss.push(211.0);
    pss.push(159.0);

    let mut dist = (211.0 - x).hypot(159.0 - y);
    let centreleft0 = 211.0 - (0.7237 * (dist / 2.0));
    let centreleft1 = 159.0 + (0.69 * (dist / 2.0));

    for i in 1 .. oof {
        let bez = [211.0, 159.0, centreleft0, centreleft1, x, y];
        let tup = bezier(1.0 * (i as f32 / oof as f32), &bez, 6);
        pss.push(tup.0);
        pss.push(tup.1);
    }

    pss.push(x);
    pss.push(y);

    let mut a = y - centreleft1;
    let mut b = centreleft0 - x;
    let mut le = a.hypot(b);
    a = x + ((a / le) * 60.0);
    b = y + ((b / le) * 60.0);
    let a1 = 258.0;
    let a2 = 228.0;
    dist = (a1 - a).hypot(a2 - b);
    let centreright0 = a1 - (0.6 * (dist / 2.0));
    let centreright1 = a2 + (0.8 * (dist / 2.0));
    let push = 20.0;
    let mut s = x - centreleft0;
    let mut t = y - centreleft1;
    le = s.hypot(t);
    s *= push / le;
    t *= push / le;
    let mut s2 = a - centreright0;
    let mut t2 = b - centreright1;
    le = s2.hypot(t2);
    s2 *= push / le;
    t2 *= push / le;

    for i in 1..oof {
        let bez = [x, y, x + s, y + t, a + s2, b + t2, a, b];
        let tup = bezier(1.0 * (i as f32 / oof as f32), &bez, 8);
        pss.push(tup.0);
        pss.push(tup.1);
    }

    pss.push(a);
    pss.push(b);

    for i in (1..oof).rev() {
        let bez = [1.0 * a1, 1.0 * a2, centreright0, centreright1, a, b];
        let tup = bezier(1.0 * (i as f32 / oof as f32), &bez, 6);
        pss.push(tup.0);
        pss.push(tup.1);
    }

    pss.push(a1);
    pss.push(a2);

    let mpos0 = ((a + x) / 2.0) - 52.0 - 15.0;
    let mpos1 = ((b + y) / 2.0) - 34.0 + 5.0;
    let dx = -38.0;
    let dy = -50.0;

    let iter = 25;

    let mut pss2: Vec<f32> = Vec::new();
    pss2.push(pss[0] + dx);
    pss2.push(pss[1] + dy);

    for i in 1..iter {
        let tup = bezier(1.0 * (i as f32 / iter as f32), &pss, 38);
        pss2.push(tup.0 + dx);
        pss2.push(tup.1 + dy);
    }

    pss2.push(pss[36] + dx);
    pss2.push(pss[37] + dy);


    // draw mouse
    match textures.get("mouse") {
        Some(texture) => {
            let mut rs = RenderStates::DEFAULT;
            rs.transform.translate(mpos0 + dx, mpos1 + dy);
            window.draw_sprite(&Sprite::with_texture(texture), &rs)
        },
        None => panic!("mouse texture not loaded")
    };


    // drawing arms
    let mut fill: Vec<[f32;2]> = Vec::new();

    for i in (0..26).step_by(2) {
        fill.push([ pss2[i], pss2[i + 1] ]);
        fill.push([ pss2[52 - i - 2], pss2[52 - i - 1] ]);
    }

    // drawing first arm arc
    let mut edge: Vec<[f32;2]> = Vec::new();
    let mut width = 6.0;

    for i in (0..50).step_by(2) {
        let vec0 = pss2[i] - pss2[i + 2];
        let vec1 = pss2[i + 1] - pss2[i + 3];
        let dist = vec0.hypot(vec1);
        edge.push([pss2[i] + ((vec1 / dist) * (width / 2.0)), pss2[i + 1] - ((vec0 / dist) * (width / 2.0))]);
        edge.push([pss2[i] - ((vec1 / dist) * (width / 2.0)), pss2[i + 1] + ((vec0 / dist) * (width / 2.0))]);
        width -= 0.08;
    }

    let vec0 = pss2[50] - pss2[48];
    let vec1 = pss2[51] - pss2[49];
    dist = vec0.hypot(vec1);
    edge.push([pss2[50] - ((vec1 / dist) * 2.0), pss2[51] + ((vec0 / dist) * 2.0)]);
    edge.push([pss2[50] + ((vec1 / dist) * 2.0), pss2[51] - ((vec0 / dist) * 2.0)]);



    let mut curr = edge[1];
    let mut arm_points = MultiPointShape::new();

    for i in 0..edge.len()-2 {
        arm_points.add_point(Vector2f::new(curr[0], curr[1]));
        if i%2==0 {curr = edge[i + 1];} else {curr = edge[i + 2];}
    }
    curr = edge[edge.len()-1];
    arm_points.add_point(Vector2f::new(curr[0], curr[1]));

    let mut arm_edge_points = arm_points.clone();
    for i in (3..edge.len()-1).rev() {
        arm_edge_points.add_point(Vector2f::new(curr[0], curr[1]));
        if i%2==0 {curr = edge[i - 1];} else {curr = edge[i - 2];}
    }


    // White paw
    let mut arm_shape = CustomShape::new(Box::new(arm_points.clone()));
    arm_shape.set_fill_color(PAW);
    arm_shape.set_outline_thickness(1.0);
    arm_shape.draw(window, &RenderStates::DEFAULT);

    // Shadow outline
    let mut arm_edge_shadow_shape = CustomShape::new(Box::new(arm_edge_points.clone()));
    arm_edge_shadow_shape.set_fill_color(Color::TRANSPARENT);
    arm_edge_shadow_shape.set_outline_color(PAW_EDGE_SHADOW);
    arm_edge_shadow_shape.set_outline_thickness(5.0);
    arm_edge_shadow_shape.draw(window, &RenderStates::DEFAULT);

    // Edge outline
    let mut arm_edge_shape = CustomShape::new(Box::new(arm_edge_points));
    arm_edge_shape.set_fill_color(Color::TRANSPARENT);
    arm_edge_shape.set_outline_color(PAW_EDGE);
    arm_edge_shape.set_outline_thickness(4.0);
    arm_edge_shape.draw(window, &RenderStates::DEFAULT);


    // Left circle shadow
    draw_circle(1.0, 12, pss2[0] - 3.0, pss2[1] - 3.0, PAW_EDGE_SHADOW, PAW_EDGE_SHADOW, 1.0, window);

    // Left circle
    draw_circle(2.0, 12, pss2[0] - 2.0, pss2[1] - 2.0, PAW_EDGE, PAW_EDGE, 1.0, window);

    // Right circle shadow
    draw_circle(1.0, 12, pss2[50] - 1.0, pss2[51] - 1.0, PAW_EDGE_SHADOW, PAW_EDGE_SHADOW, 1.0, window);

    // Right circle
    draw_circle(0.75, 12, pss2[50] - 0.5, pss2[51] - 0.5, PAW_EDGE, PAW_EDGE, 1.0, window);
}

pub fn draw_mouth (window: &mut RenderWindow, mic_volume: &Arc<Mutex<f32>>) {
    let l_anchor = Vector2f::new(271.0, 148.0);
    let r_anchor = Vector2f::new(294.0, 157.0);
    let mut m_anchor = Vector2f::new(282.0, 153.0);

    let volume = mic_volume.lock().unwrap();

    if volume.gt(&0.01) {
        m_anchor.y += volume.mul(&150.0);

        // Black circles
        draw_circle(4.0, 12, l_anchor.x - 4.0, l_anchor.y - 4.0, PAW_EDGE, PAW_EDGE, 1.0, window);
        draw_circle(4.0, 12, r_anchor.x - 4.0, r_anchor.y - 4.0, PAW_EDGE, PAW_EDGE, 1.0, window);

        // Mouth
        let bezier_curve = MultiPointShape::quadratic_bezier(l_anchor, m_anchor, r_anchor, 10);
        let mut bezier_curve_shape = CustomShape::new(Box::new(bezier_curve.clone()));
        bezier_curve_shape.set_fill_color(PINK);
        bezier_curve_shape.set_outline_color(PAW_EDGE);
        bezier_curve_shape.set_outline_thickness(5.0);
        bezier_curve_shape.draw(window, &RenderStates::DEFAULT);

        // Pink circle
        draw_circle(0.75, 12, 283.0 - 0.75, 152.0 - 0.75, PINK, PINK, 1.0, window);
    }
}

fn draw_circle (radius: f32, point_count: usize, x: f32, y: f32, fill_color: Color, outline_color: Color, outline_thickness: f32, window: &mut RenderWindow) {
    let mut circle = CircleShape::new(radius, point_count);
    circle.set_position(Vector2f::new(x, y));
    circle.set_fill_color(fill_color);
    circle.set_outline_color(outline_color);
    circle.set_outline_thickness(outline_thickness);
    circle.draw(window, &RenderStates::DEFAULT);
}

fn bezier (ratio: f32, points: &[f32], length: i32) -> (f32, f32) {
    let fact = [0.001, 0.001, 0.002, 0.006, 0.024, 0.12, 0.72, 5.04, 40.32, 362.88, 3628.8, 39916.8, 479001.6, 6227020.8, 87178291.2, 1307674368.0, 20922789888.0, 355687428096.0, 6402373705728.0, 121645100408832.0, 2432902008176640.0, 51090942171709440.0];
    let nn = (length / 2) - 1;

    let mut xx = 0.0;
    let mut yy = 0.0;

    for point in 0..=nn {
        let tmp = fact[nn as usize] / (fact[point as usize] * fact[nn as usize - point as usize]) * ratio.powi(point as i32) * (1.0 - ratio).powi(nn as i32 - point as i32);
        let temp: usize = (2 * point) as usize;
        xx += points[temp] * tmp;
        yy += points[temp + 1] * tmp;
    }
    return (xx / 1000.0, yy / 1000.0);
}

fn get_xy() -> Vector2f {
    let mut mouse = sfml::window::mouse::desktop_position();
    let w = sfml::window::VideoMode::desktop_mode().width as i32;
    let h = sfml::window::VideoMode::desktop_mode().height as i32;

    if mouse.x < 0 { mouse.x = 0; }
    if mouse.x > w { mouse.x = w-1; }
    if mouse.y < 0 { mouse.y = 0; }
    if mouse.y > h { mouse.y = h-1; }

    let mut fx: f32 = mouse.x as f32 / w as f32;
    let mut fy: f32 = mouse.y as f32 / h as f32;

    fx = if fx < 1.0 {fx} else {1.0};
    fx = if fx > 0.0 {fx} else {0.0};

    fy = if fy < 1.0 {fy} else {1.0};
    fy = if fy > 0.0 {fy} else {0.0};

    let x = -97.0 * fx + 44.0 * fy + 164.0;
    let y = -76.0 * fx - 40.0 * fy + 339.0;

    return Vector2f::new(x, y);
}

#[derive(Clone)]
pub struct MultiPointShape {
    point_count: usize,
    points: Vec<Vector2f>
}

impl CustomShapePoints for MultiPointShape {
    fn point_count(&self) -> usize {
        return self.point_count;
    }

    fn point(&self, point: usize) -> sfml::system::Vector2f {
        match self.points.get(point) {
            Some(p) => return *p,
            None => panic!("WHAT")
        }
    }
}

impl MultiPointShape {
    fn new() -> Self {
        MultiPointShape {
            point_count: 0,
            points: Vec::new(),
        }
    }

    fn add_point(&mut self, new_point: Vector2f) {
        self.points.push(new_point);
        self.point_count += 1;
    }

    fn quadratic_bezier(p0: Vector2f, p1: Vector2f, p2: Vector2f, segments: u32) -> MultiPointShape {
        let mut bezier = MultiPointShape::new();

        bezier.add_point(p0); // Needed to prevent strange behavior
        for i in 0..=segments {
            let t = i as f32 / segments as f32;
            let x = (1.0 - t).powi(2) * p0.x + 2.0 * (1.0 - t) * t * p1.x + t.powi(2) * p2.x;
            let y = (1.0 - t).powi(2) * p0.y + 2.0 * (1.0 - t) * t * p1.y + t.powi(2) * p2.y;
            bezier.add_point(Vector2f::new(x, y));
        }
        bezier.add_point(p2); // Needed to prevent strange behavior

        return bezier;
    }
}
