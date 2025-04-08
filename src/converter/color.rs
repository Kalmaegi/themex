use anyhow::{Result, anyhow};

pub fn is_dark_color(color: &str) -> bool {
    if let Ok((r, g, b, _)) = parse_color(color) {
        let brightness = 0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32;
        brightness < 128.0
    } else {
        false
    }
}

pub fn parse_color(color: &str) -> Result<(u8, u8, u8, u8)> {
    if !color.starts_with('#') {
        return Err(anyhow!("Unsupported color format: {}", color));
    }

    let color = color.trim_start_matches('#');

    match color.len() {
        // RGB
        3 => {
            let r = u8::from_str_radix(&color[0..1].repeat(2), 16)?;
            let g = u8::from_str_radix(&color[1..2].repeat(2), 16)?;
            let b = u8::from_str_radix(&color[2..3].repeat(2), 16)?;
            Ok((r, g, b, 255))
        }
        // RRGGBB
        6 => {
            let r = u8::from_str_radix(&color[0..2], 16)?;
            let g = u8::from_str_radix(&color[2..4], 16)?;
            let b = u8::from_str_radix(&color[4..6], 16)?;
            Ok((r, g, b, 255))
        }
        // RRGGBBAA
        8 => {
            let r = u8::from_str_radix(&color[0..2], 16)?;
            let g = u8::from_str_radix(&color[2..4], 16)?;
            let b = u8::from_str_radix(&color[4..6], 16)?;
            let a = u8::from_str_radix(&color[6..8], 16)?;
            Ok((r, g, b, a))
        }
        _ => Err(anyhow!("Invalid color format: #{}", color)),
    }
}

pub fn normalize_color(fg: &str, bg: &str) -> Result<String> {
    println!("will normalize color: fg: {}, bg: {}", fg, bg);
    let (fg_r, fg_g, fg_b, fg_a) = parse_color(fg)?;
    let (bg_r, bg_g, bg_b, _) = parse_color(bg)?;

    let fg_alpha = fg_a as f32 / 255.0;
    let fg_inv_alpha = 1.0 - fg_alpha;

    let r = (fg_alpha * fg_r as f32 + fg_inv_alpha * bg_r as f32).round() as u8;
    let g = (fg_alpha * fg_g as f32 + fg_inv_alpha * bg_g as f32).round() as u8;
    let b = (fg_alpha * fg_b as f32 + fg_inv_alpha * bg_b as f32).round() as u8;

    Ok(format!("#{:02x}{:02x}{:02x}", r, g, b))
}

fn rgb_to_hsl(color: &str) -> Result<(f32, f32, f32)> {
    let (r, g, b, _) = parse_color(color)?;

    let r_norm = r as f32 / 255.0;
    let g_norm = g as f32 / 255.0;
    let b_norm = b as f32 / 255.0;

    let max = r_norm.max(g_norm).max(b_norm);
    let min = r_norm.min(g_norm).min(b_norm);
    let delta = max - min;

    let lightness = (max + min) / 2.0;

    let saturation = if delta == 0.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * lightness - 1.0).abs())
    };

    let hue = if delta == 0.0 {
        0.0
    } else if max == r_norm {
        60.0 * (((g_norm - b_norm) / delta) % 6.0)
    } else if max == g_norm {
        60.0 * ((b_norm - r_norm) / delta + 2.0)
    } else {
        60.0 * ((r_norm - g_norm) / delta + 4.0)
    };

    Ok((hue, saturation, lightness))
}
