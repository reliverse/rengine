import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import type { ImgArchive } from "../../types";

export function useArchiveOperations() {
  const loadImgArchive = async (filePath: string): Promise<ImgArchive> => {
    try {
      console.log("Loading IMG archive:", filePath);
      const archive = (await invoke("load_img_archive", {
        path: filePath,
      })) as ImgArchive;
      console.log("IMG archive loaded:", archive);
      return archive;
    } catch (error) {
      console.error("Failed to load IMG archive:", error);
      throw error;
    }
  };

  const extractImgEntry = async (
    archivePath: string,
    entryName: string,
    outputPath: string
  ): Promise<void> => {
    try {
      await invoke("extract_img_entry", {
        archivePath,
        entryName,
        outputPath,
      });
      console.log(`Extracted ${entryName} to ${outputPath}`);
    } catch (error) {
      console.error(`Failed to extract ${entryName}:`, error);
      throw error;
    }
  };

  const extractMultipleEntries = async (
    archive: ImgArchive,
    archivePath: string,
    outputDir: string,
    maxEntries = 10
  ): Promise<void> => {
    for (const entry of archive.entries.slice(0, maxEntries)) {
      try {
        await extractImgEntry(
          archivePath,
          entry.name,
          `${outputDir}/${entry.name}`
        );
      } catch (error) {
        console.error(`Failed to extract ${entry.name}:`, error);
      }
    }
  };

  const selectOutputDirectory = async (): Promise<string | null> => {
    const outputDir = await open({
      directory: true,
    });
    return typeof outputDir === "string" ? outputDir : null;
  };

  const selectOutputFile = async (
    defaultName: string
  ): Promise<string | null> => {
    const outputPath = await open({
      defaultPath: defaultName,
    });
    return typeof outputPath === "string" ? outputPath : null;
  };

  return {
    loadImgArchive,
    extractImgEntry,
    extractMultipleEntries,
    selectOutputDirectory,
    selectOutputFile,
  };
}
