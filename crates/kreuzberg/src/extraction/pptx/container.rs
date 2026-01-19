//! PPTX container and ZIP archive management.
//!
//! This module handles opening PPTX files, reading files from the ZIP archive,
//! finding slide paths, and iterating through slides.

use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

use super::elements::Slide;
use super::image_handling::get_full_image_path;
use crate::error::{KreuzbergError, Result};

pub(super) struct PptxContainer {
    pub(super) archive: ZipArchive<File>,
    slide_paths: Vec<String>,
}

impl PptxContainer {
    pub(super) fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        // IO errors must bubble up unchanged - file access issues need user reports ~keep
        let file = File::open(path)?;

        let mut archive = match ZipArchive::new(file) {
            Ok(arc) => arc,
            Err(zip::result::ZipError::Io(io_err)) => return Err(io_err.into()), // Bubble up IO errors ~keep
            Err(e) => {
                return Err(KreuzbergError::parsing(format!(
                    "Failed to read PPTX archive (invalid format): {}",
                    e
                )));
            }
        };

        let slide_paths = Self::find_slide_paths(&mut archive)?;

        Ok(Self { archive, slide_paths })
    }

    pub(super) fn slide_paths(&self) -> &[String] {
        &self.slide_paths
    }

    pub(super) fn read_file(&mut self, path: &str) -> Result<Vec<u8>> {
        match self.archive.by_name(path) {
            Ok(mut file) => {
                let mut contents = Vec::new();
                // IO errors must bubble up - file read issues need user reports ~keep
                file.read_to_end(&mut contents)?;
                Ok(contents)
            }
            Err(zip::result::ZipError::FileNotFound) => {
                Err(KreuzbergError::parsing("File not found in archive".to_string()))
            }
            Err(zip::result::ZipError::Io(io_err)) => Err(io_err.into()), // Bubble up IO errors ~keep
            Err(e) => Err(KreuzbergError::parsing(format!("Zip error: {}", e))),
        }
    }

    pub(super) fn get_slide_rels_path(&self, slide_path: &str) -> String {
        super::image_handling::get_slide_rels_path(slide_path)
    }

    fn find_slide_paths(archive: &mut ZipArchive<File>) -> Result<Vec<String>> {
        if let Ok(rels_data) = Self::read_file_from_archive(archive, "ppt/_rels/presentation.xml.rels")
            && let Ok(paths) = super::parser::parse_presentation_rels(&rels_data)
        {
            return Ok(paths);
        }

        let mut slide_paths = Vec::new();
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                let name = file.name();
                if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                    slide_paths.push(name.to_string());
                }
            }
        }

        slide_paths.sort();
        Ok(slide_paths)
    }

    fn read_file_from_archive(archive: &mut ZipArchive<File>, path: &str) -> Result<Vec<u8>> {
        let mut file = match archive.by_name(path) {
            Ok(f) => f,
            Err(zip::result::ZipError::Io(io_err)) => return Err(io_err.into()), // Bubble up IO errors ~keep
            Err(e) => {
                return Err(KreuzbergError::parsing(format!(
                    "Failed to read file from archive: {}",
                    e
                )));
            }
        };
        let mut contents = Vec::new();
        // IO errors must bubble up - file read issues need user reports ~keep
        file.read_to_end(&mut contents)?;
        Ok(contents)
    }
}

pub(super) struct SlideIterator {
    container: PptxContainer,
    current_index: usize,
    total_slides: usize,
}

impl SlideIterator {
    pub(super) fn new(container: PptxContainer) -> Self {
        let total_slides = container.slide_paths().len();
        Self {
            container,
            current_index: 0,
            total_slides,
        }
    }

    pub(super) fn slide_count(&self) -> usize {
        self.total_slides
    }

    pub(super) fn next_slide(&mut self) -> Result<Option<Slide>> {
        if self.current_index >= self.total_slides {
            return Ok(None);
        }

        let slide_path = &self.container.slide_paths()[self.current_index].clone();
        let slide_number = (self.current_index + 1) as u32;

        let xml_data = self.container.read_file(slide_path)?;

        let rels_path = self.container.get_slide_rels_path(slide_path);
        let rels_data = self.container.read_file(&rels_path).ok();

        let slide = Slide::from_xml(slide_number, &xml_data, rels_data.as_deref())?;

        self.current_index += 1;

        Ok(Some(slide))
    }

    pub(super) fn get_slide_images(&mut self, slide: &Slide) -> Result<HashMap<String, Vec<u8>>> {
        let mut image_data = HashMap::new();

        for img_ref in &slide.images {
            let slide_path = &self.container.slide_paths()[slide.slide_number as usize - 1];
            let full_path = get_full_image_path(slide_path, &img_ref.target);

            if let Ok(data) = self.container.read_file(&full_path) {
                image_data.insert(img_ref.id.clone(), data);
            }
        }

        Ok(image_data)
    }
}
