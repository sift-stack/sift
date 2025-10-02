use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Generate the stub files
    let stub =
        sift_stream_bindings::stub_info().map_err(|e| format!("Failed to get stub info: {e}"))?;
    stub.generate()
        .map_err(|e| format!("Failed to generate stubs: {e}"))?;

    // Post-process the generated stub file to add __all__ and @final decorators
    post_process_stub_file()?;

    Ok(())
}

fn extract_exported_classes() -> Result<Vec<String>> {
    // Read lib.rs to extract exported class names
    let lib_content = fs::read_to_string("src/lib.rs")?;

    let mut classes = Vec::new();

    // Look for lines containing m.add_class::<...>
    for line in lib_content.lines() {
        let trimmed = line.trim();

        // Extract the part between ::< and >
        if trimmed.starts_with("m.add_class::<")
            && trimmed.ends_with(">()?;")
            && let Some(start) = trimmed.find("::<")
            && let Some(end) = trimmed.find(">()?;")
        {
            let class_path = &trimmed[start + 3..end];
            // Get the last part after the final ::
            let class_name = class_path.split("::").last().unwrap_or(class_path);
            classes.push(class_name.to_string());
        }
    }

    // Sort for consistent ordering
    classes.sort();

    if classes.is_empty() {
        return Err("No exported classes found in lib.rs".into());
    }

    println!("Found {} exported classes: {:?}", classes.len(), classes);
    Ok(classes)
}

fn post_process_stub_file() -> Result<()> {
    // Generated stub file is named with hyphens, but we want to use underscores
    let hyphenated_name = "sift-stream-bindings.pyi";
    let underscore_name = "sift_stream_bindings.pyi";

    let stub_file_path = if Path::new(hyphenated_name).exists() {
        hyphenated_name
    } else if Path::new(underscore_name).exists() {
        underscore_name
    } else {
        return Err(format!(
            "Stub file not found. Expected either {hyphenated_name} or {underscore_name}"
        )
        .into());
    };

    // Read the generated stub file
    let content = fs::read_to_string(stub_file_path)?;

    // Extract leading comments and empty lines
    let lines: Vec<&str> = content.lines().collect();
    let mut leading_comments = Vec::new();
    let mut content_start_index = 0;

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("#") {
            leading_comments.push(line.to_string());
            content_start_index = i + 1;
        } else {
            break;
        }
    }

    // Get the content without leading comments
    let content_without_comments = lines[content_start_index..].join("\n");

    // Automatically extract exported classes from lib.rs
    let classes_to_finalize = extract_exported_classes()?;

    // Generate __all__ declaration
    let all_declaration = format!(
        "__all__ = [\n{}\n]",
        classes_to_finalize
            .iter()
            .map(|class| format!("    \"{class}\","))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Add @final decorators to classes
    let mut processed_content = content_without_comments;

    // Add @final decorator before each class
    for class_name in &classes_to_finalize {
        let class_pattern = format!("class {class_name}(");
        let class_pattern_no_inherit = format!("class {class_name}:");

        // Handle classes with inheritance
        if processed_content.contains(&class_pattern) {
            processed_content = processed_content.replace(
                &class_pattern,
                &format!("@typing.final\nclass {class_name}("),
            );
        }
        // Handle classes without inheritance
        else if processed_content.contains(&class_pattern_no_inherit) {
            processed_content = processed_content.replace(
                &class_pattern_no_inherit,
                &format!("@typing.final\nclass {class_name}:"),
            );
        }
    }

    // Add __all__ at the beginning after imports
    let content_lines: Vec<&str> = processed_content.lines().collect();
    let mut result_lines = Vec::new();
    let mut added_all = false;

    for (i, line) in content_lines.iter().enumerate() {
        result_lines.push(line.to_string());

        // Add __all__ after the last import statement
        if !added_all && (line.starts_with("from ") || line.starts_with("import ")) {
            // Check if next line is also an import
            let is_last_import = i + 1 >= content_lines.len()
                || (!content_lines[i + 1].starts_with("from ")
                    && !content_lines[i + 1].starts_with("import ")
                    && !content_lines[i + 1].trim().is_empty());

            if is_last_import {
                result_lines.push("".to_string());
                result_lines.push(all_declaration.clone());
                result_lines.push("".to_string());
                added_all = true;
            }
        }
    }

    // If we didn't add __all__ yet (no imports found), add it at the beginning
    if !added_all {
        processed_content = format!("{all_declaration}\n\n{processed_content}");
    } else {
        processed_content = result_lines.join("\n");
    }

    // Combine leading comments with processed content
    let mut final_content = String::new();

    // Add leading comments
    if !leading_comments.is_empty() {
        for line in leading_comments {
            if !line.contains("noqa") {
                final_content.push_str(&line);
            }
        }
        final_content.push_str("\n\n");
    }

    // Add processed content
    final_content.push_str(&processed_content);

    // Write the processed content to the correct filename (with underscores)
    fs::write(underscore_name, &final_content)?;

    // If we read from the hyphenated version, remove it
    if stub_file_path == hyphenated_name && Path::new(hyphenated_name).exists() {
        fs::remove_file(hyphenated_name)?;
        println!("Renamed stub file from {hyphenated_name} to {underscore_name}");
    }

    println!("Successfully post-processed stub files: added __all__ and @final decorators");
    println!("Processed {} classes", classes_to_finalize.len());

    Ok(())
}
