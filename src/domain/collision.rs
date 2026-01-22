pub fn check_aabb_collision(
    pos1: (f32, f32),
    size1: (f32, f32),
    pos2: (f32, f32),
    size2: (f32, f32),
) -> bool {
    let (x1, y1) = pos1;
    let (width1, height1) = size1;
    let (x2, y2) = pos2;
    let (width2, height2) = size2;

    let x_overlap = x1 < x2 + width2 / 2.0 && x1 + width1 > x2 - width2 / 2.0;
    let y_overlap = y1 < y2 + height2 / 2.0 && y1 + height1 > y2 - height2 / 2.0;

    x_overlap && y_overlap
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_boxes_collide() {
        let collision = check_aabb_collision((0.0, 0.0), (10.0, 10.0), (5.0, 5.0), (10.0, 10.0));
        assert!(collision);
    }

    #[test]
    fn separated_boxes_do_not_collide() {
        let collision = check_aabb_collision((0.0, 0.0), (10.0, 10.0), (20.0, 20.0), (10.0, 10.0));
        assert!(!collision);
    }

    #[test]
    fn touching_edges_collide() {
        let collision = check_aabb_collision((0.0, 0.0), (10.0, 10.0), (10.0, 0.0), (10.0, 10.0));
        assert!(collision);
    }

    #[test]
    fn boxes_overlapping_only_on_x_do_not_collide() {
        let collision = check_aabb_collision((0.0, 0.0), (10.0, 10.0), (5.0, 20.0), (10.0, 10.0));
        assert!(!collision);
    }

    #[test]
    fn boxes_overlapping_only_on_y_do_not_collide() {
        let collision = check_aabb_collision((0.0, 0.0), (10.0, 10.0), (20.0, 5.0), (10.0, 10.0));
        assert!(!collision);
    }

    #[test]
    fn small_box_inside_large_box_collides() {
        let collision = check_aabb_collision((0.0, 0.0), (20.0, 20.0), (0.0, 0.0), (5.0, 5.0));
        assert!(collision);
    }

    #[test]
    fn identical_boxes_collide() {
        let collision = check_aabb_collision((10.0, 10.0), (5.0, 5.0), (10.0, 10.0), (5.0, 5.0));
        assert!(collision);
    }

    #[test]
    fn negative_positions_work() {
        let collision =
            check_aabb_collision((-10.0, -10.0), (10.0, 10.0), (-5.0, -5.0), (10.0, 10.0));
        assert!(collision);
    }
}
