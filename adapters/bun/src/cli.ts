#!/usr/bin/env bun

import fs from "node:fs";
import path from "node:path";
import pMap from "p-map";
import { DffConverter, ModelType } from "./index.js";
import { colorCombos, formatters } from "./utils/colors.js";

interface GlobalArgs {
  help?: boolean;
}

interface ConvertArgs {
  dff?: string;
  txd?: string;
  inputDir?: string;
  outputDir?: string;
  limit?: number;
  concurrency?: number;
  type?: ModelType;
  output?: string;
  format?: "gltf" | "glb";
}

type ParsedArgs = GlobalArgs & {
  command?: "convert" | "schema" | "help";
  convertArgs?: ConvertArgs;
};

function parseArgs(): ParsedArgs {
  const args = process.argv.slice(2);
  const parsed: ParsedArgs = {};

  // If no arguments or help flag, show help
  if (args.length === 0 || args.includes("--help") || args.includes("-h")) {
    parsed.help = true;
    parsed.command = "help";
    return parsed;
  }

  // Determine the command
  const firstArg = args[0];
  if (firstArg?.startsWith("-")) {
    // Legacy mode - treat as convert command with old flags
    parsed.command = "convert";
    parsed.convertArgs = parseLegacyArgs(args);
  } else {
    // First argument is a command
    switch (firstArg) {
      case "convert":
        parsed.command = "convert";
        parsed.convertArgs = parseConvertArgs(args.slice(1));
        break;
      case "schema":
        parsed.command = "schema";
        break;
      case "help":
      case "--help":
      case "-h":
        parsed.command = "help";
        break;
      default:
        console.error(
          colorCombos.errorDetail(
            `Unknown command: ${firstArg}`,
            "available commands: convert, schema, help"
          )
        );
        parsed.command = "help";
        break;
    }
  }

  return parsed;
}

function parseConvertArgs(args: string[]): ConvertArgs {
  const parsed: ConvertArgs = {
    type: ModelType.OBJECT,
    format: "glb",
  };

  const argIterator = args[Symbol.iterator]();
  let current = argIterator.next();

  while (!current.done) {
    const arg = current.value;

    switch (arg) {
      case "-d":
      case "--dff":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected file path after --dff",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.dff = current.value;
        current = argIterator.next();
        break;
      case "-t":
      case "--txd":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected file path after --txd",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.txd = current.value;
        current = argIterator.next();
        break;
      case "-i":
      case "--input-dir":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected directory path after --input-dir",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.inputDir = current.value;
        current = argIterator.next();
        break;
      case "--output-dir":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected directory path after --output-dir",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.outputDir = current.value;
        current = argIterator.next();
        break;
      case "--limit": {
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected number after --limit",
              "missing argument"
            )
          );
          process.exit(1);
        }
        const limitValue = Number.parseInt(current.value, 10);
        if (Number.isNaN(limitValue) || limitValue < 0) {
          console.error(
            colorCombos.errorDetail(
              "Limit must be a non-negative integer",
              "invalid value"
            )
          );
          process.exit(1);
        }
        parsed.limit = limitValue;
        current = argIterator.next();
        break;
      }
      case "--concurrency": {
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected number after --concurrency",
              "missing argument"
            )
          );
          process.exit(1);
        }
        const concurrencyValue = Number.parseInt(current.value, 10);
        if (Number.isNaN(concurrencyValue) || concurrencyValue <= 0) {
          console.error(
            colorCombos.errorDetail(
              "Concurrency must be a positive integer",
              "invalid value"
            )
          );
          process.exit(1);
        }
        parsed.concurrency = concurrencyValue;
        current = argIterator.next();
        break;
      }
      case "-T":
      case "--type": {
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected type after --type",
              "missing argument"
            )
          );
          process.exit(1);
        }
        const type = current.value;
        if (type === "object") {
          parsed.type = ModelType.OBJECT;
        } else if (type === "skin") {
          parsed.type = ModelType.SKIN;
        } else if (type === "car") {
          parsed.type = ModelType.CAR;
        } else {
          console.error(
            colorCombos.errorDetail(
              `Invalid type: ${type}`,
              "must be one of: object, skin, car"
            )
          );
          process.exit(1);
        }
        current = argIterator.next();
        break;
      }
      case "-o":
      case "--output":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected file path after --output",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.output = current.value;
        current = argIterator.next();
        break;
    }
    case "-f":
      case "--format":
    {
      current = argIterator.next();
      if (current.done) {
        console.error(
          colorCombos.errorDetail(
            "Expected format after --format",
            "missing argument"
          )
        );
        process.exit(1);
      }
      const format = current.value;
      if (format === "gltf" || format === "glb") {
        parsed.format = format;
      } else {
        console.error(
          colorCombos.errorDetail(
            `Invalid format: ${format}`,
            "must be one of: gltf, glb"
          )
        );
        process.exit(1);
      }
      current = argIterator.next();
      break;
    }
    default:
    if (arg.startsWith("-")) {
      console.error(
        colorCombos.errorDetail(`Unknown option: ${arg}`, "invalid flag")
      );
      return parsed;
    }
    console.error(
      colorCombos.errorDetail(`Unexpected argument: ${arg}`, "not recognized")
    );
    return parsed;
  }
}

