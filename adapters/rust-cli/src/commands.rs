use crate::handlers::*;
use crate::converters::*;
use crate::{Commands, OutputFormat, ImageFormat, Cli};
use anyhow::Result;
use std::io::{self, Write};
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use tracing::error;

/// Execute the appropriate command based on the CLI input
pub async fn execute_command(
    command: Commands,
    global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    match command {
        Commands::Img { img_command } => execute_img_command(img_command, global_format, verbose, quiet).await,
        Commands::Dff { dff_command } => execute_dff_command(dff_command, global_format, verbose, quiet).await,
        Commands::Txd { txd_command } => execute_txd_command(txd_command, global_format, verbose, quiet).await,
        Commands::Col { col_command } => execute_col_command(col_command, global_format, verbose, quiet).await,
        Commands::Ipl { ipl_command } => execute_ipl_command(ipl_command, global_format, verbose, quiet).await,
        Commands::Rw { rw_command } => execute_rw_command(rw_command, global_format, verbose, quiet).await,
        Commands::Batch { batch_command } => execute_batch_command(batch_command, global_format, verbose, quiet).await,
        Commands::DffToObj { file, output, txd, no_csv } => {
            execute_dff_to_obj(file, output, txd, no_csv, verbose, quiet).await
        }
        Commands::TxdToMaterials { file, output, format } => {
            execute_txd_to_materials(file, output, format, verbose, quiet).await
        }
        Commands::Completion { shell } => {
            generate_completion(shell);
            Ok(0)
        }
        Commands::Wizard => {
            run_wizard().await
        }
        Commands::Convert { convert_command } => execute_convert_command(convert_command, verbose, quiet).await,
        Commands::Godot {
            input,
            output,
            template,
            project_name,
            with_rust,
            ide_files,
            convert_assets,
            dff_path,
            txd_path,
        } => {
            execute_ipl_to_godot(
                input,
                output,
                template,
                project_name,
                with_rust,
                ide_files,
                convert_assets,
                dff_path,
                txd_path,
                verbose,
                quiet,
            ).await
        }
        Commands::IplToGodot {
            input,
            output,
            template,
            project_name,
            with_rust,
            ide_files,
            convert_assets,
            dff_path,
            txd_path,
        } => {
            execute_ipl_to_godot(
                input,
                output,
                template,
                project_name,
                with_rust,
                ide_files,
                convert_assets,
                dff_path,
                txd_path,
                verbose,
                quiet,
            ).await
        }
    }
}

async fn execute_img_command(
    command: crate::ImgCommands,
    global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = IMGHandler::new(verbose, quiet);
    match command {
        crate::ImgCommands::Info { file, detailed } => {
            handler.info(&file, detailed, global_format).await
        }
        crate::ImgCommands::List { file, filter, format } => {
            handler.list(&file, filter.as_deref(), format).await
        }
        crate::ImgCommands::Extract { file, files, output } => {
            let files_to_extract = if files.is_empty() { None } else { Some(files) };
            handler.extract(&file, files_to_extract.as_ref(), &output).await
        }
    }
}

async fn execute_dff_command(
    command: crate::DffCommands,
    global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = DFFHandler::new(verbose, quiet);
    match command {
        crate::DffCommands::Info { file, detailed } => {
            handler.info(&file, detailed, global_format).await
        }
        crate::DffCommands::Analyze { file, format } => {
            handler.analyze(&file, format).await
        }
    }
}

async fn execute_txd_command(
    command: crate::TxdCommands,
    global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = TXDHandler::new(verbose, quiet);
    match command {
        crate::TxdCommands::Info { file, detailed } => {
            handler.info(&file, detailed, global_format).await
        }
        crate::TxdCommands::Extract { file, output, format } => {
            handler.extract(&file, &output, format).await
        }
    }
}

async fn execute_col_command(
    command: crate::ColCommands,
    global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = COLHandler::new(verbose, quiet);
    match command {
        crate::ColCommands::Info { file, detailed } => {
            handler.info(&file, detailed, global_format).await
        }
        crate::ColCommands::Analyze { file, format } => {
            handler.analyze(&file, format).await
        }
    }
}

