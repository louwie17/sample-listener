extern crate cc;
extern crate glob;

fn main() {

    // let target = std::env::var("TARGET").unwrap();
    // if !(target.starts_with("arm-") || target.starts_with("armv7-")) {
    //     return;
    // }

    cc::Build::new()
        .files(
            glob::glob(&format!("../receiver-pine64/libs/rfm69/*.c"))
                .unwrap()
                .chain(glob::glob(&format!("../WiringNP/wiringPi/*.c"))
                    .unwrap())
                    .filter_map(|x| x.ok())
        )
        // .file("../receiver-pine64/libs/rfm69/rfm69.c")
        // https://github.com/louwie17/receiver-pine64
        .include(format!("../receiver-pine64/libs/rfm69/"))
        // https://github.com/friendlyarm/WiringNP
        .include(format!("../WiringNP/wiringPi/"))
        .static_flag(true)
        .compile("rfm69");
}