return parsed;
}

function parseLegacyArgs(args: string[]): ConvertArgs {
  const parsed: ConvertArgs = {
    type: ModelType.OBJECT,
    format: "glb",
  };

  const argIterator = args[Symbol.iterator]();
  let current = argIterator.next();

  while (!current.done) {
    const arg = current.value;

    switch (arg) {
      case "-d":
      case "--dff":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected file path after --dff",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.dff = current.value;
        current = argIterator.next();
        break;
      case "-t":
      case "--txd":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected file path after --txd",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.txd = current.value;
        current = argIterator.next();
        break;
      case "-i":
      case "--input-dir":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected directory path after --input-dir",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.inputDir = current.value;
        current = argIterator.next();
        break;
      case "--output-dir":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected directory path after --output-dir",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.outputDir = current.value;
        current = argIterator.next();
        break;
      case "--limit": {
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected number after --limit",
              "missing argument"
            )
          );
          process.exit(1);
        }
        const limitValue = Number.parseInt(current.value, 10);
        if (Number.isNaN(limitValue) || limitValue < 0) {
          console.error(
            colorCombos.errorDetail(
              "Limit must be a non-negative integer",
              "invalid value"
            )
          );
          process.exit(1);
        }
        parsed.limit = limitValue;
        current = argIterator.next();
        break;
      }
      case "--concurrency": {
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected number after --concurrency",
              "missing argument"
            )
          );
          process.exit(1);
        }
        const concurrencyValue = Number.parseInt(current.value, 10);
        if (Number.isNaN(concurrencyValue) || concurrencyValue <= 0) {
          console.error(
            colorCombos.errorDetail(
              "Concurrency must be a positive integer",
              "invalid value"
            )
          );
          process.exit(1);
        }
        parsed.concurrency = concurrencyValue;
        current = argIterator.next();
        break;
      }
      case "-T":
      case "--type": {
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected type after --type",
              "missing argument"
            )
          );
          process.exit(1);
        }
        const type = current.value;
        if (type === "object") {
          parsed.type = ModelType.OBJECT;
        } else if (type === "skin") {
          parsed.type = ModelType.SKIN;
        } else if (type === "car") {
          parsed.type = ModelType.CAR;
        } else {
          console.error(
            colorCombos.errorDetail(
              `Invalid type: ${type}`,
              "must be one of: object, skin, car"
            )
          );
          process.exit(1);
        }
        current = argIterator.next();
        break;
      }
      case "-o":
      case "--output":
        current = argIterator.next();
        if (current.done) {
          console.error(
            colorCombos.errorDetail(
              "Expected file path after --output",
              "missing argument"
            )
          );
          process.exit(1);
        }
        parsed.output = current.value;
        current = argIterator.next();
        break;
    }
    case "-f":
      case "--format":
    {
      current = argIterator.next();
      if (current.done) {
        console.error(
          colorCombos.errorDetail(
            "Expected format after --format",
            "missing argument"
          )
        );
        process.exit(1);
      }
      const format = current.value;
      if (format === "gltf" || format === "glb") {
        parsed.format = format;
      } else {
        console.error(
          colorCombos.errorDetail(
            `Invalid format: ${format}`,
            "must be one of: gltf, glb"
          )
        );
        process.exit(1);
      }
      current = argIterator.next();
      break;
    }
    default:
    if (arg.startsWith("-")) {
      console.error(
        colorCombos.errorDetail(`Unknown option: ${arg}`, "invalid flag")
      );
      return parsed;
    }
    console.error(
      colorCombos.errorDetail(`Unexpected argument: ${arg}`, "not recognized")
    );
    return parsed;
  }
}