async fn execute_ipl_command(
    command: crate::IplCommands,
    global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = IPLHandler::new(verbose, quiet);
    match command {
        crate::IplCommands::Info { file, detailed } => {
            handler.info(&file, detailed, global_format).await
        }
        crate::IplCommands::Analyze { file, format, max_entries } => {
            handler.analyze(&file, format, max_entries).await
        }
    }
}

async fn execute_rw_command(
    command: crate::RwCommands,
    _global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = RWHandler::new(verbose, quiet);
    match command {
        crate::RwCommands::Analyze { file, depth, format } => {
            handler.analyze(&file, depth, format).await
        }
    }
}

async fn execute_batch_command(
    command: crate::BatchCommands,
    _global_format: OutputFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = BatchHandler::new(verbose, quiet);
    match command {
        crate::BatchCommands::Analyze { directory, format, recursive, output, max_files } => {
            handler.analyze(&directory, format, recursive, output.as_deref(), max_files).await
        }
        crate::BatchCommands::ExtractImgs { directory, output, recursive } => {
            handler.extract_imgs(&directory, &output, recursive).await
        }
    }
}

async fn execute_convert_command(
    command: crate::ConvertCommands,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    match command {
        crate::ConvertCommands::Dff { file_or_dir, output, txd, no_csv, max_files, output_dir } => {
            execute_dff_convert(file_or_dir, output, txd, !no_csv, max_files, output_dir, verbose, quiet).await
        }
        crate::ConvertCommands::Txd { file, output, format } => {
            execute_txd_to_materials(file, output, format, verbose, quiet).await
        }
    }
}

