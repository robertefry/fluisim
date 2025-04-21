
use std::process::Command;
use std::path::PathBuf;

fn main() {
    let builder = Builder::new();

    if let Err(error) = (|| -> Result<(), String> {
        builder.whitepaper_compile()?;
        builder.whitepaper_export()?;
        Ok(())
    })() {
        eprintln!("Failed to build the whitepaper: {}", error);
    }
}

struct Builder {
    pub module_name: String,
    pub module_manifest: PathBuf,
    pub out_dir: PathBuf,
}

impl Builder {

    const DOCKER_DIR: &'static str = "/workdir";
    const DOCKER_OUT: &'static str = "/outdir";

    const TEXLIVE_IMG: &'static str = "texlive/texlive";
    const TEXLIVE_CMD: &'static str = "pdflatex";

    const WHITEPAPER_IN:  &'static str = "main.tex";
    const WHITEPAPER_OUT: &'static str = "main.pdf";

    fn new() -> Self {
        Self {
            module_name: std::env::var("CARGO_PKG_NAME").unwrap(),
            module_manifest: PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap()),
            out_dir: PathBuf::from(std::env::var("OUT_DIR").unwrap()),
        }
    }

    fn whitepaper_compile(&self) -> Result<(), String> {

        let whitepaper_dir = self.module_manifest.join("whitepaper");
        let whitepaper_out = self.out_dir.as_path();

        let status = Command::new("docker")
            .arg("run").arg("--rm")
            .arg("-v").arg(format!("{}:{}", whitepaper_dir.display(), Self::DOCKER_DIR))
            .arg("-v").arg(format!("{}:{}", whitepaper_out.display(), Self::DOCKER_OUT))
            .arg("-w").arg(Self::DOCKER_DIR)
            .arg(Self::TEXLIVE_IMG)
            .arg(Self::TEXLIVE_CMD)
            .arg("-output-directory").arg(Self::DOCKER_OUT)
            .arg(Self::WHITEPAPER_IN)
            .status()
            ;

        fn format_error(error: impl ToString) -> String {
            format!(
                "Failed to execute {} in a {} docker container: {}"
                , Builder::TEXLIVE_CMD
                , Builder::TEXLIVE_IMG
                , error.to_string()
            )
        }

        match status {
            Ok(status) if status.success() => Ok(()),
            Ok(error) => Err(format_error(error)),
            Err(error) => Err(format_error(error)),
        }
    }

    fn whitepaper_export(&self) -> Result<(), String> {

        let whitepaper_name = format!("{}.pdf", self.module_name);
        let whitepaper_pdf = self.module_manifest.join(whitepaper_name);

        let status = std::fs::rename(self.out_dir.join(Self::WHITEPAPER_OUT), &whitepaper_pdf);

        fn format_error(error: impl ToString) -> String {
            format!(
                "Failed to move the generated pdf file: {}"
                , error.to_string()
            )
        }

        match status {
            Ok(_) => Ok(()),
            Err(error) => Err(format_error(error)),
        }
    }
}