return parsed;
}
        parsed.dff = current.value
current = argIterator.next();
break;
case "-t":
      case "--txd":
        current = argIterator.next()
if (current.done) {
  console.error(
    colorCombos.errorDetail(
      "Expected file path after --txd",
      "missing argument"
    )
  );
  process.exit(1);
}
parsed.txd = current.value;
current = argIterator.next();
break;
case "-i":
      case "--input-dir":
        current = argIterator.next()
if (current.done) {
  console.error(
    colorCombos.errorDetail(
      "Expected directory path after --input-dir",
      "missing argument"
    )
  );
  process.exit(1);
}
parsed.inputDir = current.value;
current = argIterator.next();
break;
case "--output-dir":
        current = argIterator.next()
if (current.done) {
  console.error(
    colorCombos.errorDetail(
      "Expected directory path after --output-dir",
      "missing argument"
    )
  );
  process.exit(1);
}
parsed.outputDir = current.value;
current = argIterator.next();
break;
case "--limit":
{
  current = argIterator.next();
  if (current.done) {
    console.error(
      colorCombos.errorDetail(
        "Expected number after --limit",
        "missing argument"
      )
    );
    process.exit(1);
  }
  const limitValue = Number.parseInt(current.value, 10);
  if (Number.isNaN(limitValue) || limitValue < 0) {
    console.error(
      colorCombos.errorDetail(
        "Limit must be a non-negative integer",
        "invalid value"
      )
    );
    process.exit(1);
  }
  parsed.limit = limitValue;
  current = argIterator.next();
  break;
}
case "--concurrency":
{
  current = argIterator.next();
  if (current.done) {
    console.error(
      colorCombos.errorDetail(
        "Expected number after --concurrency",
        "missing argument"
      )
    );
    process.exit(1);
  }
  const concurrencyValue = Number.parseInt(current.value, 10);
  if (Number.isNaN(concurrencyValue) || concurrencyValue <= 0) {
    console.error(
      colorCombos.errorDetail(
        "Concurrency must be a positive integer",
        "invalid value"
      )
    );
    process.exit(1);
  }
  parsed.concurrency = concurrencyValue;
  current = argIterator.next();
  break;
}
case "-T":
      case "--type":
{
  current = argIterator.next();
  if (current.done) {
    console.error(
      colorCombos.errorDetail("Expected type after --type", "missing argument")
    );
    process.exit(1);
  }
  const type = current.value;
  if (type === "object") {
    parsed.type = ModelType.OBJECT;
  } else if (type === "skin") {
    parsed.type = ModelType.SKIN;
  } else if (type === "car") {
    parsed.type = ModelType.CAR;
  } else {
    console.error(
      colorCombos.errorDetail(
        `Invalid type: ${type}`,
        "must be one of: object, skin, car"
      )
    );
    process.exit(1);
  }
  current = argIterator.next();
  break;
}
case "-o":
      case "--output":
        current = argIterator.next()
if (current.done) {
  console.error(
    colorCombos.errorDetail(
      "Expected file path after --output",
      "missing argument"
    )
  );
  process.exit(1);
}
parsed.output = current.value;
current = argIterator.next();
break;
case "-f":
      case "--format":
{
  current = argIterator.next();
  if (current.done) {
    console.error(
      colorCombos.errorDetail(
        "Expected format after --format",
        "missing argument"
      )
    );
    process.exit(1);
  }
  const format = current.value;
  if (format === "gltf" || format === "glb") {
    parsed.format = format;
  } else {
    console.error(
      colorCombos.errorDetail(
        `Invalid format: ${format}`,
        "must be one of: gltf, glb"
      )
    );
    process.exit(1);
  }
  current = argIterator.next();
  break;
}
case "--schema":
{
  parsed.schema = true;
  current = argIterator.next();
  break;
}
default:
if (arg.startsWith("-")) {
  console.error(
    colorCombos.errorDetail(`Unknown option: ${arg}`, "invalid flag")
  );
  printUsage();
  process.exit(1);
} else {
  console.error(
    colorCombos.errorDetail(`Unexpected argument: ${arg}`, "not recognized")
  );
  printUsage();
  process.exit(1);
}
}
  }