async fn execute_dff_convert(
    file_or_dir: String,
    output: Option<String>,
    txd: Option<String>,
    use_csv: bool,
    max_files: Option<usize>,
    output_dir: Option<String>,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    use std::path::Path;

    let path = Path::new(&file_or_dir);

    if path.is_file() {
        // Single file conversion
        if let Some(output_path) = output {
            execute_dff_to_obj(file_or_dir, output_path, txd, use_csv, verbose, quiet).await
        } else {
            if !quiet {
                eprintln!("Error: --output is required when converting a single file");
            }
            Ok(1)
        }
    } else if path.is_dir() {
        // Directory conversion
        let output_base = output_dir.unwrap_or_else(|| "/tmp/dff_output".to_string());
        std::fs::create_dir_all(&output_base)?;

        if verbose && !quiet {
            println!("Converting DFF files from directory: {}", file_or_dir);
            println!("Output directory: {}", output_base);
            if let Some(limit) = max_files {
                println!("Limit: first {} files", limit);
            }
        }

        // Find all DFF files
        let mut dff_files = Vec::new();
        find_files_recursively(&file_or_dir, "dff", &mut dff_files)?;

        // Sort for consistent ordering
        dff_files.sort();

        // Apply limit if specified
        if let Some(limit) = max_files {
            dff_files.truncate(limit);
        }

        if dff_files.is_empty() {
            if !quiet {
                println!("No DFF files found in {}", file_or_dir);
            }
            return Ok(0);
        }

        if verbose && !quiet {
            println!("Found {} DFF file(s) to convert", dff_files.len());
        }

        let mut success_count = 0;
        let mut error_count = 0;

        for (i, dff_file) in dff_files.iter().enumerate() {
            // Quick validation to skip non-DFF files
            match rengine_core::renderware::dff::DffModel::is_valid_dff_file(dff_file) {
                Ok(true) => {
                    // File appears to be a valid DFF, proceed with conversion
                }
                Ok(false) => {
                    error!("File {} is not a valid DFF file", dff_file);
                    if !quiet {
                        println!("Skipping {} - not a valid DFF file", dff_file);
                    }
                    error_count += 1;
                    continue;
                }
                Err(e) => {
                    error!("File {} validation failed: {}", dff_file, e);
                    if !quiet {
                        println!("Skipping {} - validation error: {}", dff_file, e);
                    }
                    error_count += 1;
                    continue;
                }
            }

            if verbose && !quiet {
                println!("Converting [{}/{}] {}", i + 1, dff_files.len(), dff_file);
            } else if !quiet {
                show_progress(i + 1, dff_files.len(), &format!("Converting {}", Path::new(dff_file).file_name().unwrap_or_default().to_string_lossy()), quiet);
            }

            let filename = Path::new(dff_file)
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy();
            let output_path = format!("{}/{}.obj", output_base, filename);

            match execute_dff_to_obj(dff_file.clone(), output_path, txd.clone(), !use_csv, verbose && !quiet, true).await {
                Ok(0) => success_count += 1,
                Ok(exit_code) => {
                    error!("File {} conversion failed with exit code {}", dff_file, exit_code);
                    if !quiet {
                        println!("Failed to convert {} (exit code {})", dff_file, exit_code);
                    }
                    error_count += 1;
                }
                Err(e) => {
                    error!("File {} conversion failed: {}", dff_file, e);
                    if !quiet {
                        println!("Failed to convert {}: {}", dff_file, e);
                    }
                    error_count += 1;
                }
            }
        }

        // Also extract PNG files from TXD files in the same directory
        if verbose && !quiet {
            println!();
            println!("Extracting PNG textures from TXD files...");
        }

        let mut txd_files = Vec::new();
        find_files_recursively(&file_or_dir, "txd", &mut txd_files)?;
        txd_files.sort();

        // Apply the same limit to TXD files if specified
        if let Some(limit) = max_files {
            txd_files.truncate(limit);
        }

        let mut txd_success_count = 0;
        let mut txd_error_count = 0;

        if !txd_files.is_empty() {
            if verbose && !quiet {
                println!("Found {} TXD file(s) to extract textures from", txd_files.len());
            }

            for (i, txd_file) in txd_files.iter().enumerate() {
                if verbose && !quiet {
                    println!("Extracting [{}/{}] {}", i + 1, txd_files.len(), txd_file);
                } else if !quiet {
                    show_progress(i + 1, txd_files.len(), &format!("Extracting {}", Path::new(txd_file).file_name().unwrap_or_default().to_string_lossy()), quiet);
                }

                // Create a subdirectory for each TXD file's textures
                let txd_filename = Path::new(txd_file)
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy();
                let texture_output_dir = format!("{}/{}", output_base, txd_filename);

                match execute_txd_extract(txd_file.clone(), texture_output_dir, ImageFormat::Png, verbose && !quiet, true).await {
                    Ok(0) => txd_success_count += 1,
                    Ok(exit_code) => {
                        error!("TXD {} extraction failed with exit code {}", txd_file, exit_code);
                        if !quiet {
                            println!("Failed to extract {} (exit code {})", txd_file, exit_code);
                        }
                        txd_error_count += 1;
                    }
                    Err(e) => {
                        error!("TXD {} extraction failed: {}", txd_file, e);
                        if !quiet {
                            println!("Failed to extract {}: {}", txd_file, e);
                        }
                        txd_error_count += 1;
                    }
                }
            }
        }

        if !quiet {
            if verbose {
                println!();
            }
            println!("Conversion complete: {} successful, {} failed", success_count, error_count);
            if !txd_files.is_empty() {
                println!("TXD extraction complete: {} successful, {} failed", txd_success_count, txd_error_count);
            }
            if error_count > 0 {
                if success_count > 0 {
                    println!("Note: {} files failed to convert (likely corrupted or unsupported formats)", error_count);
                } else {
                    eprintln!("Warning: All {} files failed to convert", error_count);
                }
            }
        }

        // Return error code if any files failed to convert
        if error_count > 0 || txd_error_count > 0 {
            Ok(1)
        } else {
            Ok(0)
        }
    } else {
        if !quiet {
            eprintln!("Error: {} is not a valid file or directory", file_or_dir);
        }
        Ok(1)
    }
}

fn find_files_recursively(dir: &str, extension: &str, files: &mut Vec<String>) -> Result<()> {
    use std::fs;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            find_files_recursively(&path.to_string_lossy(), extension, files)?;
        } else if path.extension().unwrap_or_default() == extension {
            files.push(path.to_string_lossy().to_string());
        }
    }

    Ok(())
}

