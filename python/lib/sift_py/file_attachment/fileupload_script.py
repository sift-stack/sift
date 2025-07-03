#!/usr/bin/env python3
"""
SiftStack File Upload Script
Uploads files to SiftStack with optional metadata extraction using FileAttachmentService.
Supports both run names and UUIDs, with automatic metadata extraction via FFMPEG for .mp4 files only.
"""

import argparse
import subprocess
import json
import sys
import re
from datetime import datetime
from pathlib import Path
from typing import List, Optional, Dict, Any
from urllib.parse import urljoin

from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.file_attachment.service import FileAttachmentService
from sift_py.file_attachment.entity import Entity, EntityType
from sift_py.file_attachment.metadata import VideoMetadata
from sift_py.ingestion._internal.run import get_run_id_by_name
from sift_py.rest import SiftRestConfig
from sift.annotations.v1.annotations_pb2 import ListAnnotationsRequest
from sift.annotations.v1.annotations_pb2_grpc import AnnotationServiceStub

def is_uuid(identifier: str) -> bool:
    """
    Check if a string is a valid UUID.
    
    Args:
        identifier: String to check
        
    Returns:
        True if the string matches UUID format, False otherwise
    """
    uuid_pattern = re.compile(
        r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$',
        re.IGNORECASE
    )
    return bool(uuid_pattern.match(identifier))

# API URL configurations
REST_API_URLS = {
    'production': 'https://api.siftstack.com',
    'gov': 'https://gov.api.siftstack.com', 
    'development': 'https://api.development.siftstack.com'
}

GRPC_API_URLS = {
    'production': 'https://grpc-api.siftstack.com',
    'gov': 'https://gov.grpc-api.siftstack.com', 
    'development': 'https://grpc-api.development.siftstack.com'
}



def get_video_metadata(video_path: Path) -> Optional[Dict[str, Any]]:
    """
    Extract video metadata using FFMPEG.
    
    Args:
        video_path: Path to the video file
        
    Returns:
        Dictionary containing video metadata or None if extraction fails
    """
    try:
        cmd = [
            'ffprobe',
            '-v', 'quiet',
            '-print_format', 'json',
            '-show_format',
            '-show_streams',
            str(video_path)
        ]
        
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        data = json.loads(result.stdout)
        
        # Find video stream
        video_stream = None
        for stream in data.get('streams', []):
            if stream.get('codec_type') == 'video':
                video_stream = stream
                break
        
        if not video_stream:
            print(f"Warning: No video stream found in {video_path}")
            return None
        
        # Extract metadata
        duration = data.get('format', {}).get('duration')
        duration_seconds = float(duration) if duration else None
        
        width = video_stream.get('width')
        height = video_stream.get('height')
        file_size = data.get('format', {}).get('size')
        file_size = int(file_size) if file_size else None
        creation_time = data.get('format', {}).get('tags', {}).get('creation_time')
        
        return {
            'width': width,
            'height': height,
            'duration_seconds': duration_seconds,
            'file_size': file_size,
            'creation_time': creation_time,
            'codec': video_stream.get('codec_name'),
            'bitrate': data.get('format', {}).get('bit_rate'),
            'fps': eval(video_stream.get('r_frame_rate', '0/1')) if video_stream.get('r_frame_rate') else None
        }
        
    except subprocess.CalledProcessError as e:
        print(f"Error running ffprobe on {video_path}: {e}")
        print("Make sure FFMPEG is installed and accessible in your PATH")
        return None
    except json.JSONDecodeError as e:
        print(f"Error parsing ffprobe output for {video_path}: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error processing {video_path}: {e}")
        return None



