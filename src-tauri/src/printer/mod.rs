use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use barcoders::generators::image::Image as BarcodeImage;
use barcoders::sym::code39::Code39;
use printpdf::*;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::printer::get_settled_printer;
use crate::error::Result;
use crate::error::{Error, ErrorKind};
use crate::state::AppState;

const FONT_SIZE: f32 = 10.0;
const VERTICAL_LINE_POSITION: f32 = 90.0;
const PADDING: f32 = 6.0;
const WIDTH: f32 = 180.0;
const HEIGHT: f32 = 35.0;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    cloth_name: String,
    #[serde(default)]
    cloth_color: i32,
    #[serde(default)]
    cloth_flaw: Vec<i32>,
    time: String,
    code: String,
    sum: i32,
    num: i32,
    client: Client,
    shelf: Shelf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Shelf {
    name: String,
    position: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Client {
    name: String,
    phone: String,
}

#[tauri::command]
pub async fn print(state: State<'_, AppState>, items: Vec<Item>) -> Result<()> {
    let store_name = state
        .get_user_info()
        .await
        .ok_or(Error::unauthorized())?
        .store_name
        .ok_or(Error::unauthorized())?;
    // 查询数据库，获取当前打印机的系统名称
    let printer_configuration = get_settled_printer(state)
        .await?
        .ok_or(Error::with_kind(ErrorKind::PrinterNotSet))?;
    let printer = printers::get_printer_by_name(&printer_configuration.name)
        .ok_or(Error::with_kind(ErrorKind::PrinterNotFound))?;

    for item in items {
        // 生成pdf文件
        let file_name = gen_pdf(&store_name, item)?;
        tracing::debug!("print file: {}", file_name);

        // 打印
        printer
            .print_file(&file_name, None)
            .map_err(|e| Error::with_details(ErrorKind::PrintError, e))?;

        tracing::debug!("print complete, deleting file: {}", file_name);

        // 删除pdf文件
        std::fs::remove_file(file_name)?;
    }

    Ok(())
}

// 生成条形码图片
fn gen_img(code: &str) -> Result<()> {
    let barcode = Code39::new(code)?;
    let png = BarcodeImage::png(80); // You must specify the height in pixels.
    let encoded = barcode.encode();

    // Image generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes.
    let bytes = png.generate(&encoded[..])?;

    // Which you can then save to disk.
    let file = File::create(&Path::new(&format!("{}.png", code)))?;
    let mut writer = BufWriter::new(file);
    writer.write(&bytes[..])?;
    Ok(())
}

fn gen_pdf(store_name: &str, item: Item) -> Result<String> {
    let (doc, page1, layer1) =
        PdfDocument::new("PDF_Document_title", Mm(WIDTH), Mm(HEIGHT), "Layer 1");

    // 生成条形码
    gen_img(&item.code)?;

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // 使用内置的 Helvetica 字体
    let font = doc.add_external_font(File::open("MSYH.TTC")?)?;

    // 在指定位置添加文字
    current_layer.use_text(
        store_name,
        FONT_SIZE,
        Mm(PADDING),
        Mm(HEIGHT - PADDING),
        &font,
    );

    // 设置其他位置的文字
    current_layer.use_text(&item.code, FONT_SIZE, Mm(PADDING), Mm(PADDING - 1.), &font);
    let mut line_position = HEIGHT - PADDING;
    current_layer.use_text(
        format!(
            "{} {} 总件数: {}:{}",
            item.cloth_name, item.cloth_color, item.sum, item.num
        ),
        FONT_SIZE,
        Mm(VERTICAL_LINE_POSITION),
        Mm(line_position),
        &font,
    );
    line_position = line_position - FONT_SIZE + 2.;
    current_layer.use_text(
        format!(
            "瑕疵: {}",
            item.cloth_flaw
                .iter()
                .map(|item| item.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        ),
        FONT_SIZE,
        Mm(VERTICAL_LINE_POSITION),
        Mm(line_position),
        &font,
    );
    line_position = line_position - FONT_SIZE + 2.;
    current_layer.use_text(
        format!(
            "收衣时间: {}  衣柜位置: {}-{}",
            item.time, item.shelf.name, item.shelf.position
        ),
        FONT_SIZE,
        Mm(VERTICAL_LINE_POSITION),
        Mm(line_position),
        &font,
    );
    current_layer.use_text(
        format!("客户: {} 电话: {}", item.client.name, item.client.phone),
        FONT_SIZE,
        Mm(VERTICAL_LINE_POSITION),
        Mm(PADDING - 1.),
        &font,
    );

    let file_name = format!("{}.png", item.code);
    let mut image_file = File::open(&file_name)?;
    let image = Image::try_from(image_crate::codecs::png::PngDecoder::new(&mut image_file)?)?;

    image.add_to_layer(
        current_layer.clone(),
        ImageTransform {
            translate_x: Some(Mm(PADDING)), // X 方向平移
            translate_y: Some(Mm(10.)),
            scale_x: Some(5.2),
            scale_y: Some(2.5),
            ..Default::default()
        },
    );
    let pdf_file_name = format!("{}.pdf", item.code);
    doc.save(&mut BufWriter::new(File::create(&pdf_file_name)?))?;
    // 删除 png
    std::fs::remove_file(file_name)?;
    Ok(pdf_file_name)
}
