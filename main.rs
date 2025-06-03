use std::io::{self, Write};

fn main() {
    // for_loop_examples()
    // while_loop_examples()
    calculator()
}

// For loop bo'yicha examplelar
fn for_loop_examples() {
    let n: i32 = 10;

    for i in 1..=n {
        println!("Hello, world! {i}");
    }

    for i in 0..n {
        if i % 2 == 0 {
            println!("Juft son: {i}");
        } else {
            // continue;
            // break;
            println!("Toq son: {i}");
        }
    }

    println!("Hello there!")
}

// While loop bo'yicha examplelar
fn while_loop_examples() {
    let mut n: i32 = 10;

    while n > 0 {
        // print!("{n}");
        println!("{n}");
        n -= 1;
    }

    // Qolgan uslublarni hammasini while ustiga hover qilganda juda tushunarli examplelarda ko'rsatib qo'yibdi
}

fn calculator() {
    let operators: [&str; 4] = ["+", "-", "*", "/"];

    fn get_operator_names(ops: &[&str; 4]) -> String {
        return ops.join(", "); // Pastdagi kommentlangan kodlar shu bir qatorgacha qisqartirildi
        // let mut joined = String::new();

        // for (idx, op) in ops.iter().enumerate() {
        //     if idx > 0 {
        //         joined.push_str(", ");
        //     }

        //     joined.push_str(op);
        // }

        // joined
    }

    print!(
        "Operator turini tanlang (Mavjud operatorlar: {}): ",
        get_operator_names(&operators)
    );
    io::stdout().flush().unwrap();

    let mut operation_type = String::new();
    io::stdin().read_line(&mut operation_type).unwrap();
    let operation_type = operation_type.trim();

    if !operators.contains(&operation_type) {
        println!("\nBunday operator mavjud emas, qaytadan urinib ko'ring!");
        return calculator();
    }

    fn get_operator_speachly_text(op_type: &str) -> &str {
        return match op_type {
            "+" => "qo'shmoqchi",
            "-" => "ayirmoqchi",
            "*" => "ko'paytirmoqchi",
            _ => "bo'lmoqchi",
        };
    }

    fn get_num_result(n1: i32, n2: i32, op_type: &str) -> i32 {
        return match op_type {
            "+" => n1 + n2,
            "-" => n1 - n2,
            "*" => n1 * n2,
            _ => n1 / n2,
        };
    }

    println!(
        "\nTanlovingiz: {}{}{}{}",
        operation_type,
        "\nEndi nechta sonni bir-biriga/dan ",
        get_operator_speachly_text(operation_type),
        " bo'lsangiz har birini bo'shliq orqali ajratib yozib chiqing (Faqat son yozishingiz kerak!)\n\n↓ Sonlarni kiriting ↓"
    );
    io::stdout().flush().unwrap();

    let mut action_numbers: String = String::new();
    io::stdin().read_line(&mut action_numbers).unwrap();
    let action_numbers = action_numbers.trim().split(" ");

    let mut result: i32 = 0;

    for (idx, str_num) in action_numbers.enumerate() {
        let num: i32 = str_num.parse().expect("Hammasi son bo'lishi majburiy edi!");

        if idx == 0 {
            result = num;
            continue;
        }

        result = get_num_result(result, num, operation_type);
    }

    println!("\nNatija: {result}");
}

// ----------------------------------------------------------------------------------------------------
// Qisqacha izoh:
// Demak shunda(Rustni JS/TSdan farqlab yozaveraman) o'zgaruvchilar bitta let va shu letni emutable/mutable qilib ishlatilar ekan. Default emutable mutable uchun mut deb ketish kerak letdan keyin, ya'ni o'zgarmas/o'zgaruvchan
// for while if else switch case lar ham deyarli js bilan bir xil faqat qavslar va kichik ui ko'rinishida farq qiladi
// Kutubxona import qilishlari cin/cout'lari c++'ga o'xshash, function va print qilishlar pythondek
//     io::stdout().flush().unwrap();
//         Bu scriptda bunda user terminalga input qilaolishi flush() bu inputga(terminalga) focus qaratilishi va error berib qolsa unwrap() ishlab ketadi ya'ni break qiladi
//     io::stdin().read_line(&mut action_numbers).unwrap();
//         Bunda user kiritgan valueni ushlab olib parametriga berilgan o'zgaruvchiga qiymatni berib yuboradi xato ketsa unwrap ishlidi yana
// action_numbers.enumerate() bu enumerate() massivni har birini aylanib index va value sifatida beradi.
// str_num.parse().expect(...) stringni parse qiladi numberga va invalid parse bo'lib qolsa expect ishlab ketib breake qivoradi
// prin!() ni o'zi yonma yon print qilib ketaveradi, println!() esa har bir print keyingi qatorga tushib qilinadi.
// Hullas juda o'xshash va qiziq
// ----------------------------------------------------------------------------------------------------

// ------------------------------
// !ESLATMA
// Keyingi example uchun gibrid kalkulat qilaman, ya'ni istalgancha inputga yozish mumkin bo'ladi va qiymatni dastur o'zi aniqlab hisoblashi kerak bo'ladi operatsiya belgilari va sonlarni tekshirib
// Misol: 20 + 50*10- 60 /70 % 80
// Eng qiyini operatsiya amallarini ketma-ketlikda bajarish logikasi(*/+-)
// Boshlanishiga oddiy ketma-ket, keyin operatorlar ketma-ketligida qilib ko'raman
// Qachon ? Noma'lum...
// ------------------------------
