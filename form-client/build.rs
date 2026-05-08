fn main() {
    let build_date = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
    println!("cargo:rustc-env=BUILD_DATE={build_date}");
}
