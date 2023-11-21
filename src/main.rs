use minifb::{Key, Window, WindowOptions};
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let (width, height) = (2000, 1200);

    // 表示する画像をバッファとして作成する
    // 色並びは右の通り ARGB
    let argb: u32 = 255 << 24 | 125 << 16 | 125 << 8 | 125;
    let mut buffer: Vec<u32> = vec![argb; width as usize * height as usize];
    // 一部を別の色で塗りつぶす
    for _i in 0..1000 {
        let rect_x: usize = rng.gen_range(0..=width);
        let rect_y: usize = rng.gen_range(0..=height);
        let rect_width: usize = rng.gen_range(1..=500);
        let rect_height: usize = rng.gen_range(1..=500);
        let hue: usize = rng.gen_range(0..=360);
        let argb_area: u32 = hsv_to_rgb(hue as f32, 1.0, 1.0);
        draw_rectangle_fill(&mut buffer, width, height, rect_x, rect_y, rect_width, rect_height, argb_area);
    }

    // 画像を表示するウィンドウを作成
    let mut window = Window::new(
        "Image with Plots",
        width as usize,
        height as usize,
        WindowOptions::default(),
        )
        .expect("ウィンドウの作成に失敗しました");

    // ウィンドウの表示
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(&buffer, width as usize, height as usize)
            .expect("ウィンドウの更新に失敗しました");
    }

    println!("Bye!");
}

fn draw_rectangle_fill(buffer: &mut Vec<u32>, width: usize, height: usize, x_left: usize, y_left: usize, rect_width: usize, rect_height: usize, argb: u32) {
    // 描画範囲を評価する式
    let is_in_img = |x: usize, y: usize, width: usize, height: usize| -> bool {
        x < width && y < height
    };

    for x in x_left..=x_left + rect_width {
        for y in y_left..=y_left + rect_height {
            if is_in_img(x, y, width, height) {
                buffer[y * width + x] = argb;
            }
        }
    }
}

pub fn hsv_to_rgb(h: f32, s: f32, v: f32) -> u32 {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _=> (c, 0.0, x),
    };

    // ARGBの順に32ビットで返す
    255 << 24 |(((r + m) * 255.0) as u32) << 16 | (((g + m) * 255.0) as u32) << 8 | (((b + m) * 255.0) as u32)
}

