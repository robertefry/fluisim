
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let builder = Builder::new();
    builder.build_whitepaper();
}

struct Builder {
    pub module_name: String,
    pub module_manifest: PathBuf,
    pub out_dir: PathBuf,
}

impl Builder {

    fn new() -> Self {
        Self {
            module_name: std::env::var("CARGO_PKG_NAME").unwrap(),
            module_manifest: PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()),
            out_dir: PathBuf::from(std::env::var("OUT_DIR").unwrap()),
        }
    }

    fn build_whitepaper(&self) {

        // Compile the whitepaper.

        let whitepaper_dir = self.module_manifest.join("whitepaper");
        let whitepaper_out = self.out_dir.as_path();

        let docker_dir     = "/workdir";
        let docker_out     = "/outdir";

        let texlive_img    = "texlive/texlive";
        let texlive_cmd    = "pdflatex";
        let texlive_in     = "main.tex";
        let texlive_out    = "main.pdf";

        let status = Command::new("docker")
            .arg("run").arg("--rm")
            .arg("-v").arg(format!("{}:{}", whitepaper_dir.display(), docker_dir))
            .arg("-v").arg(format!("{}:{}", whitepaper_out.display(), docker_out))
            .arg("-w").arg(docker_dir)
            .arg(texlive_img)
            .arg(texlive_cmd)
            .arg("-output-directory").arg(docker_out)
            .arg(texlive_in)
            .status()
            ;

        match status {
            Ok(status) if status.success() => { /* noop */ }
            _ => { panic!("Failed to execute {} in a {} docker container.", texlive_cmd, texlive_img) }
        }

        // Move the whitepaper pdf to the module manifest directory.

        let whitepaper_name = format!("{}.pdf", self.module_name);
        let whitepaper_pdf = self.module_manifest.join(whitepaper_name);

        std::fs::rename(self.out_dir.join(texlive_out), &whitepaper_pdf)
            .expect("Failed to move the generated pdf file.");
    }
}
