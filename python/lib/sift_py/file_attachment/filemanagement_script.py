#!/usr/bin/env python3
"""
SiftStack File Attachment Deletion Script
Lists and deletes file attachments from SiftStack entities using FileAttachmentService.
Supports runs, annotations, and annotation logs with both run names and UUIDs.
"""

import argparse
import sys
import re
from pathlib import Path
from typing import List, Optional

from sift_py.grpc.transport import SiftChannelConfig, use_sift_channel
from sift_py.file_attachment.service import FileAttachmentService
from sift_py.file_attachment.entity import Entity, EntityType
from sift_py.rest import SiftRestConfig
from sift_py.ingestion._internal.run import get_run_id_by_name
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

def format_file_size(size_bytes: int) -> str:
    """
    Format file size in human readable format.
    
    Args:
        size_bytes: File size in bytes
        
    Returns:
        Formatted size string (e.g., "1.5 MB")
    """
    if size_bytes == 0:
        return "0 bytes"
    
    size_names = ["bytes", "KB", "MB", "GB", "TB"]
    import math
    i = int(math.floor(math.log(size_bytes, 1024)))
    p = math.pow(1024, i)
    s = round(size_bytes / p, 2)
    return f"{s} {size_names[i]}"

def list_attachments(file_attachment_service: FileAttachmentService, entity: Entity, silent: bool = False) -> List:
    """
    List all file attachments for an entity.
    
    Args:
        file_attachment_service: Initialized FileAttachmentService
        entity: Target entity
        silent: If True, don't print detailed listing (just retrieve)
        
    Returns:
        List of file attachments
    """
    if not silent:
        print(f"Retrieving file attachments for {entity.entity_type.value} {entity.entity_id}...")
    
    try:
        attachments = file_attachment_service.retrieve_attachments(entity)
        
        if not attachments:
            if not silent:
                print("  No file attachments found.")
            return []
        
        if not silent:
            print(f"  Found {len(attachments)} file attachment(s):")
            print()
            
            for i, attachment in enumerate(attachments, 1):
                file_size = format_file_size(attachment.file_size) if attachment.file_size else "Unknown"
                created_date = attachment.created_date.ToDatetime().strftime("%Y-%m-%d %H:%M:%S") if attachment.created_date else "Unknown"
                
                print(f"  {i:2d}. {attachment.file_name}")
                print(f"      ID: {attachment.remote_file_id}")
                print(f"      Size: {file_size}")
                print(f"      Created: {created_date}")
                if attachment.description:
                    print(f"      Description: {attachment.description}")
                
                # Display metadata if available
                if hasattr(attachment, 'video_metadata') and attachment.video_metadata:
                    vm = attachment.video_metadata
                    has_real_video_metadata = (
                        (getattr(vm, 'width', 0) or 0) > 0 or
                        (getattr(vm, 'height', 0) or 0) > 0 or
                        (getattr(vm, 'duration_seconds', 0) or 0) > 0
                    )
                    if has_real_video_metadata:
                        print(f"      Video Metadata:")
                        if vm.width and vm.height:
                            print(f"        Resolution: {vm.width}x{vm.height}")
                        if vm.duration_seconds:
                            duration_min = vm.duration_seconds / 60
                            print(f"        Duration: {vm.duration_seconds:.2f}s ({duration_min:.2f} min)")
                        if vm.timestamp:
                            timestamp_str = vm.timestamp.ToDatetime().strftime("%Y-%m-%d %H:%M:%S")
                            print(f"        Timestamp: {timestamp_str}")
                elif hasattr(attachment, 'image_metadata') and attachment.image_metadata:
                    print(f"      Image Metadata:")
                    if attachment.image_metadata.width and attachment.image_metadata.height:
                        print(f"        Resolution: {attachment.image_metadata.width}x{attachment.image_metadata.height}")
                elif hasattr(attachment, 'audio_metadata') and attachment.audio_metadata:
                    print(f"      Audio Metadata:")
                    if attachment.audio_metadata.duration_seconds:
                        duration_min = attachment.audio_metadata.duration_seconds / 60
                        print(f"        Duration: {attachment.audio_metadata.duration_seconds:.2f}s ({duration_min:.2f} min)")
                    if attachment.audio_metadata.timestamp:
                        timestamp_str = attachment.audio_metadata.timestamp.ToDatetime().strftime("%Y-%m-%d %H:%M:%S")
                        print(f"        Timestamp: {timestamp_str}")
                
                print()
        
        return attachments
        
    except Exception as e:
        print(f"Error retrieving attachments: {e}")
        return []

