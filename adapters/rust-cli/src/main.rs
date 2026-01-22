use std::process;

#[allow(unused_imports)]
use clap::{CommandFactory, Parser, Subcommand};
#[allow(unused_imports)]
use clap_complete::{generate, Shell};
use tracing::{error, info};

mod commands;
mod handlers;
mod converters;

#[cfg(test)]
mod integration_tests;

use commands::execute_command;

/// Rengine CLI - High-Performance GTA File Processing Tool 🦀
///
/// A fast, reliable Rust implementation for processing GTA game files.
/// Supports IMG archives, DFF models, TXD textures, COL collisions, IPL maps, and more.
///
/// EXAMPLES:
///     rengine-cli img info gta3.img                    # Show IMG archive info
///     rengine-cli dff convert model.dff model.obj      # Convert DFF to OBJ
///     rengine-cli batch analyze . --format dff         # Analyze all DFF files
///     rengine-cli ipl-to-godot map.ipl scene.tscn      # Convert IPL to Godot scene
///
/// For more help on any command, use: rengine-cli <command> --help
#[derive(Parser)]
#[command(name = "rengine-cli")]
#[command(name = "rengine-cli")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Suppress non-error output
    #[arg(short, long)]
    quiet: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value = "text")]
    format: OutputFormat,

    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Subcommand)]
enum ConvertCommands {
    /// Convert DFF model(s) to OBJ format
    #[command(name = "dff")]
    Dff {
        /// DFF file or directory containing DFF files
        file_or_dir: String,
        /// Output OBJ file path (ignored when input is directory)
        #[arg(short, long)]
        output: Option<String>,
        /// TXD file for materials (optional, single file mode only)
        #[arg(long)]
        txd: Option<String>,
        /// Disable loading of GTA SA CSV data for model names and TXD paths
        #[arg(long)]
        no_csv: bool,
        /// Maximum number of files to process (when input is directory)
        #[arg(long)]
        max_files: Option<usize>,
        /// Output directory for batch conversion (when input is directory)
        #[arg(long)]
        output_dir: Option<String>,
    },
    /// Convert TXD texture to materials
    #[command(name = "txd")]
    Txd {
        /// TXD file to process
        file: String,
        /// Output directory for textures and materials
        #[arg(short, long)]
        output: String,
        /// Output image format
        #[arg(short, long, value_enum, default_value = "png")]
        format: ImageFormat,
    },
}

#[derive(Subcommand)]
enum Commands {
    /// IMG archive operations
    Img {
        #[command(subcommand)]
        img_command: ImgCommands,
    },
    /// DFF model operations
    Dff {
        #[command(subcommand)]
        dff_command: DffCommands,
    },
    /// TXD texture operations
    Txd {
        #[command(subcommand)]
        txd_command: TxdCommands,
    },
    /// COL collision operations
    Col {
        #[command(subcommand)]
        col_command: ColCommands,
    },
    /// IPL map operations
    Ipl {
        #[command(subcommand)]
        ipl_command: IplCommands,
    },
    /// RenderWare chunk analysis
    Rw {
        #[command(subcommand)]
        rw_command: RwCommands,
    },
    /// Batch processing operations
    Batch {
        #[command(subcommand)]
        batch_command: BatchCommands,
    },
    /// File conversion operations
    #[command(name = "convert")]
    Convert {
        #[command(subcommand)]
        convert_command: ConvertCommands,
    },
    /// Generate shell completion scripts
    Completion {
        /// Shell to generate completion for
        #[arg(value_enum)]
        shell: Shell,
    },
    /// Interactive wizard for common operations
    #[command(name = "wizard")]
    Wizard,
    /// Convert IPL to Godot project/scene (alias for ipl-to-godot)
    #[command(name = "godot")]
    Godot {
        /// IPL file or directory containing IPL files
        input: String,
        /// Output directory for Godot project (when input is directory) or scene file (.tscn when input is file)
        #[arg(short, long)]
        output: String,
        /// Template scene file to extend
        #[arg(long)]
        template: Option<String>,
        /// Name for the Godot project
        #[arg(long, default_value = "IPLMap")]
        project_name: String,
        /// Include Rust GDExtension integration and demo scenes
        #[arg(long)]
        with_rust: bool,
        /// IDE files to use for model name lookup (auto-detected if not specified)
        #[arg(long, num_args = 1..)]
        ide_files: Option<Vec<String>>,
        /// Convert DFF models referenced in IPL to OBJ format for Godot
        #[arg(long)]
        convert_assets: bool,
        /// Directory containing DFF model files (used with --convert-assets)
        #[arg(long)]
        dff_path: Option<String>,
        /// Directory containing TXD texture files (used with --convert-assets)
        #[arg(long)]
        txd_path: Option<String>,
    },
    /// Legacy: Convert DFF model to OBJ format (use 'convert dff' instead)
    #[command(name = "dff-to-obj")]
    DffToObj {
        /// DFF file to convert
        file: String,
        /// Output OBJ file path
        #[arg(short, long)]
        output: String,
        /// TXD file for materials (optional)
        #[arg(long)]
        txd: Option<String>,
        /// Disable loading of GTA SA CSV data for model names and TXD paths
        #[arg(long)]
        no_csv: bool,
    },
    #[command(name = "txd-to-materials")]
    TxdToMaterials {
        /// TXD file to process
        file: String,
        /// Output directory for textures and materials
        #[arg(short, long)]
        output: String,
        /// Output image format
        #[arg(short, long, value_enum, default_value = "png")]
        format: ImageFormat,
    },
    #[command(name = "ipl-to-godot")]
    IplToGodot {
        /// IPL file or directory containing IPL files
        input: String,
        /// Output directory for Godot project (when input is directory) or scene file (.tscn when input is file)
        #[arg(short, long)]
        output: String,
        /// Template scene file to extend
        #[arg(long)]
        template: Option<String>,
        /// Name for the Godot project
        #[arg(long, default_value = "IPLMap")]
        project_name: String,
        /// Include Rust GDExtension integration and demo scenes
        #[arg(long)]
        with_rust: bool,
        /// IDE files to use for model name lookup (auto-detected if not specified)
        #[arg(long, num_args = 1..)]
        ide_files: Option<Vec<String>>,
        /// Convert DFF models referenced in IPL to OBJ format for Godot
        #[arg(long)]
        convert_assets: bool,
        /// Directory containing DFF model files (used with --convert-assets)
        #[arg(long)]
        dff_path: Option<String>,
        /// Directory containing TXD texture files (used with --convert-assets)
        #[arg(long)]
        txd_path: Option<String>,
    },
}

