use raylib::prelude::*;

fn main() {
    let image_width = 500;
    let image_height = 500;

    let mut new_image = Image::gen_image_color(
        image_width,
        image_height,
        Color::WHITE
    );

    // Centro de la araña
    let center_x = 250;
    let center_y = 250;

    // Cuerpo (abdomen) - círculo grande
    new_image.draw_circle(center_x, center_y + 20, 50, Color::BLACK);

    // Cabeza (cefalotórax) - círculo más pequeño
    new_image.draw_circle(center_x, center_y - 40, 30, Color::BLACK);

    // Patas - 4 de cada lado, cada una hecha de 2 líneas para simular la "quiebra" de las patas
    let leg_color = Color::BLACK;

    // Patas lado izquierdo
    new_image.draw_line(center_x - 20, center_y - 20, center_x - 100, center_y - 60, leg_color);
    new_image.draw_line(center_x - 100, center_y - 60, center_x - 160, center_y - 40, leg_color);

    new_image.draw_line(center_x - 20, center_y, center_x - 110, center_y - 10, leg_color);
    new_image.draw_line(center_x - 110, center_y - 10, center_x - 170, center_y + 10, leg_color);

    new_image.draw_line(center_x - 20, center_y + 20, center_x - 110, center_y + 30, leg_color);
    new_image.draw_line(center_x - 110, center_y + 30, center_x - 170, center_y + 60, leg_color);

    new_image.draw_line(center_x - 20, center_y + 40, center_x - 100, center_y + 80, leg_color);
    new_image.draw_line(center_x - 100, center_y + 80, center_x - 160, center_y + 100, leg_color);

    // Patas lado derecho (espejadas)
    new_image.draw_line(center_x + 20, center_y - 20, center_x + 100, center_y - 60, leg_color);
    new_image.draw_line(center_x + 100, center_y - 60, center_x + 160, center_y - 40, leg_color);

    new_image.draw_line(center_x + 20, center_y, center_x + 110, center_y - 10, leg_color);
    new_image.draw_line(center_x + 110, center_y - 10, center_x + 170, center_y + 10, leg_color);

    new_image.draw_line(center_x + 20, center_y + 20, center_x + 110, center_y + 30, leg_color);
    new_image.draw_line(center_x + 110, center_y + 30, center_x + 170, center_y + 60, leg_color);

    new_image.draw_line(center_x + 20, center_y + 40, center_x + 100, center_y + 80, leg_color);
    new_image.draw_line(center_x + 100, center_y + 80, center_x + 160, center_y + 100, leg_color);

    // Ojitos
    new_image.draw_circle(center_x - 10, center_y - 45, 4, Color::RED);
    new_image.draw_circle(center_x + 10, center_y - 45, 4, Color::RED);

    let output_file_name = "spider.png";
    new_image.export_image(output_file_name);

    println!("Image saved successfully as '{}'!", output_file_name);
}