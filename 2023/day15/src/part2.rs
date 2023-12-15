const MULTIPLY_CONSTANT: usize = 17;

struct Lens {
    label: String,
    focal_length: usize,
}

pub fn solve(input: &str) -> String {
    let mut lens_boxes: [Vec<Lens>; 256] = [(); 256].map(|_| Vec::new());
    let lens_instructions: Vec<&str> = input.trim_end().split(',').collect();

    for step in lens_instructions {
        apply_lens_instructions_step(step, &mut lens_boxes);
    }

    // Calculate the sum of each boxes focusing powers
    lens_boxes
        .iter()
        .enumerate()
        .map(calculate_box_focusing_power)
        .sum::<usize>()
        .to_string()
}

fn calculate_box_focusing_power(box_with_index: (usize, &Vec<Lens>)) -> usize {
    let (box_index, lens_box) = box_with_index;
    lens_box
        .iter()
        .enumerate()
        .map(|(index, lens)| lens.focal_length * (index + 1) * (box_index + 1))
        .sum::<usize>()
}

fn apply_lens_instructions_step(step: &str, lens_boxes: &mut [Vec<Lens>; 256]) {
    let label: String = extract_label(step).expect("label");
    if step.contains('=') {
        let box_number = hash(&label);
        let focal_length: usize = step
            .chars()
            .last()
            .expect("focal length")
            .to_digit(10)
            .expect("digit") as usize;
        let lens = Lens {
            label,
            focal_length,
        };
        insert_to_lens_box(&mut lens_boxes[box_number], lens)
    } else if step.contains('-') {
        remove_from_lens_boxes(lens_boxes, &label);
    }
}

fn extract_label(input: &str) -> Option<String> {
    input
        .find(|c: char| c == '=' || c == '-')
        .map(|idx| input[..idx].to_string())
}

fn remove_from_lens_boxes(lens_boxes: &mut [Vec<Lens>; 256], label: &str) {
    for lens_box in lens_boxes {
        if let Some(index) = lens_box.iter().position(|lens| lens.label == label) {
            lens_box.remove(index);
        }
    }
}

fn insert_to_lens_box(lens_box: &mut Vec<Lens>, lens: Lens) {
    if let Some(lens_index) = lens_box.iter().position(|l| l.label == lens.label) {
        lens_box[lens_index] = lens;
    } else {
        lens_box.push(lens);
    }
}

fn hash(input: &str) -> usize {
    input.chars().fold(0, |current_value, character| {
        (current_value + character as usize) * MULTIPLY_CONSTANT % 256
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let solution = "145";
        assert_eq!(solution, solve(input));
    }
}
