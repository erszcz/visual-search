extern crate search;

use search::map;

const TEST_IMAGE: &str = "/Users/erszcz/work/erszcz/visual-search/test.png";

fn main() {
    let image = map::png::load_image(TEST_IMAGE);
    println!("{:?}", image);
}