async fn execute_dff_to_obj(
    file: String,
    output: String,
    txd: Option<String>,
    use_csv: bool,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let converter = DffToObjConverter::new(verbose, quiet);
    converter.convert(&file, &output, txd.as_deref(), use_csv).await
}

async fn execute_txd_to_materials(
    file: String,
    output: String,
    format: ImageFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let converter = TxdToMaterialsConverter::new(verbose, quiet);
    converter.convert(&file, &output, format).await
}

async fn execute_txd_extract(
    file: String,
    output: String,
    format: ImageFormat,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let handler = TXDHandler::new(verbose, quiet);
    handler.extract(&file, &output, format).await
}

async fn execute_ipl_to_godot(
    input: String,
    output: String,
    template: Option<String>,
    project_name: String,
    with_rust: bool,
    ide_files: Option<Vec<String>>,
    convert_assets: bool,
    dff_path: Option<String>,
    txd_path: Option<String>,
    verbose: bool,
    quiet: bool,
) -> Result<i32> {
    let converter = IplToGodotConverter::new(verbose, quiet);
    converter.convert(
        &input,
        &output,
        template.as_deref(),
        &project_name,
        with_rust,
        ide_files.as_deref(),
        convert_assets,
        dff_path.as_deref(),
        txd_path.as_deref(),
    ).await
}

/// Generate shell completion scripts
fn generate_completion(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "rengine-cli", &mut io::stdout());
}

