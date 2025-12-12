use std::collections::HashMap;
use rayon::prelude::*;

pub fn run(lines: &[String]) -> usize {
    let (shapes_raw, regions) = parse_input(lines);
    
    let mut shape_areas: Vec<usize> = vec![];
    let max_shape_idx = shapes_raw.keys().max().copied().unwrap_or(0);
    for idx in 0..=max_shape_idx {
        let s = shapes_raw.get(&idx).cloned().unwrap_or_else(Vec::new);
        let area = count_hashes(&s);
        shape_areas.push(area);
    }

   
    let solvable = regions.par_iter()
        .enumerate()
        .filter(|(i, (w, h, counts))| {
            if *i % 100 == 0 {
                eprintln!("Processing region {}/{}...", i, regions.len());
            }
            can_fit_by_area(*w, *h, &shape_areas, counts)
        })
        .count();
    
    eprintln!("Done! {} solvable regions found.", solvable);
    solvable
}

type ShapeGrid = Vec<String>;
type Region = (usize, usize, Vec<usize>);
type ParseResult = (HashMap<usize, ShapeGrid>, Vec<Region>);

pub fn parse_input(lines: &[String]) -> ParseResult {
    let mut shapes: HashMap<usize, ShapeGrid> = HashMap::new();
    let mut regions: Vec<(usize, usize, Vec<usize>)> = Vec::new();

    #[derive(PartialEq)]
    enum State { Shapes, Regions }
    let mut state = State::Shapes;

    let mut i = 0usize;
    while i < lines.len() {
        let line = lines[i].trim();
        if line.is_empty() {
            i += 1;
            continue;
        }
        
        if state == State::Shapes
            && let Some(colpos) = line.find(':')
        {
            let left = &line[..colpos];
            if left.contains('x') && left.chars().all(|c| c.is_ascii_digit() || c=='x') {
                state = State::Regions;
                continue;
            }
        }

        match state {
            State::Shapes => {
                if let Some(colpos) = line.find(':') {
                    let idx_str = line[..colpos].trim();
                    if let Ok(idx) = idx_str.parse::<usize>() {
                        let after = line[colpos+1..].trim();
                        let mut grid: Vec<String> = Vec::new();
                        if !after.is_empty() {
                            for token in after.split_whitespace() {
                                if token.chars().all(|c| c=='.' || c=='#') {
                                    grid.push(token.to_string());
                                }
                            }
                            shapes.insert(idx, grid);
                            i += 1;
                            continue;
                        } else {
                            i += 1;
                            while i < lines.len() {
                                let l = lines[i].trim();
                                if l.is_empty() { break; }
                                if l.ends_with(':') && l[..l.len()-1].chars().all(|c| c.is_ascii_digit()) {
                                    break;
                                }
                                if let Some(colpos2) = l.find(':') {
                                    let left2 = &l[..colpos2];
                                    if left2.contains('x') && left2.chars().all(|c| c.is_ascii_digit() || c=='x') {
                                        break;
                                    }
                                }
                                grid.push(l.to_string());
                                i += 1;
                            }
                            shapes.insert(idx, grid);
                            continue;
                        }
                    }
                }
                i += 1;
            }
            State::Regions => {
                let rest = line;
                if !rest.is_empty()
                    && let Some(colpos) = rest.find(':')
                {
                    let left = rest[..colpos].trim();
                    let after_part = rest[colpos+1..].trim();
                    if let Some(xpos) = left.find('x') {
                        let w_s = &left[..xpos];
                        let h_s = &left[xpos+1..];
                        if let (Ok(w), Ok(h)) = (w_s.parse::<usize>(), h_s.parse::<usize>()) {
                            let mut nums: Vec<usize> = Vec::new();
                            let mut tokens: Vec<String> = after_part.split_whitespace().map(|s| s.to_string()).collect();
                            let mut look = 1usize;
                            while tokens.len() < 6 && i + look < lines.len() {
                                let more = lines[i + look].trim();
                                if more.is_empty() { break; }
                                for t in more.split_whitespace() { tokens.push(t.to_string()); }
                                look += 1;
                            }
                            for tok in tokens.iter().take(6) {
                                if let Ok(n) = tok.parse::<usize>() {
                                    nums.push(n);
                                }
                            }
                            if nums.len() == 6 {
                                regions.push((w, h, nums));
                            }
                            if look > 1 {
                                i += look - 1;
                            }
                        }
                    }
                }
                i += 1;
            }
        }
    }

    (shapes, regions)
}

pub fn count_hashes(grid: &ShapeGrid) -> usize {
    grid.iter()
        .map(|row| row.chars().filter(|&c| c == '#').count())
        .sum()
}

pub fn can_fit_by_area(w: usize, h: usize, shape_areas: &[usize], counts: &[usize]) -> bool {
    let region_area = w * h;
    
    let mut required_area = 0usize;
    for (i, &count) in counts.iter().enumerate() {
        if i < shape_areas.len() {
            required_area += count * shape_areas[i];
        }
    }
    
    required_area <= region_area
}
