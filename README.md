 🧾 Student Report Card Generator (Rust CLI App)

This is a Rust-based console application that generates clean, PDF report cards for students based on their total marks and number of subjects.

 📌 Features

- Accepts student name, total marks, and subject count
- Calculates average and assigns grade:
  - A: 90+
  - B: 75–89
  - C: 60–74
  - D: <60
- Generates beautifully formatted PDF reports in `results/` folder
- Allows entering multiple students in a single run

 🚀 How to Run

1. Clone the repo:
   ```bash
   git clone https://github.com/<your-username>/student_report_card.git
   cd student_report_card
2. Run the app:

   cargo run
Enter student details when prompted. PDFs will appear in the results/ folder.

📁 Output Example
PDF file:

results/John_report.pdf