def get_annotation_uuid(grpc_url: str, api_key: str, annotation_identifier: str) -> str:
    """
    Get annotation UUID from name or return UUID if already provided.
    Automatically detects if annotation_identifier is a UUID.
    
    Args:
        grpc_url: SiftStack gRPC API URL
        api_key: SiftStack API key
        annotation_identifier: Annotation name or UUID
        
    Returns:
        Annotation UUID
        
    Raises:
        Exception: If annotation lookup fails
    """
    # Check if annotation_identifier is already a UUID
    if is_uuid(annotation_identifier):
        print(f"Using provided UUID: {annotation_identifier}")
        return annotation_identifier
    
    print(f"Looking up annotation UUID for name: '{annotation_identifier}'")
    
    # Create gRPC channel config
    grpc_config = SiftChannelConfig(uri=grpc_url, apikey=api_key)
    
    try:
        with use_sift_channel(grpc_config) as channel:
            svc = AnnotationServiceStub(channel)
            req = ListAnnotationsRequest(
                filter=f'name=="{annotation_identifier}"',
                page_size=1,
            )
            res = svc.ListAnnotations(req)
            
            if len(res.annotations) > 0:
                annotation_id = res.annotations[0].annotation_id
                print(f"Found annotation UUID: {annotation_id}")
                return annotation_id
            else:
                raise Exception(f"Annotation '{annotation_identifier}' not found")
                    
    except Exception as e:
        print(f"Error looking up annotation: {e}")
        print("Make sure:")
        print("  - The annotation name is correct")
        print("  - You have access to the annotation")
        print("  - The API key has the right permissions")
        raise

def get_run_uuid(grpc_url: str, api_key: str, run_identifier: str) -> str:
    """
    Get run UUID from name or return UUID if already provided.
    Automatically detects if run_identifier is a UUID.
    
    Args:
        grpc_url: SiftStack gRPC API URL
        api_key: SiftStack API key
        run_identifier: Run name or UUID
        
    Returns:
        Run UUID
        
    Raises:
        Exception: If run lookup fails
    """
    # Check if run_identifier is already a UUID
    if is_uuid(run_identifier):
        print(f"Using provided UUID: {run_identifier}")
        return run_identifier
    
    print(f"Looking up run UUID for name: '{run_identifier}'")
    
    # Create gRPC channel config
    grpc_config = SiftChannelConfig(uri=grpc_url, apikey=api_key)
    
    try:
        with use_sift_channel(grpc_config) as channel:
            run_id = get_run_id_by_name(channel, run_identifier)
            
            if run_id:
                print(f"Found run UUID: {run_id}")
                return run_id
            else:
                raise Exception(f"Run '{run_identifier}' not found")
                    
    except Exception as e:
        print(f"Error looking up run: {e}")
        print("Make sure:")
        print("  - The run name is correct")
        print("  - You have access to the run")
        print("  - The API key has the right permissions")
        raise

def upload_file(file_attachment_service: FileAttachmentService, file_path: Path, entity: Entity, 
                metadata: Optional[VideoMetadata] = None, description: str = "") -> str:
    """
    Upload a single file using FileAttachmentService.
    
    Args:
        file_attachment_service: Initialized FileAttachmentService
        file_path: Path to file to upload
        entity: Target entity (run, annotation, etc.)
        metadata: Optional video metadata
        description: File description
        
    Returns:
        Remote file ID
        
    Raises:
        Exception: If upload fails
    """
    try:
        remote_file = file_attachment_service.upload_attachment(
            path=file_path,
            entity=entity,
            metadata=metadata,
            description=description
        )
        return remote_file.remote_file_id
    except Exception as e:
        print(f"Error uploading {file_path}: {e}")
        raise

