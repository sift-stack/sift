# ABAC Resources Management Script

This script provides functions to create, update, and delete ABAC (Attribute-Based Access Control) resources:

- **User Attributes** - Keys and values for user attributes (principal attributes)
- **Resource Attributes** - Keys, enum values, and attributes for resources (entity attributes)
- **Access Policies** - Cedar-based access policies

## Setup

### Local Development

1. Install the package:
   ```bash
   cd python
   pip install -e .
   ```

2. Set environment variables:
   ```bash
   export SIFT_API_KEY="your_api_key"
   export BASE_URI="localhost:50051"  # or your Sift gRPC endpoint
   ```

3. Or create a `.env` file:
   ```
   SIFT_API_KEY=your_api_key
   BASE_URI=localhost:50051
   ```

### Google Colab / Remote Usage

For using in Google Colab or other environments, see [README_COLAB.md](README_COLAB.md) for installation instructions.

The package name is **`sift-stack-py`** (available on PyPI), but the new proto files need to be committed to the repo first before they're available in the published package.

## Usage

### In Jupyter Notebook

Copy the contents of `main.py` into a Jupyter notebook cell and run it. Then you can use the functions directly:

```python
# Create a user attribute key
key_id = create_user_attribute_key(
    name="department",
    description="User department",
    value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING
)

# Create a user attribute value
create_user_attribute_value(
    user_attribute_key_id=key_id,
    user_id="my_user",
    string_value="Engineering"
)
```

### As a Standalone Script

```bash
python main.py
```

This will print usage examples.

## Examples

### User Attributes

```python
# Create a user attribute key
key_id = create_user_attribute_key(
    name="department",
    description="User department",
    value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING
)

# Create a user attribute value
create_user_attribute_value(
    user_attribute_key_id=key_id,
    user_id="my_user",
    string_value="Engineering"
)

# List all user attribute keys
list_user_attribute_keys()

# Archive user attribute keys
archive_user_attribute_keys([key_id])
```

### Resource Attributes

```python
# Create a resource attribute key with enum type
resource_key_id = create_resource_attribute_key(
    display_name="environment",
    description="Deployment environment",
    key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM
)

# Create an enum value
enum_value_id = create_resource_attribute_enum_value(
    resource_attribute_key_id=resource_key_id,
    display_name="production"
)

# Create a resource attribute (assign to an asset)
create_resource_attribute(
    resource_attribute_key_id=resource_key_id,
    entity_id="asset_id",
    entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
    resource_attribute_enum_value_id=enum_value_id
)

# Update a resource attribute key
update_resource_attribute_key(
    resource_attribute_key_id=resource_key_id,
    display_name="updated_environment",
    description="Updated description"
)

# List all resource attribute keys
list_resource_attribute_keys()

# Archive resources
archive_resource_attribute_key(resource_key_id)
```

### Access Policies

```python
# Create a policy
policy_id = create_policy(
    name="Example Policy",
    cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" };',
    description="Example policy for engineering department"
)

# List all policies
list_policies()

# Update a policy
update_policy(
    policy_id=policy_id,
    cedar_policy='permit(principal, action, resource) when { principal.department == "Engineering" && resource.environment == "production" };',
    version_notes="Added environment check"
)

# Archive a policy
archive_policy(policy_id)
```

## Available Functions

### User Attributes
- `create_user_attribute_key()` - Create a user attribute key
- `list_user_attribute_keys()` - List all user attribute keys
- `create_user_attribute_value()` - Create a user attribute value
- `archive_user_attribute_keys()` - Archive user attribute keys
- `archive_user_attribute_values()` - Archive user attribute values

### Resource Attributes
- `create_resource_attribute_key()` - Create a resource attribute key
- `create_resource_attribute_enum_value()` - Create an enum value for enum-type keys
- `create_resource_attribute()` - Create a resource attribute (assign to entity)
- `update_resource_attribute_key()` - Update a resource attribute key
- `list_resource_attribute_keys()` - List all resource attribute keys
- `archive_resource_attribute_key()` - Archive a resource attribute key
- `archive_resource_attribute()` - Archive a resource attribute

### Access Policies
- `create_policy()` - Create an access policy
- `list_policies()` - List all policies
- `update_policy()` - Update a policy
- `archive_policy()` - Archive a policy

## Cedar Policy Syntax

Policies use the Cedar policy language. See the [Cedar documentation](https://docs.cedarpolicy.com/policies/syntax-policy.html) for syntax details.

Example policy:
```cedar
permit(principal, action, resource) 
when { 
    principal.department == "Engineering" && 
    resource.environment == "production" 
};
```

