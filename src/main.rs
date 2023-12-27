mod days;

use days::get_days;

fn main() {
    let ejercicios = get_days();

    for (idx, ejercicio) in ejercicios.iter().enumerate() {
        println!("Ejercicio {}: ", idx + 1);

        let parte_1 = ejercicio.resultado_parte_1();
        println!("\tParte 1: {}", parte_1);

        let parte_2 = ejercicio.resultado_parte_2();
        println!("\tParte 2: {}", parte_2);
    }
}