def main():
    parser = argparse.ArgumentParser(
        description='Upload files to SiftStack with optional metadata extraction for .mp4 files',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # Upload any files to run using name
  %(prog)s --api-key YOUR_KEY --entity "launch v1.0" --files video1.mp4 data.csv image.png
  
  # Upload to run using UUID directly (automatically detected)
  %(prog)s --api-key YOUR_KEY --entity "f942399b-9e4c-4302-a9ad-0486c5c08e4e" --files *.mp4 *.csv
  
  # Upload to annotation using name
  %(prog)s --api-key YOUR_KEY --entity "Test Annotation" --files video.mp4 --entity-type annotations
  
  # Upload to annotation using UUID directly
  %(prog)s --api-key YOUR_KEY --entity "afe78569-172d-4982-ab79-c726844a1df7" --files video.mp4 --entity-type annotations
  
  # Auto metadata extraction (only works with .mp4 files)
  %(prog)s --api-key YOUR_KEY --entity "test run" --files *.mp4 --auto-metadata
  
  # Auto metadata with custom timestamp (only works with .mp4 files)
  %(prog)s --api-key YOUR_KEY --entity "launch" --files video.mp4 --auto-metadata --timestamp "2024-01-15T10:30:00"
  
  # Auto metadata extraction. Use video creation time as the timestamp in Sift (only works with .mp4 files)
  %(prog)s --api-key YOUR_KEY --entity "launch" --files video.mp4 --auto-metadata --use-creation-time
  
  # Upload mixed file types (metadata only extracted for .mp4 files)
  %(prog)s --api-key YOUR_KEY --entity "launch" --files video.mp4 data.csv image.png --auto-metadata
  
  # Development environment
  %(prog)s --api-key YOUR_KEY --entity "dev run" --files video.mp4 --env development
        """
    )
    
    # Required arguments
    parser.add_argument('--api-key', required=True, help='SiftStack API key')
    parser.add_argument('--entity', required=True, help='Run/annotation name or UUID (UUIDs are automatically detected)')
    parser.add_argument('--files', required=True, nargs='+', help='Files to upload (supports glob patterns, auto-metadata only works with .mp4 files)')
    
    # Optional arguments
    parser.add_argument('--env', choices=['production', 'gov', 'development'], 
                       default='production', help='SiftStack environment (default: production)')
    parser.add_argument('--auto-metadata', action='store_true', 
                       help='Automatically extract metadata using FFMPEG (only applies to .mp4 files)')
    parser.add_argument('--use-creation-time', action='store_true',
                       help='Use video creation time as timestamp in metadata (only applies to .mp4 files)')
    parser.add_argument('--timestamp', 
                       help='Custom timestamp for video metadata (ISO format: YYYY-MM-DDTHH:MM:SS, only applies to .mp4 files)')
    parser.add_argument('--description', default='', 
                       help='Description for uploaded files')
    parser.add_argument('--entity-type', choices=['runs', 'annotations'], 
                       default='runs', help='Entity type (default: runs)')
    
    args = parser.parse_args()
    
    # Set up API configuration
    rest_api_url = REST_API_URLS[args.env]
    grpc_api_url = GRPC_API_URLS[args.env]
    
    rest_base_uri = rest_api_url.replace("https://", "")
    grpc_base_uri = grpc_api_url.replace("https://", "")
    
    rest_config = SiftRestConfig(uri=rest_base_uri, apikey=args.api_key)
    sift_channel_config = SiftChannelConfig(uri=grpc_base_uri, apikey=args.api_key)

    # Get entity UUID based on type
    try:
        if args.entity_type == 'annotations':
            entity_uuid = get_annotation_uuid(grpc_api_url, args.api_key, args.entity)
        else:
            entity_uuid = get_run_uuid(grpc_api_url, args.api_key, args.entity)
    except Exception as e:
        print(f"Failed to get {args.entity_type} UUID: {e}")
        sys.exit(1)
    
    # Create entity
    entity = Entity(entity_id=entity_uuid, entity_type=EntityType(args.entity_type))
    
    # Expand file patterns and check existence
    all_files = []
    for file_pattern in args.files:
        files = list(Path('.').glob(file_pattern))
        if not files:
            print(f"Warning: No files found matching pattern '{file_pattern}'")
        all_files.extend(files)
    
    if not all_files:
        print("Error: No files found to upload!")
        sys.exit(1)
    
    # Remove duplicates and sort
    all_files = sorted(set(all_files))
    
    # Display configuration
    print(f"Environment: {args.env}")
    print(f"REST API: {rest_api_url}")
    print(f"gRPC API: {grpc_api_url}")
    print(f"{args.entity_type.title()}: {args.entity} -> {entity_uuid}")
    print(f"Entity Type: {args.entity_type}")
    print(f"Files to upload: {len(all_files)}")
    for f in all_files:
        print(f"  - {f}")
    print()
    
    # Process files
    successful_uploads = 0
    failed_uploads = 0
    
    try:
        with use_sift_channel(sift_channel_config) as channel:
            file_attachment_service = FileAttachmentService(channel, rest_config)
            
            for file_path in all_files:
                print(f"Processing: {file_path}")
                
                # Check if file exists
                if not file_path.exists():
                    print(f"  Error: File not found!")
                    failed_uploads += 1
                    continue
                
                # Extract metadata if requested and file is .mp4
                metadata = None
                if args.auto_metadata and file_path.suffix.lower() == '.mp4':
                    print(f"  Extracting metadata (MP4 file)...")
                    metadata_dict = get_video_metadata(file_path)
                    
                    if metadata_dict:
                        try:
                            # Determine timestamp
                            timestamp = None
                            if args.timestamp:
                                try:
                                    # Handle various timestamp formats
                                    timestamp_str = args.timestamp
                                    if timestamp_str.endswith('Z'):
                                        timestamp_str = timestamp_str[:-1] + '+00:00'
                                    elif 'T' in timestamp_str and '+' not in timestamp_str and '-' in timestamp_str[10:]:
                                        # Handle format like "2025-06-16T17:37:30.033-07:00"
                                        pass  # Already in correct format
                                    elif 'T' in timestamp_str and '+' not in timestamp_str and '-' not in timestamp_str[10:]:
                                        # Handle format like "2025-06-16T17:37:30.033" (no timezone)
                                        timestamp_str = timestamp_str + '+00:00'
                                    
                                    timestamp = datetime.fromisoformat(timestamp_str)
                                    print(f"  Using custom timestamp: {timestamp}")
                                except ValueError as e:
                                    print(f"  Warning: Invalid timestamp format '{args.timestamp}': {e}")
                                    print(f"  Expected format: YYYY-MM-DDTHH:MM:SS or YYYY-MM-DDTHH:MM:SS.SSS±HH:MM")
                            elif args.use_creation_time and metadata_dict.get('creation_time'):
                                try:
                                    creation_time_str = metadata_dict['creation_time']
                                    if creation_time_str.endswith('Z'):
                                        creation_time_str = creation_time_str[:-1] + '+00:00'
                                    timestamp = datetime.fromisoformat(creation_time_str)
                                    print(f"  Using video creation time: {timestamp}")
                                except ValueError as e:
                                    print(f"  Warning: Invalid creation time format '{metadata_dict['creation_time']}': {e}")
                            
                            metadata = VideoMetadata(
                                width=metadata_dict['width'],
                                height=metadata_dict['height'],
                                duration_seconds=metadata_dict['duration_seconds'],
                                timestamp=timestamp
                            )
                            print(f"  Metadata: {metadata_dict['width']}x{metadata_dict['height']}, "
                                  f"{metadata_dict['duration_seconds']:.2f}s, "
                                  f"{metadata_dict.get('file_size', 'N/A')} bytes")
                        except Exception as e:
                            print(f"  Warning: Could not create metadata object: {e}")
                    else:
                        print(f"  Warning: Could not extract metadata")
                elif args.auto_metadata and file_path.suffix.lower() != '.mp4':
                    print(f"  Skipping metadata extraction (not an MP4 file)")
                
                # Upload file
                try:
                    description = args.description or f"Uploaded via script: {file_path.name}"
                    remote_file_id = upload_file(file_attachment_service, file_path, entity, metadata, description)
                    print(f"Uploaded successfully! ID: {remote_file_id}")
                    successful_uploads += 1
                except Exception as e:
                    print(f"Upload failed: {e}")
                    failed_uploads += 1
                
                print()
                
    except Exception as e:
        print(f"Error initializing file attachment service: {e}")
        sys.exit(1)
    
    # Summary
    print("=" * 50)
    print(f"Upload Summary:")
    print(f"  Successful: {successful_uploads}")
    print(f"  Failed: {failed_uploads}")
    print(f"  Total: {len(all_files)}")
    
    if failed_uploads > 0:
        sys.exit(1)

if __name__ == "__main__":
    main()
