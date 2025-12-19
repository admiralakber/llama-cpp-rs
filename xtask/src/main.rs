use anyhow::Result;
use clap::{Parser, Subcommand};
use xshell::{cmd, Shell};

#[derive(Parser)]
#[command(name = "xtask")]
#[command(about = "Development tasks for the llama-cpp-rs meoslabs fork", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify the repository state (submodules, build flags)
    Verify,
    /// Bump llama.cpp submodule to a specific tag or commit
    BumpLlama {
        /// The tag or commit hash to checkout
        #[arg(short, long)]
        target: String,
    },
    /// Show help/maintenance guide summary
    Guide,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    match cli.command {
        Commands::Verify => verify(&sh),
        Commands::BumpLlama { target } => bump_llama(&sh, &target),
        Commands::Guide => {
            println!("See docs/MAINTAINER_GUIDE.md for detailed instructions.");
            Ok(())
        }
    }
}

fn verify(sh: &Shell) -> Result<()> {
    println!("🔍 Verifying repository state...");
    
    // Check submodule status
    if !sh.path_exists("llama-cpp-sys-2/llama.cpp/CMakeLists.txt") {
        anyhow::bail!("llama.cpp submodule not initialized. Run `git submodule update --init --recursive`.");
    }

    // Check for our custom patches in build.rs
    let build_rs = sh.read_file("llama-cpp-sys-2/build.rs")?;
    if !build_rs.contains("tools/mtmd") {
        eprintln!("⚠️  Warning: llama-cpp-sys-2/build.rs does not seem to contain MTMD include patches.");
    } else {
        println!("✅ build.rs patches look present.");
    }

    // Check for server subdirectory in CMakeLists
    let tools_cmake = sh.read_file("llama-cpp-sys-2/llama.cpp/tools/CMakeLists.txt")?;
    if !tools_cmake.contains("add_subdirectory(server)") {
         eprintln!("⚠️  Warning: llama.cpp/tools/CMakeLists.txt might be missing 'add_subdirectory(server)'.");
    } else {
        println!("✅ CMakeLists.txt patches look present.");
    }

    println!("✅ Verification complete.");
    Ok(())
}

fn bump_llama(sh: &Shell, target: &str) -> Result<()> {
    println!("🚀 Bumping llama.cpp to {}", target);
    let llama_dir = "llama-cpp-sys-2/llama.cpp";
    
    let _push = sh.push_dir(llama_dir);
    cmd!(sh, "git fetch origin").run()?;
    cmd!(sh, "git checkout {target}").run()?;
    drop(_push);

    println!("✅ Checked out {}. Remember to verify patches using `cargo xtask verify` and re-apply if lost.", target);
    Ok(())
}

