use day_twelve::christmas_tree_farm::{can_fit_by_area, count_hashes, parse_input};

#[test]
fn test_example() {
    let input = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"#;

    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let (shapes, regions) = parse_input(&lines);

    // Verify we parsed 6 shapes
    assert_eq!(shapes.len(), 6, "Should have 6 shapes");

    // Verify shape areas (create ordered vec from hashmap)
    let mut shape_areas = [0; 6];
    for (idx, grid) in shapes.iter() {
        if *idx < 6 {
            shape_areas[*idx] = count_hashes(grid);
        }
    }
    assert_eq!(shape_areas[0], 7, "Shape 0 should have area 7");
    assert_eq!(shape_areas[1], 7, "Shape 1 should have area 7");
    assert_eq!(shape_areas[2], 7, "Shape 2 should have area 7");
    assert_eq!(shape_areas[3], 7, "Shape 3 should have area 7");
    assert_eq!(shape_areas[4], 7, "Shape 4 should have area 7");
    assert_eq!(shape_areas[5], 7, "Shape 5 should have area 7");

    // Verify we parsed 3 regions
    assert_eq!(regions.len(), 3, "Should have 3 regions");

    // Test region 1: 4x4 with counts [0,0,0,0,2,0]
    let (w1, h1, counts1) = &regions[0];
    assert_eq!(*w1, 4, "Region 1 width should be 4");
    assert_eq!(*h1, 4, "Region 1 height should be 4");
    assert_eq!(
        counts1,
        &[0, 0, 0, 0, 2, 0],
        "Region 1 counts should be [0,0,0,0,2,0]"
    );

    // Region 1: area = 16, required = 2*7 = 14, should fit
    assert!(
        can_fit_by_area(*w1, *h1, counts1, &shape_areas),
        "Region 1 (4x4) should fit 2 of shape 4"
    );

    // Test region 2: 12x5 with counts [1,0,1,0,2,2]
    let (w2, h2, counts2) = &regions[1];
    assert_eq!(*w2, 12, "Region 2 width should be 12");
    assert_eq!(*h2, 5, "Region 2 height should be 5");
    assert_eq!(
        counts2,
        &[1, 0, 1, 0, 2, 2],
        "Region 2 counts should be [1,0,1,0,2,2]"
    );

    // Region 2: area = 60, required = 1*7 + 1*7 + 2*7 + 2*7 = 7+7+14+14 = 42, should fit
    assert!(
        can_fit_by_area(*w2, *h2, counts2, &shape_areas),
        "Region 2 (12x5) should fit [1,0,1,0,2,2]"
    );

    // Test region 3: 12x5 with counts [1,0,1,0,3,2]
    let (w3, h3, counts3) = &regions[2];
    assert_eq!(*w3, 12, "Region 3 width should be 12");
    assert_eq!(*h3, 5, "Region 3 height should be 5");
    assert_eq!(
        counts3,
        &[1, 0, 1, 0, 3, 2],
        "Region 3 counts should be [1,0,1,0,3,2]"
    );

    // Region 3: area = 60, required = 1*7 + 1*7 + 3*7 + 2*7 = 7+7+21+14 = 49, should fit
    assert!(
        can_fit_by_area(*w3, *h3, counts3, &shape_areas),
        "Region 3 (12x5) should fit [1,0,1,0,3,2]"
    );
}

#[test]
fn test_shape_areas() {
    let input = r#"0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
"#;

    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let (shapes, _) = parse_input(&lines);

    let mut shape_areas = [0; 6];
    for (idx, grid) in shapes.iter() {
        if *idx < 6 {
            shape_areas[*idx] = count_hashes(grid);
        }
    }

    // Manually verify each shape
    assert_eq!(shape_areas[0], 7); // Shape 0: 3+2+2 = 7
    assert_eq!(shape_areas[1], 7); // Shape 1: 3+2+2 = 7
    assert_eq!(shape_areas[2], 7); // Shape 2: 2+3+2 = 7
    assert_eq!(shape_areas[3], 7); // Shape 3: 2+3+2 = 7
    assert_eq!(shape_areas[4], 7); // Shape 4: 3+1+3 = 7
    assert_eq!(shape_areas[5], 7); // Shape 5: 3+1+3 = 7
}

#[test]
fn test_impossible_region() {
    let input = r#"0:
###

2x2: 10 0 0 0 0 0
"#;

    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let (shapes, regions) = parse_input(&lines);

    let mut shape_areas = [0; 1];
    for (idx, grid) in shapes.iter() {
        if *idx < 1 {
            shape_areas[*idx] = count_hashes(grid);
        }
    }

    let (w, h, counts) = &regions[0];
    // Region area = 4, but needs 10 * 3 = 30, impossible
    assert!(
        !can_fit_by_area(*w, *h, counts, &shape_areas),
        "Should not fit 10 shapes of size 3 in a 2x2 region"
    );
}

#[test]
fn test_exact_fit() {
    let input = r#"0:
##

2x2: 2 0 0 0 0 0
"#;

    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let (shapes, regions) = parse_input(&lines);

    let mut shape_areas = [0; 1];
    for (idx, grid) in shapes.iter() {
        if *idx < 1 {
            shape_areas[*idx] = count_hashes(grid);
        }
    }

    let (w, h, counts) = &regions[0];
    // Region area = 4, needs 2 * 2 = 4, exact fit
    assert!(
        can_fit_by_area(*w, *h, counts, &shape_areas),
        "Should exactly fit 2 shapes of size 2 in a 2x2 region"
    );
}
