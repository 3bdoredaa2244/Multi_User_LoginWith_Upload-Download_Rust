use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::stable::stable64_size;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct FileMeta {
    pub file_type: String,
    pub size: usize,
}

/// File storage (chunks + metadata)
thread_local! {
    static FILES: RefCell<HashMap<String, (FileMeta, Vec<Vec<u8>>)>> = RefCell::new(HashMap::new());
}

/// Check if file exists
#[query]
fn check_file_exists(name: String) -> bool {
    FILES.with(|files| files.borrow().contains_key(&name))
}

/// Delete file
#[update]
fn delete_file(name: String) -> bool {
    FILES.with(|files| files.borrow_mut().remove(&name).is_some())
}

/// Get a file chunk
#[query]
fn get_file_chunk(name: String, index: u64) -> Option<Vec<u8>> {
    FILES.with(|files| {
        files
            .borrow()
            .get(&name)
            .and_then(|(_, chunks)| chunks.get(index as usize).cloned())
    })
}

/// Get file type
#[query]
fn get_file_type(name: String) -> Option<String> {
    FILES.with(|files| files.borrow().get(&name).map(|(meta, _)| meta.file_type.clone()))
}

/// Get all files metadata
#[query]
fn get_files() -> Vec<(String, String, u64)> {
    FILES.with(|files| {
        files
            .borrow()
            .iter()
            .map(|(name, (meta, chunks))| {
                (
                    meta.file_type.clone(),
                    name.clone(),
                    meta.size as u64,
                )
            })
            .collect()
    })
}

/// Get total number of chunks for a file
#[query]
fn get_total_chunks(name: String) -> u64 {
    FILES.with(|files| {
        files
            .borrow()
            .get(&name)
            .map(|(_, chunks)| chunks.len() as u64)
            .unwrap_or(0)
    })
}

/// Upload file chunk
#[update]
fn upload_file_chunk(name: String, chunk: Vec<u8>, index: u64, file_type: String) {
    FILES.with(|files| {
        let mut files = files.borrow_mut();
        let entry = files.entry(name.clone()).or_insert_with(|| {
            (
                FileMeta {
                    file_type: file_type.clone(),
                    size: 0,
                },
                Vec::new(),
            )
        });

        // Expand chunks vector if needed
        if entry.1.len() <= index as usize {
            entry.1.resize(index as usize + 1, Vec::new());
        }

        // Insert chunk
        entry.1[index as usize] = chunk;
        entry.0.size = entry.1.iter().map(|c| c.len()).sum();
    });
}

// Candid export
ic_cdk::export_candid!();
