// Integration tests using real GTA SA files
// These tests require the GTA SA resource files to be present

#[cfg(test)]
mod integration_tests {
    use std::path::Path;
    use crate::handlers::{DFFHandler, TXDHandler};
    use crate::converters::{DffToObjConverter, TxdToMaterialsConverter};
    use crate::{OutputFormat, ImageFormat};

    const GTA_SA_PATH: &str = "/home/blefnk/Documents/reliverse/rengine/preloaded/resources/gta-sa";

    fn gta_sa_available() -> bool {
        Path::new(GTA_SA_PATH).exists()
    }

    #[tokio::test]
    async fn test_dff_info_with_real_file() {
        if !gta_sa_available() {
            println!("Skipping integration test: GTA SA files not available");
            return;
        }

        let dff_file = format!("{}/15x125Road1.dff", GTA_SA_PATH);
        if !Path::new(&dff_file).exists() {
            println!("Skipping test: DFF file not found");
            return;
        }

        let handler = DFFHandler::new(false, true); // quiet mode
        let result = handler.info(&dff_file, false, OutputFormat::Json).await;
        assert!(result.is_ok(), "DFF info should succeed with real file");
    }

    #[tokio::test]
    async fn test_dff_analyze_with_real_file() {
        if !gta_sa_available() {
            println!("Skipping integration test: GTA SA files not available");
            return;
        }

        let dff_file = format!("{}/15x125Road1.dff", GTA_SA_PATH);
        if !Path::new(&dff_file).exists() {
            println!("Skipping test: DFF file not found");
            return;
        }

        let handler = DFFHandler::new(false, true); // quiet mode
        let result = handler.analyze(&dff_file, OutputFormat::Json).await;
        assert!(result.is_ok(), "DFF analyze should succeed with real file");
    }

    #[tokio::test]
    async fn test_txd_info_with_real_file() {
        if !gta_sa_available() {
            println!("Skipping integration test: GTA SA files not available");
            return;
        }

        let txd_file = format!("{}/2notherbuildsfe.txd", GTA_SA_PATH);
        if !Path::new(&txd_file).exists() {
            println!("Skipping test: TXD file not found");
            return;
        }

        let handler = TXDHandler::new(false, true); // quiet mode
        let result = handler.info(&txd_file, false, OutputFormat::Json).await;
        assert!(result.is_ok(), "TXD info should succeed with real file");
    }

    #[tokio::test]
    async fn test_txd_extract_with_real_file() {
        if !gta_sa_available() {
            println!("Skipping integration test: GTA SA files not available");
            return;
        }

        let txd_file = format!("{}/7_11_door.txd", GTA_SA_PATH);
        if !Path::new(&txd_file).exists() {
            println!("Skipping test: TXD file not found");
            return;
        }

        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir = temp_dir.path().to_str().unwrap();

        let handler = TXDHandler::new(false, true); // quiet mode
        let result = handler.extract(&txd_file, output_dir, ImageFormat::Png).await;
        assert!(result.is_ok(), "TXD extract should succeed with real file");

        // Check that some files were created
        let output_path = Path::new(output_dir);
        assert!(output_path.exists(), "Output directory should exist");
        let entries = std::fs::read_dir(output_path).unwrap();
        let file_count = entries.count();
        assert!(file_count > 0, "Should have extracted at least one texture file");
    }

    #[tokio::test]
    async fn test_dff_to_obj_converter_with_real_file() {
        if !gta_sa_available() {
            println!("Skipping integration test: GTA SA files not available");
            return;
        }

        let dff_file = format!("{}/15x125Road1.dff", GTA_SA_PATH);
        if !Path::new(&dff_file).exists() {
            println!("Skipping test: DFF file not found");
            return;
        }

        let temp_dir = tempfile::tempdir().unwrap();
        let output_file = temp_dir.path().join("test.obj").to_str().unwrap().to_string();

        let converter = DffToObjConverter::new(false, true); // quiet mode
        let result = converter.convert(&dff_file, &output_file, None, false).await;
        assert!(result.is_ok(), "DFF to OBJ conversion should succeed with real file");

        // Check that output file was created
        assert!(Path::new(&output_file).exists(), "OBJ file should be created");
    }

    #[tokio::test]
    async fn test_txd_to_materials_converter_with_real_file() {
        if !gta_sa_available() {
            println!("Skipping integration test: GTA SA files not available");
            return;
        }

        let txd_file = format!("{}/7_11_door.txd", GTA_SA_PATH);
        if !Path::new(&txd_file).exists() {
            println!("Skipping test: TXD file not found");
            return;
        }

        let temp_dir = tempfile::tempdir().unwrap();
        let output_dir = temp_dir.path().to_str().unwrap();

        let converter = TxdToMaterialsConverter::new(false, true); // quiet mode
        let result = converter.convert(&txd_file, output_dir, ImageFormat::Png).await;
        assert!(result.is_ok(), "TXD to materials conversion should succeed with real file");

        // Check that output directory has files
        let output_path = Path::new(output_dir);
        assert!(output_path.exists(), "Output directory should exist");

        // Should have at least a materials.mtl file
        let mtl_file = output_path.join("materials.mtl");
        assert!(mtl_file.exists(), "Materials file should be created");
    }
}