def delete_attachments(file_attachment_service: FileAttachmentService, attachments: List, 
                      file_ids: Optional[List[str]] = None, filenames: Optional[List[str]] = None,
                      all_files: bool = False, force: bool = False) -> int:
    """
    Delete specified file attachments.
    
    Args:
        file_attachment_service: Initialized FileAttachmentService
        attachments: List of available attachments
        file_ids: Specific file IDs to delete
        filenames: Specific filenames to delete
        all_files: Whether to delete all files
        force: Skip confirmation prompt
        
    Returns:
        Number of files deleted
    """
    if not attachments:
        print("No attachments to delete.")
        return 0
    
    if all_files:
        files_to_delete = attachments
        print(f"Deleting all {len(attachments)} file attachment(s)...")
    elif file_ids:
        files_to_delete = [att for att in attachments if att.remote_file_id in file_ids]
        if len(files_to_delete) != len(file_ids):
            found_ids = [att.remote_file_id for att in files_to_delete]
            missing_ids = [fid for fid in file_ids if fid not in found_ids]
            print(f"Warning: Could not find file(s) with ID(s): {missing_ids}")
        print(f"Deleting {len(files_to_delete)} file attachment(s) by ID...")
    elif filenames:
        files_to_delete = [att for att in attachments if att.file_name in filenames]
        if len(files_to_delete) != len(filenames):
            found_names = [att.file_name for att in files_to_delete]
            missing_names = [name for name in filenames if name not in found_names]
            print(f"Warning: Could not find file(s) with name(s): {missing_names}")
        print(f"Deleting {len(files_to_delete)} file attachment(s) by filename...")
    else:
        print("No files specified for deletion.")
        return 0
    
    if not files_to_delete:
        print("No valid files to delete.")
        return 0
    
    # Show what will be deleted
    print("Files to be deleted:")
    for i, attachment in enumerate(files_to_delete, 1):
        print(f"  {i}. {attachment.file_name} (ID: {attachment.remote_file_id})")
    
    # Confirm deletion
    if not force:
        response = input("\nAre you sure you want to delete these files? (yes/no): ")
        if response.lower() not in ['yes', 'y']:
            print("Deletion cancelled.")
            return 0
    
    try:
        file_attachment_service.delete_file_attachments(*files_to_delete)
        print(f"Successfully deleted {len(files_to_delete)} file attachment(s).")
        return len(files_to_delete)
    except Exception as e:
        print(f"Error deleting files: {e}")
        return 0