return parsed;
}

interface FilePair {
  dffPath: string;
  txdPath?: string; // Optional - may not exist
  baseName: string;
}

function findFilePairs(inputDir: string): FilePair[] {
  const pairs: FilePair[] = [];
  const files = fs.readdirSync(inputDir);

  // Group files by base name (without extension)
  const fileGroups: Record<string, { dff?: string; txd?: string }> = {};

  for (const file of files) {
    const fullPath = path.join(inputDir, file);
    const stat = fs.statSync(fullPath);

    if (stat.isFile()) {
      const ext = path.extname(file).toLowerCase();
      const baseName = path.basename(file, ext);

      if (!fileGroups[baseName]) {
        fileGroups[baseName] = {};
      }

      if (ext === ".dff") {
        fileGroups[baseName].dff = fullPath;
      } else if (ext === ".txd") {
        fileGroups[baseName].txd = fullPath;
      }
    }
  }

  // Create pairs for all DFF files (TXD is optional)
  for (const [baseName, group] of Object.entries(fileGroups)) {
    if (group.dff) {
      pairs.push({
        dffPath: group.dff,
        txdPath: group.txd, // May be undefined
        baseName,
      });
    }
  }

  return pairs;
}

async function convertSingleFile(argv: CliArgs) {
  const dff = argv.dff;
  const txd = argv.txd;
  const format = argv.format || "glb";
  const type = argv.type || ModelType.OBJECT;

  // Validate DFF file exists
  if (!dff) {
    console.error(
      colorCombos.errorDetail(
        "DFF file path is required",
        "missing --dff argument"
      )
    );
    process.exit(1);
  }
  if (!fs.existsSync(dff)) {
    console.error(
      colorCombos.errorDetail(
        `DFF file not found: ${formatters.file(dff)}`,
        "file does not exist"
      )
    );
    process.exit(1);
  }

  // Determine output path
  let outputPath = argv.output;
  if (!outputPath) {
    const dffName = path.basename(dff, path.extname(dff));
    outputPath = `${dffName}.${format}`;
  }

  // Read DFF file (required)
  console.log(colorCombos.fileProcessing(dff, "reading"));
  const dffBuffer = fs.readFileSync(dff);

  // Read TXD file (optional)
  let txdBuffer: Buffer | null = null;
  if (txd) {
    if (fs.existsSync(txd)) {
      console.log(colorCombos.textureProcessing(txd, "reading"));
      txdBuffer = fs.readFileSync(txd);
    } else {
      console.warn(
        colorCombos.warningSuggest(
          `TXD file not found: ${formatters.file(txd)}`,
          "continuing without textures"
        )
      );
    }
  } else {
    console.warn(
      colorCombos.warningSuggest(
        "No TXD file provided",
        "converting without textures"
      )
    );
  }

  // Convert
  console.log(
    colorCombos.processing(
      `Converting to ${format.toUpperCase()}`,
      `type: ${type}`
    )
  );
  const converter = new DffConverter(dffBuffer, txdBuffer, type);
  const result = await converter.convertDffToGltf();

  // Export
  console.log(colorCombos.successFile(`Exporting to: ${outputPath}`));
  if (format === "glb") {
    const buffer = await result.getBuffer();
    fs.writeFileSync(outputPath, buffer);
  } else {
    result.exportAs(outputPath);
  }
}