#[derive(Subcommand)]
enum ImgCommands {
    /// Show IMG archive information
    Info {
        /// IMG file path
        file: String,
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },
    /// List files in IMG archive
    List {
        /// IMG file path
        file: String,
        /// Filter by file extension (e.g., .dff)
        #[arg(long)]
        filter: Option<String>,
        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
    /// Extract files from IMG archive
    Extract {
        /// IMG file path
        file: String,
        /// Specific files to extract (empty for all)
        #[arg(num_args = 0..)]
        files: Vec<String>,
        /// Output directory
        #[arg(short, long)]
        output: String,
    },
}

#[derive(Subcommand)]
enum DffCommands {
    /// Show DFF model information
    Info {
        /// DFF file path
        file: String,
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },
    /// Analyze DFF model structure
    Analyze {
        /// DFF file path
        file: String,
        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Subcommand)]
enum TxdCommands {
    /// Show TXD texture information
    Info {
        /// TXD file path
        file: String,
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },
    /// Extract textures from TXD
    Extract {
        /// TXD file path
        file: String,
        /// Output directory
        #[arg(short, long)]
        output: String,
        /// Output image format
        #[arg(short, long, value_enum, default_value = "png")]
        format: ImageFormat,
    },
}

#[derive(Subcommand)]
enum ColCommands {
    /// Show COL collision information
    Info {
        /// COL file path
        file: String,
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },
    /// Analyze COL collision structure
    Analyze {
        /// COL file path
        file: String,
        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },
}

#[derive(Subcommand)]
enum IplCommands {
    /// Show IPL map information
    Info {
        /// IPL file path
        file: String,
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },
    /// Analyze IPL map structure
    Analyze {
        /// IPL file path
        file: String,
        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        /// Maximum number of entries to analyze
        #[arg(long)]
        max_entries: Option<usize>,
    },
}

#[derive(Subcommand)]
enum RwCommands {
    /// Analyze RenderWare file chunks
    Analyze {
        /// RenderWare file path (DFF/TXD/COL)
        file: String,
        /// Maximum chunk depth to analyze
        #[arg(short, long, default_value = "3")]
        depth: usize,
        /// Output format
        #[arg(short, long, value_enum, default_value = "tree")]
        format: RwFormat,
    },
}

#[derive(Subcommand)]
enum BatchCommands {
    /// Analyze multiple files
    Analyze {
        /// Directory to scan
        directory: String,
        /// File format to analyze
        #[arg(short, long, value_enum, default_value = "all")]
        format: BatchFormat,
        /// Scan directories recursively
        #[arg(short, long)]
        recursive: bool,
        /// Output file for results
        #[arg(short, long)]
        output: Option<String>,
        /// Maximum number of files to process
        #[arg(long)]
        max_files: Option<usize>,
    },
    /// Extract all IMG archives in directory
    #[command(name = "extract-imgs")]
    ExtractImgs {
        /// Directory containing IMG files
        directory: String,
        /// Output directory for extracted files
        #[arg(short, long)]
        output: String,
        /// Scan directories recursively
        #[arg(short, long)]
        recursive: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum BatchFormat {
    Dff,
    Txd,
    Col,
    Ipl,
    Img,
    All,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum ImageFormat {
    Png,
    Jpg,
}

#[derive(clap::ValueEnum, Clone, Debug, Default)]
enum RwFormat {
    #[default]
    Tree,
    Text,
    Json,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    // Initialize logging
    let filter_level = if cli.verbose {
        "debug"
    } else if cli.quiet {
        "error"
    } else {
        "info"
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter_level)
        .init();

    // Execute the command
    let result = execute_command(cli.command, cli.format, cli.verbose, cli.quiet).await;

    match result {
        Ok(exit_code) => {
            if !cli.quiet {
                info!("Command completed successfully");
            }
            process::exit(exit_code);
        }
        Err(e) => {
            error!("Command failed: {}", e);
            if cli.verbose {
                error!("Error details: {:?}", e);
            }
            process::exit(1);
        }
    }
}