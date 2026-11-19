use super::*;

fn cell(image_id: u32, row: Option<u32>, column: Option<u32>) -> Option<PlaceholderCell> {
    Some(PlaceholderCell {
        image_id,
        placement_id: 0,
        row,
        column,
        image_id_high: None,
    })
}

fn run(x: usize, length: usize, image_id: u32, row: u32, column: u32) -> PlaceholderRun {
    PlaceholderRun {
        x,
        length,
        image_id,
        placement_id: 0,
        image_row: row,
        image_column: column,
    }
}

#[test]
fn diacritic_table_is_sorted_and_indexes_from_zero() {
    assert!(DIACRITICS.windows(2).all(|pair| pair[0] < pair[1]));
    assert_eq!(diacritic_index('\u{305}'), Some(0));
    assert_eq!(diacritic_index('\u{30D}'), Some(1));
    assert_eq!(diacritic_index('\u{1D244}'), Some(296));
    assert_eq!(diacritic_index('\u{306}'), None);
}

#[test]
fn cells_with_full_diacritics_join_into_one_run() {
    let cells = [
        cell(5, Some(2), Some(0)),
        cell(5, Some(2), Some(1)),
        cell(5, Some(2), Some(2)),
    ];
    assert_eq!(placeholder_runs(&cells), vec![run(0, 3, 5, 2, 0)]);
}

#[test]
fn cells_without_diacritics_continue_the_left_neighbour() {
    let cells = [
        cell(5, Some(1), Some(3)),
        cell(5, None, None),
        cell(5, None, None),
    ];
    assert_eq!(placeholder_runs(&cells), vec![run(0, 3, 5, 1, 3)]);
}

#[test]
fn a_first_cell_without_diacritics_is_the_image_origin() {
    assert_eq!(
        placeholder_runs(&[cell(5, None, None)]),
        vec![run(0, 1, 5, 0, 0)]
    );
}

#[test]
fn a_cell_with_only_a_row_continues_the_column_on_the_same_row_and_restarts_on_another() {
    let cells = [
        cell(5, Some(1), Some(0)),
        cell(5, Some(1), None),
        cell(5, Some(2), None),
    ];
    assert_eq!(
        placeholder_runs(&cells),
        vec![run(0, 2, 5, 1, 0), run(2, 1, 5, 2, 0)]
    );
}

#[test]
fn a_gap_a_different_image_or_a_column_jump_starts_a_new_run() {
    let cells = [
        cell(5, Some(0), Some(0)),
        None,
        cell(5, Some(0), Some(1)),
        cell(6, Some(0), Some(2)),
        cell(6, Some(0), Some(4)),
    ];
    assert_eq!(
        placeholder_runs(&cells),
        vec![
            run(0, 1, 5, 0, 0),
            run(2, 1, 5, 0, 1),
            run(3, 1, 6, 0, 2),
            run(4, 1, 6, 0, 4),
        ]
    );
}

#[test]
fn the_third_diacritic_supplies_the_top_byte_of_the_image_id() {
    let high = Some(PlaceholderCell {
        image_id_high: Some(2),
        ..cell(5, Some(0), Some(0)).unwrap()
    });
    let runs = placeholder_runs(&[high, cell(5, None, None)]);
    assert_eq!(runs, vec![run(0, 2, (2 << 24) | 5, 0, 0)]);
}

#[test]
fn different_placement_ids_do_not_join() {
    let other = Some(PlaceholderCell {
        placement_id: 9,
        ..cell(5, None, None).unwrap()
    });
    let runs = placeholder_runs(&[cell(5, Some(0), Some(0)), other]);
    assert_eq!(runs.len(), 2);
}
