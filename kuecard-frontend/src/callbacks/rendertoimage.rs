use resvg::{tiny_skia::Pixmap, usvg::{Options, Size, Transform, Tree}};
use vector_x::Vector2;

pub fn render_to_image(
    svg_bytes: &[u8],
    target_size: impl Into<Vector2<u32>>,
    options: Options
) -> Result<Vec<u8>, String> {
    let target_size: Vector2<u32> = target_size.into();

    let res = Tree::from_data(svg_bytes, &options);

    if res.is_err() {
        return Result::Err(res.err().unwrap().to_string());
    }

    let tree: Tree = res.unwrap();

    let size: Size = tree.size();

    let scale_x: f32 = (target_size.one as f32) / size.width();
    let scale_y: f32 = (target_size.two as f32) / size.height();

    let scale: f32 = f32::min(scale_x, scale_y);

    let img_size: Vector2<u32> = Vector2::new(
        f32::round(size.width() * scale) as u32,
        f32::round(size.height() * scale) as u32 
    );

    let opt = Pixmap::new(img_size.one, img_size.two);

    if opt.is_none() {
        return Result::Err("Unable to create pixmap!".into());
    }

    let mut pixmap: Pixmap = opt.unwrap();

    resvg::render(
        &tree, 
        Transform::from_scale(scale, scale), 
        &mut pixmap.as_mut()
    );

    return Result::Ok(Vec::from(pixmap.data()));
}