use ggez::glam::Vec2;
use ggez::graphics::Rect;

pub fn bounce_ball_off_rect(pos: &mut Vec2, vel: &mut Vec2, radius: f32, rect: &Rect) -> bool {
    let ball_box = Rect::new(pos.x - radius, pos.y - radius, radius * 2.0, radius * 2.0);
    if !ball_box.overlaps(rect) {
        return false;
    }
    let delta = Vec2::new(
        pos.x - (rect.x + rect.w / 2.0),
        pos.y - (rect.y + rect.h / 2.0),
    );
    let overlap_x = (ball_box.w + rect.w) / 2.0 - delta.x.abs();
    let overlap_y = (ball_box.h + rect.h) / 2.0 - delta.y.abs();
    if overlap_x < overlap_y {
        let sign = if delta.x >= 0.0 { 1.0 } else { -1.0 };
        pos.x += overlap_x * sign;
        vel.x = vel.x.abs() * sign;
    } else {
        let sign = if delta.y >= 0.0 { 1.0 } else { -1.0 };
        pos.y += overlap_y * sign;
        vel.y = vel.y.abs() * sign;
    }
    true
}