async function convertFilePair(
  pair: FilePair,
  outputDir: string | undefined,
  format: "gltf" | "glb",
  type: ModelType
): Promise<{
  success: boolean;
  baseName: string;
  outputPath: string;
  hasTextures: boolean;
  error?: Error;
}> {
  try {
    console.log(""); // Add blank line before each file
    console.log(colorCombos.processing(`Converting: ${pair.baseName}`));

    // Read DFF file (required)
    const dffBuffer = fs.readFileSync(pair.dffPath);

    // Read TXD file (optional)
    let txdBuffer: Buffer | null = null;
    let hasTextures = false;
    if (pair.txdPath) {
      try {
        txdBuffer = fs.readFileSync(pair.txdPath);
        hasTextures = true;
      } catch (_error) {
        // TXD file not readable, will convert without textures
      }
    }

    // Convert
    const converter = new DffConverter(dffBuffer, txdBuffer, type);
    const result = await converter.convertDffToGltf();

    // Determine output path
    const outputPath = outputDir
      ? path.join(outputDir, `${pair.baseName}.${format}`)
      : `${pair.baseName}.${format}`;

    // Export
    if (format === "glb") {
      const buffer = await result.getBuffer();
      fs.writeFileSync(outputPath, buffer);
    } else {
      result.exportAs(outputPath);
    }

    return { success: true, baseName: pair.baseName, outputPath, hasTextures };
  } catch (error) {
    return {
      success: false,
      baseName: pair.baseName,
      outputPath: "",
      hasTextures: false,
      error: error as Error,
    };
  }
}

async function convertBatch(argv: CliArgs) {
  const inputDir = argv.inputDir;
  const outputDir = argv.outputDir;
  const limit = argv.limit === undefined ? 10 : argv.limit;
  const concurrency = argv.concurrency || 4;
  const format = argv.format || "glb";
  const type = argv.type || ModelType.OBJECT;

  // Validate input directory exists
  if (!inputDir) {
    console.error(
      colorCombos.errorDetail(
        "Input directory path is required",
        "missing --input-dir argument"
      )
    );
    process.exit(1);
  }
  if (!fs.existsSync(inputDir)) {
    console.error(
      colorCombos.errorDetail(
        `Input directory not found: ${formatters.file(inputDir)}`,
        "directory does not exist"
      )
    );
    process.exit(1);
  }

  // Create output directory if specified
  if (outputDir) {
    try {
      fs.mkdirSync(outputDir, { recursive: true });
    } catch (_error) {
      console.error(
        colorCombos.errorDetail(
          `Failed to create output directory: ${formatters.file(outputDir)}`,
          "permission or filesystem error"
        )
      );
      process.exit(1);
    }
  }

  // Find file pairs
  const allFilePairs = findFilePairs(inputDir);
  if (allFilePairs.length === 0) {
    console.error(
      colorCombos.errorDetail(
        `No matching DFF/TXD file pairs found in directory: ${formatters.file(inputDir)}`,
        "no .dff files found"
      )
    );
    process.exit(1);
  }

  // Apply limit if specified
  const filePairs = limit === 0 ? allFilePairs : allFilePairs.slice(0, limit);
  if (limit > 0 && limit < allFilePairs.length) {
    console.log(
      colorCombos.warningSuggest(
        `Limiting to ${formatters.count(limit, "file pair(s)")} (found ${formatters.count(allFilePairs.length, "total")})`,
        "use --limit 0 for unlimited processing"
      )
    );
  } else if (limit === 0) {
    console.log(
      colorCombos.processing(
        "Processing all files (unlimited)",
        `found ${allFilePairs.length} total`
      )
    );
  } else {
    console.log(
      colorCombos.successFile(
        `Found ${filePairs.length} file pair(s) to convert`
      )
    );
  }

  console.log(
    colorCombos.info("⟳", `Processing with concurrency: ${concurrency}`)
  );
  const startTime = Date.now();

  // Process files concurrently using p-map
  const results = await pMap(
    filePairs,
    (pair) => convertFilePair(pair, outputDir, format, type),
    { concurrency }
  );

  const endTime = Date.now();
  const duration = ((endTime - startTime) / 1000).toFixed(1);

  // Count successes and failures
  const successful = results.filter((r) => r.success).length;
  const failed = results.filter((r) => !r.success).length;
  const withTextures = results.filter((r) => r.success && r.hasTextures).length;

  console.log(
    formatters.success(
      "✓",
      `Batch conversion completed in ${formatters.duration(Number.parseFloat(duration))}`
    )
  );
  console.log(
    formatters.success(
      "  ",
      `${formatters.count(successful, "files converted successfully")}`
    )
  );

  if (withTextures > 0) {
    console.log(
      formatters.info(
        "  ",
        `${formatters.count(withTextures, "files included textures")}`
      )
    );
  }

  if (failed > 0) {
    console.log(
      formatters.error(
        "  ",
        `${formatters.count(failed, "files failed to convert")}`
      )
    );
  }

  // Show limit information
  if (limit === 0) {
    console.log(
      formatters.info(
        "  ",
        `Processed all ${formatters.count(allFilePairs.length, "available files")} (unlimited)`
      )
    );
  } else if (argv.limit === undefined) {
    console.log(
      formatters.info(
        "  ",
        `Used default limit of ${formatters.count(limit, "files")}`
      )
    );
  } else {
    console.log(
      formatters.info(
        "  ",
        `Limited to ${formatters.count(limit, "files")} (${formatters.count(allFilePairs.length, "available")})`
      )
    );
  }
}

