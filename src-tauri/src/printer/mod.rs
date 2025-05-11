use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use barcoders::generators::image::Image as BarcodeImage;
use barcoders::sym::code39::Code39;
use printpdf::*;
use serde::{Deserialize, Serialize};
use tauri::State;

use crate::db::Curd;
use crate::db::printer::get_settled_printer;
use crate::dict_data::DictData;
use crate::drying_rack::DryingRack;
use crate::error::Result;
use crate::error::{Error, ErrorKind};
use crate::local_users::LocalUser;
use crate::order_clothes::OrderCloth;
use crate::orders::Order;
use crate::state::AppState;
use crate::tags::Tag;

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
    let printer_configuration = get_settled_printer(state, "business".to_string())
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

// 生成小票PDF
async fn gen_receipt_pdf(
    pool: &sqlx::Pool<sqlx::Sqlite>,
    store: LocalUser,
    order: PrintReceiptReq,
) -> Result<String> {
    // 更精确的动态高度计算
    let base_height = 80.0; // 基础内容高度（单位mm），适当加大
    let font_size = 8.0;
    let line_gap = 5.0; // 行间距缩小
    let mut detail_lines = 0.0;

    // 计算每行能容纳的字符数（考虑中文字符宽度）
    let chars_per_line = ((WIDTH - 8.0) / (font_size * 0.6)) as usize;
    let store_name = store.store_name.unwrap_or_default();

    for cloth in &order.clothes {
        // 名称/颜色/挂号一行，编码一行
        detail_lines += 2.0;

        // 计算瑕疵标签需要的行数
        let mut flaw_names = Vec::new();
        if let Some(flaw) = &cloth.clothing_flaw {
            for flaw_id in flaw.split(',') {
                if let Ok(id) = flaw_id.trim().parse::<i64>() {
                    if let Ok(Some(flaw_tag)) = Tag::get_by_id(pool, id).await {
                        if let Some(name) = flaw_tag.tag_name {
                            flaw_names.push(name);
                        }
                    }
                }
            }
        }

        // 计算瑕疵标签需要的行数
        let flaw_text = format!("瑕疵: {}", flaw_names.join("、"));
        let flaw_lines = (flaw_text.chars().count() as f32 / chars_per_line as f32).ceil();
        detail_lines += flaw_lines;

        // 服务类型/服务要求/洗护价各一行（有则加）
        if cloth.service_type.is_some() {
            detail_lines += 1.0;
        }
        if cloth.service_requirement.is_some() {
            detail_lines += 1.0;
        }
        detail_lines += 1.0; // 洗护价
    }

    let detail_height = detail_lines * line_gap;
    let height = base_height + detail_height;
    let width = 58.0;
    let mut y = height - 8.0;

    // 预先获取所有需要的数据
    let mut cloth_details = Vec::new();
    for cloth in &order.clothes {
        let color = cloth.clothing_color.unwrap_or_default();
        let color = Tag::get_by_id(pool, color)
            .await?
            .unwrap_or_default()
            .tag_name
            .unwrap_or_default();
        let service_type = cloth.service_type.as_deref().unwrap_or("");
        let st_label = DictData::select_dict_label(pool, "sys_service_type", service_type)
            .await?
            .unwrap_or(service_type.to_string());
        let service_requirement = cloth.service_requirement.as_deref().unwrap_or("");
        let sr_label =
            DictData::select_dict_label(pool, "sys_service_requirement", service_requirement)
                .await?
                .unwrap_or(service_requirement.to_string());

        // 获取瑕疵名称
        let mut flaw_names = Vec::new();
        if let Some(flaw) = &cloth.clothing_flaw {
            for flaw_id in flaw.split(',') {
                if let Ok(id) = flaw_id.trim().parse::<i64>() {
                    if let Ok(Some(flaw_tag)) = Tag::get_by_id(pool, id).await {
                        if let Some(name) = flaw_tag.tag_name {
                            flaw_names.push(name);
                        }
                    }
                }
            }
        }

        // 获取衣挂信息
        let hanger_info = if cloth.hanger_name.is_none()
            || cloth.hanger_name.as_deref().unwrap_or("").is_empty()
        {
            if let Some(location_code) = cloth.hang_location_code {
                // 根据location_code查询衣挂信息
                if let Ok(Some(rack)) = DryingRack::get_by_id(pool, location_code).await {
                    rack.name.unwrap_or_default()
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            cloth.hanger_name.as_deref().unwrap_or("").to_string()
        };

        cloth_details.push((color, st_label, sr_label, flaw_names, hanger_info));
    }

    // 创建PDF文档
    let (doc, page1, layer1) = PdfDocument::new("Receipt", Mm(width), Mm(height), "Layer 1");
    let current_layer = doc.get_page(page1).get_layer(layer1);
    let font = doc.add_external_font(File::open("MSYH.TTC")?)?;

    // 店名居中
    let store_name_width = store_name.chars().count() as f32 * font_size * 0.6;
    let store_name_x = (width - store_name_width) / 2.0;
    current_layer.use_text(
        store_name,
        font_size + 2.0,
        Mm(store_name_x.max(0.0)),
        Mm(y),
        &font,
    );
    y -= line_gap + 2.0;

    // 订单号
    let order_number = order.order.order_number.as_deref().unwrap_or("");
    current_layer.use_text(
        format!("订单号: {}", order_number),
        font_size,
        Mm(4.0),
        Mm(y),
        &font,
    );
    y -= line_gap;
    // 打印时间
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    current_layer.use_text(
        format!("打印时间: {}", now),
        font_size,
        Mm(4.0),
        Mm(y),
        &font,
    );
    y -= line_gap;

    // 条形码（宽度80%，居中）
    if !order_number.is_empty() {
        gen_img(order_number)?;
        let file_name = format!("{}.png", order_number);
        let mut image_file = File::open(&file_name)?;
        let image = Image::try_from(image_crate::codecs::png::PngDecoder::new(&mut image_file)?)?;
        let barcode_width = width * 0.8;
        let barcode_x = (width - barcode_width) / 2.0;
        image.add_to_layer(
            current_layer.clone(),
            ImageTransform {
                translate_x: Some(Mm(barcode_x)),
                translate_y: Some(Mm(y - 10.0)),
                scale_x: Some(2.0),
                scale_y: Some(2.0),
                ..Default::default()
            },
        );
        y -= 3.0;
        std::fs::remove_file(file_name)?;
    }

    // 分割线
    current_layer.use_text(
        "----------------------------------------",
        font_size,
        Mm(2.0),
        Mm(y),
        &font,
    );
    y -= line_gap;

    // 表头：名称 颜色 衣挂号，均匀分布
    let col_width = width / 3.0;
    current_layer.use_text("名称", font_size, Mm(4.0), Mm(y), &font);
    current_layer.use_text("颜色", font_size, Mm(4.0 + col_width), Mm(y), &font);
    current_layer.use_text("衣挂号", font_size, Mm(4.0 + col_width * 2.0), Mm(y), &font);
    y -= line_gap;
    current_layer.use_text(
        "----------------------------------------",
        font_size,
        Mm(2.0),
        Mm(y),
        &font,
    );
    y -= line_gap;

    let total_count = order.clothes.len();

    // 衣物明细
    for (cloth, (color, st_label, sr_label, flaw_names, hanger_info)) in
        order.clothes.iter().zip(cloth_details)
    {
        // 名称 颜色 衣挂号
        let mut name = cloth
            .cloth_info
            .as_ref()
            .and_then(|info| info.title.as_deref())
            .unwrap_or("未知")
            .to_string();
        if name.chars().count() > 6 {
            name = name.chars().take(6).collect();
        }
        let hanger = format!(
            "{} - {}",
            hanger_info,
            cloth.hanger_number.unwrap_or_default()
        );
        current_layer.use_text(&name, font_size, Mm(4.0), Mm(y), &font);
        current_layer.use_text(&color, font_size, Mm(4.0 + col_width), Mm(y), &font);
        current_layer.use_text(hanger, font_size, Mm(col_width * 2.0), Mm(y), &font);
        y -= line_gap;
        // 编码
        let code = cloth.hang_cloth_code.as_deref().unwrap_or("");
        current_layer.use_text(
            format!("衣物编码: {}", code),
            font_size,
            Mm(4.0),
            Mm(y),
            &font,
        );
        y -= line_gap;
        // 瑕疵
        if !flaw_names.is_empty() {
            let flaw_text = format!("瑕疵: {}", flaw_names.join("、"));
            let mut chars: Vec<char> = flaw_text.chars().collect();

            while !chars.is_empty() {
                let line_end = if chars.len() > chars_per_line {
                    // 找到最后一个完整的标签位置
                    let mut last_comma = 0;
                    for (i, &c) in chars.iter().enumerate() {
                        if i >= chars_per_line {
                            break;
                        }
                        if c == '、' {
                            last_comma = i;
                        }
                    }
                    if last_comma > 0 {
                        last_comma + 1
                    } else {
                        chars_per_line
                    }
                } else {
                    chars.len()
                };

                let line: String = chars.drain(..line_end).collect();
                current_layer.use_text(&line, font_size, Mm(4.0), Mm(y), &font);
                y -= line_gap;
            }
        }
        // 服务类型
        if !st_label.is_empty() {
            current_layer.use_text(
                format!("服务类型: {}", st_label),
                font_size,
                Mm(4.0),
                Mm(y),
                &font,
            );
            y -= line_gap;
        }
        // 服务要求
        if !sr_label.is_empty() {
            current_layer.use_text(
                format!("服务要求: {}", sr_label),
                font_size,
                Mm(4.0),
                Mm(y),
                &font,
            );
            y -= line_gap;
        }
        // 洗护价
        let price = cloth
            .price_value
            .map(|p| format!("洗护价: ¥{:.2}", p))
            .unwrap_or("洗护价: ".to_string());
        current_layer.use_text(price, font_size, Mm(4.0), Mm(y), &font);
        y -= line_gap;
        current_layer.use_text(
            "----------------------------------------",
            font_size,
            Mm(2.0),
            Mm(y),
            &font,
        );
        y -= line_gap;
    }

    // 总金额、总件数
    let total_amount_text = format!("总金额: ¥{:.2}", order.mount);
    let total_count_text = format!("总件数:{}", total_count);

    // 计算总件数文本的宽度
    let total_count_width = total_count_text.chars().count() as f32 * font_size * 0.6;

    // 计算总件数的起始位置（总宽度减去总件数文本宽度，再减去一些边距）
    let total_count_x = width - total_count_width - 4.0;

    // 打印总金额
    current_layer.use_text(total_amount_text, font_size, Mm(4.0), Mm(y), &font);

    // 打印总件数
    current_layer.use_text(total_count_text, font_size, Mm(total_count_x), Mm(y), &font);
    y -= line_gap;

    current_layer.use_text(
        format!(
            "付款方式: {}",
            order.payment_method.as_deref().unwrap_or("未知")
        ),
        font_size,
        Mm(4.0),
        Mm(y),
        &font,
    );
    y -= line_gap;

    // 客户信息
    let client_name = order.order.nick_name.as_deref().unwrap_or("");
    let client_phone = order.order.phonenumber.as_deref().unwrap_or("");
    current_layer.use_text(
        format!("客户: {}", client_name),
        font_size,
        Mm(4.0),
        Mm(y),
        &font,
    );
    y -= line_gap;
    current_layer.use_text(
        format!("电话: {}", client_phone),
        font_size,
        Mm(4.0),
        Mm(y),
        &font,
    );

    y -= line_gap;
    current_layer.use_text(
        "----------------------------------------",
        font_size,
        Mm(2.0),
        Mm(y),
        &font,
    );

    y -= line_gap;

    let store_phone = store.owner_phone.as_deref().unwrap_or("");
    current_layer.use_text(
        format!("服务热线: {}", store_phone),
        font_size,
        Mm(4.0),
        Mm(y),
        &font,
    );

    // 保存PDF
    let pdf_file_name = format!("receipt_{}.pdf", order_number);
    doc.save(&mut BufWriter::new(File::create(&pdf_file_name)?))?;
    Ok(pdf_file_name)
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrintReceiptReq {
    pub order: Order,
    pub mount: f64,
    pub payment_method: Option<String>,
    pub clothes: Vec<OrderCloth>,
}

#[tauri::command]
pub async fn print_receipt(state: State<'_, AppState>, order: PrintReceiptReq) -> Result<()> {
    let store = state.get_user_info().await.ok_or(Error::unauthorized())?;
    let pool = &state.pool;
    // 获取receipt类型打印机
    let printer_configuration =
        crate::db::printer::get_settled_printer(state.clone(), "receipt".to_string())
            .await?
            .ok_or(Error::with_kind(ErrorKind::PrinterNotSet))?;
    let printer = printers::get_printer_by_name(&printer_configuration.name)
        .ok_or(Error::with_kind(ErrorKind::PrinterNotFound))?;
    // 生成小票pdf
    let file_name = gen_receipt_pdf(pool, store, order).await?;
    tracing::debug!("print receipt file: {}", file_name);
    // 打印
    printer
        .print_file(&file_name, None)
        .map_err(|e| Error::with_details(ErrorKind::PrintError, e))?;
    tracing::debug!("print complete, deleting file: {}", file_name);
    // std::fs::remove_file(file_name)?;
    Ok(())
}
