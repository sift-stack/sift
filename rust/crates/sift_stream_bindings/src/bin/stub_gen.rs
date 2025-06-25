use std::fs;
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    // Generate the stub files
    let stub =
        sift_stream_bindings::stub_info().map_err(|e| format!("Failed to get stub info: {}", e))?;
    stub.generate()
        .map_err(|e| format!("Failed to generate stubs: {}", e))?;

    // Post-process the generated stub file to add __all__ and @final decorators
    post_process_stub_file()?;

    Ok(())
}

fn post_process_stub_file() -> Result<()> {
    // First, try to find the generated stub file - it might be named with hyphens
    let hyphenated_name = "sift-stream-bindings.pyi";
    let underscore_name = "sift_stream_bindings.pyi";

    let stub_file_path = if Path::new(hyphenated_name).exists() {
        hyphenated_name
    } else if Path::new(underscore_name).exists() {
        underscore_name
    } else {
        return Err(format!(
            "Stub file not found. Expected either {} or {}",
            hyphenated_name, underscore_name
        )
        .into());
    };

    // Read the generated stub file
    let content = fs::read_to_string(stub_file_path)?;

    // List of all exported classes that need @final decorator
    let classes_to_finalize = vec![
        "SiftStreamPy",
        "FlowPy",
        "SiftStreamBuilderPy",
        "IngestionConfigFormPy",
        "FlowConfigPy",
        "ChannelConfigPy",
        "ChannelDataTypePy",
        "ChannelEnumTypePy",
        "ChannelBitFieldElementPy",
        "DurationPy",
        "RecoveryStrategyPy",
        "RetryPolicyPy",
        "RunFormPy",
        "TimeValuePy",
        "ChannelValuePy",
        "ChannelValueTypePy",
        "IngestWithConfigDataStreamRequestPy",
    ];

    // Generate __all__ declaration
    let all_declaration = format!(
        "__all__ = [\n{}\n]\n\n",
        classes_to_finalize
            .iter()
            .map(|class| format!("    \"{}\",", class))
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Add @final decorators to classes
    let mut processed_content = content;

    // Add typing import at the top if not present
    if !processed_content.contains("from typing") {
        processed_content = format!("from typing import final\n\n{}", processed_content);
    } else if !processed_content.contains("final") {
        // Add final to existing typing import
        processed_content =
            processed_content.replace("from typing import", "from typing import final,");
    }

    // Add @final decorator before each class
    for class_name in &classes_to_finalize {
        let class_pattern = format!("class {}(", class_name);
        let class_pattern_no_inherit = format!("class {}:", class_name);

        // Handle classes with inheritance
        if processed_content.contains(&class_pattern) {
            processed_content = processed_content
                .replace(&class_pattern, &format!("@final\nclass {}(", class_name));
        }
        // Handle classes without inheritance
        else if processed_content.contains(&class_pattern_no_inherit) {
            processed_content = processed_content.replace(
                &class_pattern_no_inherit,
                &format!("@final\nclass {}:", class_name),
            );
        }
    }

    // Add __all__ at the beginning after imports
    let lines: Vec<&str> = processed_content.lines().collect();
    let mut result_lines = Vec::new();
    let mut added_all = false;

    for (i, line) in lines.iter().enumerate() {
        result_lines.push(line.to_string());

        // Add __all__ after the last import statement
        if !added_all && (line.starts_with("from ") || line.starts_with("import ")) {
            // Check if next line is also an import
            let is_last_import = i + 1 >= lines.len()
                || (!lines[i + 1].starts_with("from ")
                    && !lines[i + 1].starts_with("import ")
                    && !lines[i + 1].trim().is_empty());

            if is_last_import {
                result_lines.push("".to_string());
                result_lines.push(all_declaration.trim_end().to_string());
                result_lines.push("".to_string());
                added_all = true;
            }
        }
    }

    // If we didn't add __all__ yet (no imports found), add it at the beginning
    if !added_all {
        let final_content = format!("{}\n{}", all_declaration, processed_content);
        processed_content = final_content;
    } else {
        processed_content = result_lines.join("\n");
    }

    // Write the processed content to the correct filename (with underscores)
    fs::write(underscore_name, processed_content)?;

    // If we read from the hyphenated version, remove it
    if stub_file_path == hyphenated_name && Path::new(hyphenated_name).exists() {
        fs::remove_file(hyphenated_name)?;
        println!(
            "Renamed stub file from {} to {}",
            hyphenated_name, underscore_name
        );
    }

    println!("Successfully post-processed stub file: added __all__ and @final decorators");

    Ok(())
}