/// Interactive wizard for common operations
async fn run_wizard() -> Result<i32> {
    println!("🦀 Rengine CLI Interactive Wizard");
    println!("=================================");
    println!();
    println!("What would you like to do?");
    println!("1. Analyze a single file");
    println!("2. Convert a file");
    println!("3. Batch process multiple files");
    println!("4. Generate shell completion");
    println!("5. Show help");
    println!();
    print!("Enter your choice (1-5): ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim();

    match choice {
        "1" => {
            println!();
            println!("📁 File Analysis Wizard");
            println!("What type of file would you like to analyze?");
            println!("1. IMG archive");
            println!("2. DFF model");
            println!("3. TXD texture");
            println!("4. COL collision");
            println!("5. IPL map");
            println!("6. RenderWare chunk");
            print!("Enter file type (1-6): ");
            io::stdout().flush().unwrap();

            let mut file_type = String::new();
            io::stdin().read_line(&mut file_type).unwrap();
            let file_type = file_type.trim();

            print!("Enter file path: ");
            io::stdout().flush().unwrap();
            let mut file_path = String::new();
            io::stdin().read_line(&mut file_path).unwrap();
            let file_path = file_path.trim();

            match file_type {
                "1" => println!("Run: rengine-cli img info \"{}\"", file_path),
                "2" => println!("Run: rengine-cli dff info \"{}\"", file_path),
                "3" => println!("Run: rengine-cli txd info \"{}\"", file_path),
                "4" => println!("Run: rengine-cli col info \"{}\"", file_path),
                "5" => println!("Run: rengine-cli ipl info \"{}\"", file_path),
                "6" => println!("Run: rengine-cli rw analyze \"{}\"", file_path),
                _ => println!("❌ Invalid choice"),
            }
        }
        "2" => {
            println!();
            println!("🔄 File Conversion Wizard");
            println!("What would you like to convert?");
            println!("1. DFF model to OBJ");
            println!("2. TXD texture to materials");
            println!("3. IPL map to Godot scene");
            print!("Enter conversion type (1-3): ");
            io::stdout().flush().unwrap();

            let mut conv_type = String::new();
            io::stdin().read_line(&mut conv_type).unwrap();
            let conv_type = conv_type.trim();

            match conv_type {
                "1" => {
                    print!("Enter DFF file path: ");
                    io::stdout().flush().unwrap();
                    let mut dff_path = String::new();
                    io::stdin().read_line(&mut dff_path).unwrap();
                    let dff_path = dff_path.trim();

                    print!("Enter output OBJ path: ");
                    io::stdout().flush().unwrap();
                    let mut obj_path = String::new();
                    io::stdin().read_line(&mut obj_path).unwrap();
                    let obj_path = obj_path.trim();

                    println!("Run: rengine-cli convert dff \"{}\" --output \"{}\"", dff_path, obj_path);
                    println!("Or:   rengine-cli dff-to-obj \"{}\" --output \"{}\"", dff_path, obj_path);
                }
                "2" => {
                    print!("Enter TXD file path: ");
                    io::stdout().flush().unwrap();
                    let mut txd_path = String::new();
                    io::stdin().read_line(&mut txd_path).unwrap();
                    let txd_path = txd_path.trim();

                    print!("Enter output directory: ");
                    io::stdout().flush().unwrap();
                    let mut out_dir = String::new();
                    io::stdin().read_line(&mut out_dir).unwrap();
                    let out_dir = out_dir.trim();

                    println!("Run: rengine-cli convert txd \"{}\" --output \"{}\"", txd_path, out_dir);
                    println!("Or:   rengine-cli txd-to-materials \"{}\" --output \"{}\"", txd_path, out_dir);
                }
                "3" => {
                    print!("Enter IPL file or directory path: ");
                    io::stdout().flush().unwrap();
                    let mut ipl_path = String::new();
                    io::stdin().read_line(&mut ipl_path).unwrap();
                    let ipl_path = ipl_path.trim();

                    print!("Enter output directory: ");
                    io::stdout().flush().unwrap();
                    let mut out_dir = String::new();
                    io::stdin().read_line(&mut out_dir).unwrap();
                    let out_dir = out_dir.trim();

                    println!("Run: rengine-cli godot \"{}\" --output \"{}\"", ipl_path, out_dir);
                    println!("Or:   rengine-cli ipl-to-godot \"{}\" --output \"{}\"", ipl_path, out_dir);
                }
                _ => println!("❌ Invalid choice"),
            }
        }
        "3" => {
            println!();
            println!("📊 Batch Processing Wizard");
            print!("Enter directory to scan: ");
            io::stdout().flush().unwrap();
            let mut directory = String::new();
            io::stdin().read_line(&mut directory).unwrap();
            let directory = directory.trim();

            println!("What type of files?");
            println!("1. All files");
            println!("2. DFF models");
            println!("3. TXD textures");
            println!("4. COL collisions");
            println!("5. IPL maps");
            print!("Enter file type (1-5): ");
            io::stdout().flush().unwrap();

            let mut file_type = String::new();
            io::stdin().read_line(&mut file_type).unwrap();
            let file_type = file_type.trim();

            let format = match file_type {
                "1" => "all",
                "2" => "dff",
                "3" => "txd",
                "4" => "col",
                "5" => "ipl",
                _ => "all",
            };

            println!("Run: rengine-cli batch analyze \"{}\" --format {} --recursive", directory, format);
            println!("Add --max-files 50 to limit to first 50 files");
        }
        "4" => {
            println!();
            println!("🔧 Shell Completion Setup");
            println!("Available shells: bash, zsh, fish, powershell, elvish");
            println!();
            println!("# For Bash:");
            println!("rengine-cli completion bash > ~/.bash_completion.d/rengine-cli");
            println!("echo 'source ~/.bash_completion.d/rengine-cli' >> ~/.bashrc");
            println!();
            println!("# For Zsh:");
            println!("rengine-cli completion zsh > ~/.zsh/_rengine-cli");
            println!();
            println!("# For Fish:");
            println!("rengine-cli completion fish > ~/.config/fish/completions/rengine-cli.fish");
        }
        "5" | _ => {
            println!("Run: rengine-cli --help");
        }
    }

    Ok(0)
}

/// Simple progress indicator for long operations
fn show_progress(current: usize, total: usize, item: &str, quiet: bool) {
    if quiet {
        return;
    }

    let percentage = if total > 0 { (current * 100) / total } else { 100 };
    eprint!("\rProcessing: {} ({}/{}) - {}%", item, current, total, percentage);
    io::stderr().flush().unwrap();

    if current >= total {
        eprintln!(); // New line when done
    }
}