use iced::{
    Color, Point,
    widget::canvas::{self, Path, path::lyon_path::geom::euclid::default::Transform2D},
};

const STROKE: &str = "
M 500 225
C 393 225 326 341 379 434
C 385 444 393 454 401 462
L 338 505
C 335 540 346 556 358 577
L 415 577
C 383 764 234 678 235 907
L 765 907
C 766 678 617 764 585 577
L 642 577
C 654 556 665 540 662 505
L 599 462
C 607 454 615 444 621 434
C 674 341 607 225 500 225
Z
";

const INNER: &str = "
M 500 247.324
C 417.648 247.324 363.574 329.018 389.65 402.596
C 392.038 409.332 395.097 416.001 398.868 422.532
C 408.743 439.635 422.792 453.955 440 464
L 360 516
C 360 530 364 540 372 554
L 440 554
C 437.386 580.256 432.825 602.306 426.815 621.091
C 382.24 760.416 258 720.098 258 884
L 742 884
C 742 698 582 775 560 554
L 628 554
C 636 540 640 530 640 516
L 560 464
C 577.208 453.955 591.257 439.635 601.132 422.532
C 644.964 346.613 592.644 252.188 506.691 247.505
C 504.482 247.385 502.252 247.324 500 247.324
Z
";

const SHADOW0: &str = "
M 440 554
C 439.09 563.143 437.943 571.777 436.582 579.939
C 494.19 615.74 552.604 736.93 695.859 761.19
C 646.789 715.367 574.289 697.535 560 554
Z
";

const SHADOW1: &str = "
M 560 554
L 628 554
C 636 540 640 530 640 516
L 560 464
C 570.028 482.197 604.253 514.57 560 554
Z
";

const SHADOW2: &str = "
M 486 464
L 560 464
C 577.208 453.955 591.257 439.635 601.132 422.532
C 632.143 368.82 615.025 305.844 571.919 271.941
C 629.563 375.062 570.432 437.373 486 464
Z
";

const GLARE0: &str = "
M 426.815 621.091
C 382.24 760.416 258 720.098 258 884
C 306.646 730.341 415.542 807.777 426.815 621.091
Z
";

const GLARE1: &str = "
M 440 464
L 360 516
L 416 516
C 419.055 499.553 426.868 482.691 440 464
Z
";

const GLARE2: &str = "
M 506.691 247.505
C 504.482 247.385 502.252 247.324 500 247.324
C 417.648 247.324 363.574 329.018 389.65 402.596
C 390.399 380.388 412.643 271.938 506.691 247.505
";

fn get_path(data: &str) -> Path {
    Path::new(|p| {
        let mut nums = Vec::new();
        let mut cmd = None;

        for token in data.split_whitespace() {
            match token {
                "M" | "L" | "C" | "Z" => {
                    if token != "Z" {
                        cmd = Some(token);
                        nums.clear();
                    } else {
                        p.close();
                        cmd = None;
                    }
                }
                _ => {
                    nums.push(token.parse::<f64>().unwrap());

                    match cmd {
                        Some("M") if nums.len() == 2 => {
                            p.move_to(Point::new(nums[0] as f32, nums[1] as f32));
                            nums.clear();
                        }
                        Some("L") if nums.len() == 2 => {
                            p.line_to(Point::new(nums[0] as f32, nums[1] as f32));
                            nums.clear();
                        }
                        Some("C") if nums.len() == 6 => {
                            p.bezier_curve_to(
                                Point::new(nums[0] as f32, nums[1] as f32),
                                Point::new(nums[2] as f32, nums[3] as f32),
                                Point::new(nums[4] as f32, nums[5] as f32),
                            );
                            nums.clear();
                        }
                        _ => {}
                    }
                }
            }
        }
    })
}

pub fn draw(frame: &mut canvas::Frame, size: f32, offset: Point) {
    let scale = size / 1000.0;

    let translation = Transform2D::new(scale, 0.0, 0.0, scale, offset.x, offset.y);
    let colors_paths = [
        (Color::from_rgb8(37, 35, 35), vec![STROKE]),
        (Color::from_rgb8(92, 91, 90), vec![INNER]),
        (
            Color::from_rgb8(70, 70, 69),
            vec![SHADOW0, SHADOW1, SHADOW2],
        ),
        (
            Color::from_rgb8(133, 130, 129),
            vec![GLARE0, GLARE1, GLARE2],
        ),
    ];

    for (color, paths) in colors_paths {
        for path in paths.iter() {
            let mut path = get_path(path);
            path = path.transform(&translation);
            frame.fill(&path, color);
        }
    }
}
