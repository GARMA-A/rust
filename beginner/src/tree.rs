use std::io;

fn tree() {
    let mut buf = String::new();

    println!("Enter a number : ");
    io::stdin().read_line(&mut buf).unwrap();
    let num: i32 = buf.trim().parse().unwrap();
    draw_tree(&num);
}

fn draw_tree(height: &i32) {
    let height = height - 1;
    let (mut num_of_stars_to_print, mut num_of_spaces_to_print): (i32, i32) = (0, height + 3);
    for cur_row in (-1..height).step_by(1) {
        while num_of_spaces_to_print > 0 {
            print!(" ");
            num_of_spaces_to_print -= 1;
        }
        while (cur_row + 1) * 2 + 1 > num_of_stars_to_print {
            print!("*");
            num_of_stars_to_print += 1;
        }
        num_of_spaces_to_print = (height - cur_row + 1).abs();
        num_of_stars_to_print = 0;
        print!("\n")
    }
}