function generateSchema() {
  const schemaPath =
    "/home/blefnk/B/R/reliverse/rengine/node_modules/@reliverse/rengine/schema.json";

  try {
    // Ensure the directory exists
    const dir = path.dirname(schemaPath);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }

    console.log(colorCombos.successFile(`Generating schema: ${schemaPath}`));

    // Copy schema from the pre-defined location
    if (fs.existsSync(schemaPath)) {
      console.log(colorCombos.info("✓", "Schema file already exists"));
    }

    return { success: true };
  } catch (error) {
    console.error(
      colorCombos.errorDetail(
        `Failed to generate schema: ${error}`,
        "filesystem error"
      )
    );
    return { success: false, error: error as Error };
  }
}

function printUsage() {
  console.log(`
Usage: rengine [options]

Options:
  -d, --dff <file>       Path to the DFF file (for single file conversion)
  -t, --txd <file>       Path to the TXD file (optional, for textures)
  -i, --input-dir <dir>  Input directory containing DFF files (TXD files optional)
  --output-dir <dir>     Output directory for converted files (for batch conversion)
  --limit <number>       Maximum number of files to convert in batch mode (default: 10, 0 = unlimited)
  --concurrency <number> Number of concurrent conversions (default: 4)
  -T, --type <type>      Model type: object, skin, car (default: object)
  -o, --output <file>    Output file path (for single file conversion, default: input filename with .glb extension)
  -f, --format <format>  Output format: gltf, glb (default: glb)
  --schema                Generate Rengine JSON schema file
  -h, --help             Show this help message

Examples:
  # Single file conversion
  rengine -d model.dff -t model.txd  # With textures
  rengine -d model.dff               # Without textures (TXD optional)
  rengine --dff vehicle.dff --txd vehicle.txd --type car --output vehicle.glb
  rengine -d character.dff -t character.txd -T skin -f gltf

  # Batch directory conversion
  rengine -i ./models --output-dir ./output  # Processes all DFF files, TXD optional
  rengine --input-dir /path/to/dff-files --output-dir /path/to/output --type car
  rengine -i ./models --limit 5  # Convert only first 5 files
  rengine -i ./models --concurrency 8  # Use 8 concurrent conversions

  # Schema generation
  rengine --schema              # Generate Rengine JSON schema file`);
}

async function main() {
  const argv = parseArgs();

  if (argv.help) {
    printUsage();
    return;
  }

  // Handle schema generation
  if (argv.schema) {
    const result = generateSchema();
    if (result.success) {
      console.log("Schema generated successfully!");
    }
    return;
  }

  // Determine mode: single file or batch directory
  const hasSingleFileInputs = argv.dff; // DFF is required, TXD is optional
  const hasBatchInputs = argv.inputDir;
  const hasValidInputs = hasSingleFileInputs || hasBatchInputs;

  if (!hasValidInputs) {
    console.error(
      colorCombos.errorDetail(
        "Invalid arguments",
        "provide --dff for single file conversion, --input-dir for batch conversion, or --schema for schema generation"
      )
    );
    printUsage();
    process.exit(1);
  }

  if (hasSingleFileInputs && hasBatchInputs) {
    console.error(
      colorCombos.errorDetail(
        "Conflicting arguments",
        "cannot use both --dff and --input-dir at the same time"
      )
    );
    printUsage();
    process.exit(1);
  }

  try {
    if (hasSingleFileInputs) {
      await convertSingleFile(argv);
      console.log("Conversion completed successfully!");
    } else {
      await convertBatch(argv);
    }
  } catch (error) {
    console.error(formatters.error("✗", "Error during conversion:"), error);
    process.exit(1);
  }
}

main();
