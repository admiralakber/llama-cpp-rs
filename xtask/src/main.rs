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
    /// Automate upstream synchronization (The Drift Defender)
    ///
    /// Fetches upstream, creates a branch, merges, and validates patches.
    Sync,
    /// Show help/maintenance guide summary
    Guide,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let sh = Shell::new()?;

    match cli.command {
        Commands::Verify => verify(&sh),
        Commands::BumpLlama { target } => bump_llama(&sh, &target),
        Commands::Sync => sync_upstream(&sh),
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

    let mut ok = true;

    // Check for our custom patches in build.rs
    let build_rs = sh.read_file("llama-cpp-sys-2/build.rs")?;
    if !build_rs.contains("tools/mtmd") {
        eprintln!("❌ Error: llama-cpp-sys-2/build.rs lost MTMD include patches.");
        ok = false;
    } else {
        println!("✅ build.rs patches present.");
    }

    // Check for server subdirectory in CMakeLists
    let tools_cmake = sh.read_file("llama-cpp-sys-2/llama.cpp/tools/CMakeLists.txt")?;
    if !tools_cmake.contains("add_subdirectory(server)") {
         eprintln!("❌ Error: llama.cpp/tools/CMakeLists.txt lost 'add_subdirectory(server)'.");
         ok = false;
    } else {
        println!("✅ CMakeLists.txt patches present.");
    }

    if !ok {
        anyhow::bail!("Verification failed! Critical patches are missing. Please re-apply them.");
    }

    println!("✅ Verification complete. Repository is healthy.");
    Ok(())
}

fn sync_upstream(sh: &Shell) -> Result<()> {
    println!("🛡️  Initiating Drift Defender protocol...");

    // 1. Check for clean working directory
    let status = cmd!(sh, "git status --porcelain").read()?;
    if !status.is_empty() {
        anyhow::bail!("Working directory is dirty. Please commit or stash changes before syncing.");
    }

    // 2. Ensure upstream remote exists
    let remotes = cmd!(sh, "git remote").read()?;
    if !remotes.contains("upstream") {
        println!("➕ Adding upstream remote (utilityai/llama-cpp-rs)...");
        cmd!(sh, "git remote add upstream https://github.com/utilityai/llama-cpp-rs.git").run()?;
    }

    // 3. Fetch upstream
    println!("⬇️  Fetching upstream...");
    cmd!(sh, "git fetch upstream").run()?;

    // 4. Create sync branch
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    let branch_name = format!("chore/sync-upstream-{}", timestamp);
    println!("🌿 Creating branch {}...", branch_name);
    cmd!(sh, "git checkout -b {branch_name}").run()?;

    // 5. Merge upstream/main
    println!("🔀 Merging upstream/main...");
    if let Err(e) = cmd!(sh, "git merge upstream/main").run() {
        eprintln!("⚠️  Merge conflict detected: {}", e);
        eprintln!("🛑 ACTION REQUIRED: Resolve conflicts manually.");
        eprintln!("   1. Fix conflicts in affected files.");
        eprintln!("   2. Ensure build.rs and CMakeLists.txt retain Meoslabs patches.");
        eprintln!("   3. Run `cargo xtask verify` to check your work.");
        eprintln!("   4. Commit the merge.");
        return Ok(());
    }

    // 6. Update submodules (in case upstream bumped llama.cpp)
    println!("📦 Updating submodules...");
    cmd!(sh, "git submodule update --init --recursive").run()?;

    // 7. Verify
    println!("🕵️  Running post-merge verification...");
    if let Err(e) = verify(sh) {
        eprintln!("❌ Sync verification failed: {}", e);
        eprintln!("🛑 ACTION REQUIRED: Re-apply missing patches manually.");
    } else {
        println!("✨ Sync successful! Branch {} is ready for review.", branch_name);
        println!("   Run `git push -u origin {}` to share.", branch_name);
    }

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

