use printpdf::*;
use std::fs::File;
use std::io::BufWriter;
use std::io;
use chrono::Local;

fn calculate_average(total_marks: u32, num_subjects: u32) -> f32 {
    total_marks as f32 / num_subjects as f32
}

fn get_grade(average: f32) -> &'static str {
    match average {
        x if x >= 90.0 => "A",
        x if x >= 75.0 => "B",
        x if x >= 60.0 => "C",
        _ => "D",
    }
}

fn generate_pdf(name: &str, average: f32, grade: &str) {
    let (doc, page1, layer1) =
        PdfDocument::new("Student Report Card", Mm(210.0), Mm(297.0), "Layer 1");
    let layer = doc.get_page(page1).get_layer(layer1);

    let title_font = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .unwrap();
    let body_font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .unwrap();

    // Title
    layer.use_text("🎓 Student Report Card 🎓", 24.0, Mm(55.0), Mm(250.0), &title_font);

    // Decorative line
    layer.use_text("----------------------------------------", 14.0, Mm(20.0), Mm(240.0), &body_font);

    // Body content
    let lines = vec![
        format!("📛 Name           : {}", name),
        format!("📊 Average Marks : {:.2}", average),
        format!("🏅 Grade          : {}", grade),
        format!("🗓️  Date           : {}", Local::now().format("%Y-%m-%d")),
    ];

    let mut y_position = 220.0;
    for line in lines {
        layer.use_text(line, 16.0, Mm(30.0), Mm(y_position), &body_font);
        y_position -= 15.0;
    }

    // Save the PDF
    let file_path = format!("results/{}_report.pdf", name.replace(" ", "_"));
    let file = File::create(&file_path).unwrap();
    doc.save(&mut BufWriter::new(file)).unwrap();

    println!(
        "✅ PDF generated at: file://{}\n",
        std::fs::canonicalize(&file_path).unwrap().display()
    );
}

fn main() {
    loop {
        let mut name = String::new();
        let mut total_marks = String::new();
        let mut num_subjects = String::new();

        println!("Enter student name:");
        io::stdin().read_line(&mut name).unwrap();

        println!("Enter total marks:");
        io::stdin().read_line(&mut total_marks).unwrap();

        println!("Enter number of subjects:");
        io::stdin().read_line(&mut num_subjects).unwrap();

        let total_marks: u32 = match total_marks.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("❌ Invalid total marks.");
                continue;
            }
        };

        let num_subjects: u32 = match num_subjects.trim().parse() {
            Ok(val) => val,
            Err(_) => {
                println!("❌ Invalid number of subjects.");
                continue;
            }
        };

        let average = calculate_average(total_marks, num_subjects);
        let grade = get_grade(average);

        println!("\n--- Report Card ---");
        println!("Name: {}", name.trim());
        println!("Average Marks: {:.2}", average);
        println!("Grade: {}", grade);

        generate_pdf(name.trim(), average, grade);

        // Ask if user wants to enter another
        let mut again = String::new();
        println!("\nDo you want to enter another student? (y/n):");
        io::stdin().read_line(&mut again).unwrap();

        if again.trim().to_lowercase() != "y" {
            println!("👋 Exiting... All reports saved in /results.");
            break;
        }
    }
}

