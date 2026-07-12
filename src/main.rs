use raylib::prelude::*;

fn main() {
    let image_width = 600;
    let image_height = 600;
    let mut new_image = Image::gen_image_color(image_width, image_height, Color::WHITE);

    let center_x = 300;
    let center_y = 320;
    let body_color = Color::new(25, 20, 20, 255);
    let leg_color = Color::new(15, 12, 12, 255);

    // ---------- ABDOMEN (ovalado, simulado con varios circulos superpuestos) ----------
    for i in 0..8 {
        let t = i as f32 / 7.0;
        let y_off = (t - 0.5) * 90.0;
        let radius = 55.0 - (t - 0.5).abs() * 50.0;
        new_image.draw_circle(center_x, center_y + 40 + y_off as i32, radius as i32, body_color);
    }

    // Patron/manchas en el abdomen
    let pattern_color = Color::new(90, 60, 40, 255);
    new_image.draw_circle(center_x, center_y + 15, 10, pattern_color);
    new_image.draw_circle(center_x - 12, center_y + 35, 8, pattern_color);
    new_image.draw_circle(center_x + 12, center_y + 35, 8, pattern_color);
    new_image.draw_circle(center_x, center_y + 55, 9, pattern_color);
    new_image.draw_circle(center_x - 10, center_y + 75, 7, pattern_color);
    new_image.draw_circle(center_x + 10, center_y + 75, 7, pattern_color);

    // ---------- CEFALOTORAX (cabeza/torso delantero) ----------
    new_image.draw_circle(center_x, center_y - 45, 32, body_color);
    new_image.draw_circle(center_x, center_y - 20, 25, body_color); // conexion con abdomen

    // ---------- COLMILLOS (quelíceros) ----------
    new_image.draw_line(center_x - 8, center_y - 20, center_x - 14, center_y - 2, leg_color);
    new_image.draw_line(center_x + 8, center_y - 20, center_x + 14, center_y - 2, leg_color);
    new_image.draw_circle(center_x - 14, center_y - 2, 3, leg_color);
    new_image.draw_circle(center_x + 14, center_y - 2, 3, leg_color);

    // ---------- OJOS (8 ojos tipicos de arana, en 2 filas) ----------
    let eye_color = Color::new(150, 0, 0, 255);
    // Fila delantera (4 ojos pequeños)
    new_image.draw_circle(center_x - 18, center_y - 55, 3, eye_color);
    new_image.draw_circle(center_x - 6, center_y - 58, 3, eye_color);
    new_image.draw_circle(center_x + 6, center_y - 58, 3, eye_color);
    new_image.draw_circle(center_x + 18, center_y - 55, 3, eye_color);
    // Fila trasera (4 ojos mas grandes)
    new_image.draw_circle(center_x - 14, center_y - 62, 4, eye_color);
    new_image.draw_circle(center_x - 4, center_y - 65, 4, eye_color);
    new_image.draw_circle(center_x + 4, center_y - 65, 4, eye_color);
    new_image.draw_circle(center_x + 14, center_y - 62, 4, eye_color);

    // ---------- FUNCION PARA PATAS ARTICULADAS (3 segmentos: coxa, femur, tarso) ----------
    // side: -1.0 = izquierda, 1.0 = derecha
    // base_y: altura de donde nace la pata en el cuerpo
    // spread: que tan abiertas van las patas
    let draw_leg = |img: &mut Image, side: f32, base_y: i32, angle_deg: f32, length_mult: f32| {
        let angle = angle_deg.to_radians();
        let start_x = center_x + (side * 18.0) as i32;
        let start_y = base_y;

        // Segmento 1: coxa -> rodilla 1 (sube y se abre)
        let seg1_len = 70.0 * length_mult;
        let mid1_x = start_x + (side * angle.cos() * seg1_len) as i32;
        let mid1_y = start_y - (angle.sin() * seg1_len * 0.5) as i32;

        // Segmento 2: rodilla 1 -> rodilla 2 (baja un poco, se estira)
        let seg2_len = 90.0 * length_mult;
        let mid2_x = mid1_x + (side * angle.cos() * seg2_len * 0.9) as i32;
        let mid2_y = mid1_y + (seg2_len * 0.45) as i32;

        // Segmento 3: rodilla 2 -> punta (baja mas, mas fino)
        let seg3_len = 60.0 * length_mult;
        let end_x = mid2_x + (side * angle.cos() * seg3_len * 0.6) as i32;
        let end_y = mid2_y + (seg3_len * 0.9) as i32;

        img.draw_line(start_x, start_y, mid1_x, mid1_y, leg_color);
        img.draw_line(mid1_x, mid1_y, mid1_x, mid1_y, leg_color); // relleno junta
        img.draw_line(mid1_x, mid1_y, mid2_x, mid2_y, leg_color);
        img.draw_line(mid2_x, mid2_y, end_x, end_y, leg_color);

        // Articulaciones (puntitos en las rodillas para dar volumen)
        img.draw_circle(mid1_x, mid1_y, 3, leg_color);
        img.draw_circle(mid2_x, mid2_y, 2, leg_color);
    };

    // 4 patas de cada lado, saliendo del cefalotorax a distintos angulos
    let angles = [35.0, 15.0, -10.0, -35.0];
    let base_ys = [center_y - 35, center_y - 25, center_y - 15, center_y - 5];
    let lengths = [1.0, 1.05, 1.05, 0.95];

    for i in 0..4 {
        draw_leg(&mut new_image, -1.0, base_ys[i], angles[i], lengths[i]);
        draw_leg(&mut new_image, 1.0, base_ys[i], angles[i], lengths[i]);
    }

    // ---------- PELITOS (detalles finos en las patas, opcional) ----------
    for i in 0..6 {
        let x = center_x - 130 + i * 45;
        new_image.draw_line(x, center_y + 90, x - 3, center_y + 100, leg_color);
    }

    let output_file_name = "spider_realista.png";
    new_image.export_image(output_file_name);
    println!("Image saved successfully as '{}'!", output_file_name);
}