def main():
    parser = argparse.ArgumentParser(
        description='List and delete file attachments from SiftStack entities',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  # List all file attachments for a run
  %(prog)s --api-key YOUR_KEY --entity "launch v1.0" --list
  
  # List all file attachments for a run (using UUID - automatically detected)
  %(prog)s --api-key YOUR_KEY --entity "f942399b-9e4c-4302-a9ad-0486c5c08e4e" --list
  
  # Delete all file attachments for a run
  %(prog)s --api-key YOUR_KEY --entity "launch v1.0" --delete-all --force
  
  # Delete specific file attachments by ID
  %(prog)s --api-key YOUR_KEY --entity "launch v1.0" --delete-ids "file-id-1" "file-id-2" --force
  
  # Delete specific file attachments by filename (much easier!)
  %(prog)s --api-key YOUR_KEY --entity "launch v1.0" --delete-files "video.mp4" "data.csv" --force
  
  # List and delete with confirmation
  %(prog)s --api-key YOUR_KEY --entity "launch v1.0" --list --delete-all
  
  # List all file attachments for an annotation
  %(prog)s --api-key YOUR_KEY --entity "Test Annotation" --entity-type annotations --list
  
  # Delete all file attachments for an annotation
  %(prog)s --api-key YOUR_KEY --entity "Test Annotation" --entity-type annotations --delete-all --force
  
  # Work with annotations using UUID directly
  %(prog)s --api-key YOUR_KEY --entity "afe78569-172d-4982-ab79-c726844a1df7" --entity-type annotations --list
        """
    )
    
    # Required arguments
    parser.add_argument('--api-key', required=True, help='SiftStack API key')
    
    # Entity specification (mutually exclusive with entity)
    entity_group = parser.add_mutually_exclusive_group(required=True)
    entity_group.add_argument('--entity', help='Run/annotation name or UUID to work with')
    entity_group.add_argument('--entity-id', help='Entity ID (when not using --entity)')
    
    # Optional arguments
    parser.add_argument('--env', choices=['production', 'gov', 'development'], 
                       default='production', help='SiftStack environment (default: production)')
    parser.add_argument('--entity-type', choices=['runs', 'annotations'], 
                       default='runs', help='Entity type (default: runs)')
    
    # Action arguments
    parser.add_argument('--list', action='store_true', 
                       help='List all file attachments for the entity')
    parser.add_argument('--delete-all', action='store_true',
                       help='Delete all file attachments for the entity')
    parser.add_argument('--delete-ids', nargs='+', metavar='FILE_ID',
                       help='Delete specific file attachments by ID')
    parser.add_argument('--delete-files', nargs='+', metavar='FILENAME',
                       help='Delete specific file attachments by filename (much easier than using IDs)')
    parser.add_argument('--force', action='store_true',
                       help='Skip confirmation prompt for deletion')
    
    args = parser.parse_args()
    
    # Validate arguments
    if not args.list and not args.delete_all and not args.delete_ids and not args.delete_files:
        print("Error: Must specify at least one action (--list, --delete-all, --delete-ids, or --delete-files)")
        sys.exit(1)
    
    if args.entity_id and args.entity:
        print("Error: Cannot specify both --entity and --entity-id")
        sys.exit(1)
    
    # Set up API configuration
    rest_api_url = REST_API_URLS[args.env]
    grpc_api_url = GRPC_API_URLS[args.env]
    
    rest_base_uri = rest_api_url.replace("https://", "")
    grpc_base_uri = grpc_api_url.replace("https://", "")
    
    rest_config = SiftRestConfig(uri=rest_base_uri, apikey=args.api_key)
    sift_channel_config = SiftChannelConfig(uri=grpc_base_uri, apikey=args.api_key)
    
    # Determine entity ID
    if args.entity:
        try:
            if args.entity_type == 'annotations':
                entity_id = get_annotation_uuid(grpc_api_url, args.api_key, args.entity)
            else:
                entity_id = get_run_uuid(grpc_api_url, args.api_key, args.entity)
        except Exception as e:
            print(f"Failed to get {args.entity_type} UUID: {e}")
            sys.exit(1)
    else:
        entity_id = args.entity_id
    
    # Create entity
    entity = Entity(entity_id=entity_id, entity_type=EntityType(args.entity_type))
    
    # Display configuration
    print(f"Environment: {args.env}")
    print(f"REST API: {rest_api_url}")
    print(f"gRPC API: {grpc_api_url}")
    print(f"Entity: {entity.entity_type.value} {entity_id}")
    print()
    
    # Initialize file attachment service
    try:
        with use_sift_channel(sift_channel_config) as channel:
            file_attachment_service = FileAttachmentService(channel, rest_config)
            
            # Retrieve attachments if needed for listing or deletion
            attachments = []
            if args.list or args.delete_all or args.delete_ids or args.delete_files:
                # Use silent mode if we're only deleting (not listing)
                silent = not args.list
                attachments = list_attachments(file_attachment_service, entity, silent)
            
            # Delete attachments if requested
            deleted_count = 0
            if args.delete_all or args.delete_ids or args.delete_files:
                deleted_count = delete_attachments(
                    file_attachment_service, 
                    attachments, 
                    args.delete_ids, 
                    args.delete_files,
                    args.delete_all,
                    args.force
                )
            
            # Summary
            if args.list:
                print(f"Total attachments: {len(attachments)}")
            if deleted_count > 0:
                print(f"Deleted attachments: {deleted_count}")
                
    except Exception as e:
        print(f"Error: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main() 