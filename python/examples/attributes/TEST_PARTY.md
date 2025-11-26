# ABAC Resources Test Party Document

## Feature Overview

Attribute-Based Access Control (ABAC) enables fine-grained access control for Sift resources through a combination of:

- **User Attributes**: Metadata attached to users (principals) that describe their roles, teams, permissions, and organizational context
- **Resource Attributes**: Metadata attached to resources (channels, runs, assets) that describe their classification, ownership, status, and sharing settings
- **Access Policies**: Cedar-based authorization rules that evaluate user and resource attributes to grant or deny access

This feature allows you to implement complex access control scenarios such as:
- Protecting sensitive data from unauthorized modifications
- Enforcing team-based ownership and permissions
- Implementing approval workflows for critical operations
- Controlling external sharing and data visibility

## Setup Steps

### Step 1: Create a New User in Dev

Create a new user account in the dev environment to test ABAC functionality:

- **Suggested format**: `<your_email>+abac@siftstack.com`
- Register and confirm your email address
- This user will be used to test various ABAC scenarios

### Step 2: Get Your User ID

Query the database to find your `user_id` for the newly created user:

```sql
SELECT user_id, email FROM users WHERE email = '<your_email>+abac@siftstack.com';
```

Copy the `user_id` to paste into the Python notebook. You'll use this ID to assign user attributes.

**Note**: There is a script to add user_attributes to your user. Feel free to populate the fields however you wish for testing purposes.

### Step 3: Open and Run the Python Notebook

Open the Colab notebook:
**[https://colab.research.google.com/drive/1qt2Op1w1skgooJQdl6HgbSbAYtbeQCt4#scrollTo=EVPJbs8oiArB](https://colab.research.google.com/drive/1qt2Op1w1skgooJQdl6HgbSbAYtbeQCt4#scrollTo=EVPJbs8oiArB)**

Run the first few blocks which will:
- Install the Python client from GitHub
- Create Python client code to interact with ABAC resources
- Set up the necessary imports and configuration

For detailed Colab setup instructions, see [README_COLAB.md](README_COLAB.md).

### Step 4: Test Resources

Test resources have been created for testing policies. These can be used to experiment with different scenarios. You'll reference these resource IDs when assigning resource attributes in the examples below.

---

## Test Scenarios

## Scenario 1: Internal Users View All Resources

### Business Context

Internal users (users with `organization="sift"`) have full visibility and can view all resources (channels, runs, assets) in the system. This provides internal team members with comprehensive access to all data for operational and administrative purposes.

### Required Attributes

**User Attributes:**
- `organization` (STRING): The user's organization name
  - Users with `organization="sift"` are considered internal users and can view all resources

**Resource Attributes:**
- None required for this scenario

### Policies

This scenario uses 1 policy:

1. **Sift Organization Views All Resources**: Permits users with `organization="sift"` to view all channels, runs, and assets

### Complete Setup Code

```python
# Replace with your actual user_id from Step 2
USER_ID = "your-user-id-here"

# Replace with actual resource IDs you want to test with
CHANNEL_ID = "channel-id-1"
RUN_ID = "run-id-1"
ASSET_ID = "asset-id-1"

from sift.user_attributes.v1.user_attributes_pb2 import UserAttributeValueType

# ===== CREATE USER ATTRIBUTE KEY =====
with ABACClient(channel_config) as client:
    # Create organization attribute key
    organization_key_id = create_user_attribute_key(
        name="organization",
        description="User's organization",
        value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        client=client
    )
    
    # Assign user to sift organization (internal user)
    create_user_attribute_value(
        user_attribute_key_id=organization_key_id,
        user_id=USER_ID,
        string_value="sift",
        client=client
    )

# ===== CREATE POLICIES =====
internal_access_policies = [
    {
        'name': 'Sift Organization Views All Resources',
        'cedar_policy': '''permit (
    principal,
    action == SiftApp::Action::"view",
    resource
)
when
{
    principal has organization &&
    principal.organization == "sift"
};''',
        'description': 'Users from sift organization can view all channels, runs, and assets'
    }
]

with ABACClient(channel_config) as client:
    policy_ids = create_policies(internal_access_policies, client=client)

print("\n✅ Scenario 1 setup complete!")
print(f"Created {len(policy_ids)} policies for Internal Users View All Resources")
```

### Validation Steps

1. **Verify internal user can view all channels**: As a user with `organization="sift"`, attempt to view any channel. This should succeed.

2. **Verify internal user can view all runs**: As a user with `organization="sift"`, attempt to view any run. This should succeed.

3. **Verify internal user can view all assets**: As a user with `organization="sift"`, attempt to view any asset. This should succeed.

---

## Scenario 2: External Channel Sharing

### Business Context

Enable external users (users with `organization != "sift"`) to view resources that are explicitly marked as shareable with external users. This allows controlled sharing of specific channels, runs, and assets with external partners while maintaining security boundaries.

**Key Rule**: If a user's organization is not "sift", they can only view resources that have the `external_user=true` resource attribute. This ensures external users have limited, explicit access to only shared resources.

### Required Attributes

**User Attributes:**
- `organization` (STRING): The user's organization name
  - Users with `organization != "sift"` are considered external users

**Resource Attributes:**
- `external_user` (BOOLEAN): Flag indicating if the resource is shareable with external users

### Policies

This scenario uses 1 policy:

1. **Non-Sift Users View External-Shareable Resources**: Permits users with `organization != "sift"` to view resources where `external_user=true`

### Complete Setup Code

```python
# Replace with your actual user_id from Step 2
USER_ID_EXTERNAL = "your-external-user-id"

# Replace with actual resource IDs you want to test with
CHANNEL_ID_SHARED = "channel-id-shared"  # Resource with external_user=true
CHANNEL_ID_INTERNAL = "channel-id-internal"  # Resource without external_user attribute

from sift.user_attributes.v1.user_attributes_pb2 import UserAttributeValueType
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttributeKeyType,
    ResourceAttributeEntityType
)

# ===== CREATE USER ATTRIBUTE KEY =====
with ABACClient(channel_config) as client:
    # Create organization attribute key (if not already created)
    organization_key_id = create_user_attribute_key(
        name="organization",
        description="User's organization",
        value_type=UserAttributeValueType.USER_ATTRIBUTE_VALUE_TYPE_STRING,
        client=client
    )
    
    # Assign user to external organization (not "sift")
    create_user_attribute_value(
        user_attribute_key_id=organization_key_id,
        user_id=USER_ID_EXTERNAL,
        string_value="partner-org",
        client=client
    )

# ===== CREATE RESOURCE ATTRIBUTE KEY =====
with ABACClient(channel_config) as client:
    # Create external_user (boolean) resource attribute
    external_user_key_id = create_resource_attribute_key(
        display_name="external_user",
        description="Resource is shareable with external users",
        key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN,
        client=client
    )

# ===== ASSIGN RESOURCE ATTRIBUTES TO RESOURCES =====
with ABACClient(channel_config) as client:
    # Channel 1: Shareable with external users
    create_resource_attribute(
        resource_attribute_key_id=external_user_key_id,
        entity_id=CHANNEL_ID_SHARED,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_CHANNEL,
        boolean_value=True,
        client=client
    )
    
    # Channel 2: Not shareable (no external_user attribute set)

# ===== CREATE POLICIES =====
external_sharing_policies = [
    {
        'name': 'Non-Sift Users View External-Shareable Resources',
        'cedar_policy': '''permit (
    principal,
    action == SiftApp::Action::"view",
    resource
)
when
{
    principal has organization &&
    principal.organization != "sift" &&
    resource has external_user &&
    resource.external_user == true
};''',
        'description': 'Users from non-sift organizations can view resources marked as external_user=true'
    }
]

with ABACClient(channel_config) as client:
    policy_ids = create_policies(external_sharing_policies, client=client)

print("\n✅ Scenario 2 setup complete!")
print(f"Created {len(policy_ids)} policies for External Channel Sharing")
```

### Validation Steps

1. **Verify external user can view shared resource**: As a user with `organization="partner-org"` (not "sift"), attempt to view `CHANNEL_ID_SHARED` (which has `external_user=true`). This should succeed.

2. **Verify external user cannot view non-shared resource**: As a user with `organization="partner-org"`, attempt to view `CHANNEL_ID_INTERNAL` (which does not have `external_user=true`). This should be denied.

3. **Verify internal user can still view all**: As a user with `organization="sift"`, attempt to view both `CHANNEL_ID_SHARED` and `CHANNEL_ID_INTERNAL`. Both should succeed (due to Scenario 1 policy).

---

## Scenario 3: Mission Run Approval Workflow

### Business Context

Implement an approval workflow for mission runs, ensuring that runs requiring approval cannot be used in analysis until they are approved by an authorized user. This enforces quality gates and compliance requirements.

### Required Attributes

**User Attributes:**
- Note: This scenario uses `SiftApp::UserGroup::"engineering"` which is a user group, not a user attribute. User groups are managed separately from user attributes.

**Resource Attributes:**
- `approval_required` (BOOLEAN): Flag indicating if the run requires approval
- `approval_status` (ENUM): Approval status with values: "approved", "pending", "rejected"

### Policies

This scenario uses 3 policies:

1. **Deny Use Unapproved Runs in Analysis**: Blocks use of runs in analysis if approval is required but status is not "approved"
2. **Engineering Can Use Approved Runs in Analysis**: Allows engineering users to use approved runs in analysis
3. **Engineering Can Use Non-Approval-Required Runs**: Allows engineering users to use runs that don't require approval

**Note**: The "approve" action is a write operation and is not currently supported. This scenario focuses on read operations (using runs in analysis).

### Complete Setup Code

```python
# Replace with your actual user_id from Step 2
USER_ID = "your-user-id-here"

# Replace with actual run IDs you want to test with
RUN_ID_APPROVED = "run-id-approved"
RUN_ID_PENDING = "run-id-pending"
RUN_ID_NO_APPROVAL = "run-id-no-approval"

from sift.user_attributes.v1.user_attributes_pb2 import UserAttributeValueType
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttributeKeyType,
    ResourceAttributeEntityType
)

# ===== CREATE USER ATTRIBUTE KEY =====
# Note: This scenario uses SiftApp::UserGroup::"engineering" which is a user group,
# not a user attribute. User groups are managed separately from user attributes.
# If you need to assign users to groups, that is done through a different API.
# For this test, ensure your test user is already in the "engineering" user group.

# ===== CREATE RESOURCE ATTRIBUTE KEYS =====
with ABACClient(channel_config) as client:
    # Create approval_required (boolean)
    approval_required_key_id = create_resource_attribute_key(
        display_name="approval_required",
        description="Whether the run requires approval",
        key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN,
        client=client
    )
    
    # Create approval_status (enum)
    approval_status_key_id = create_resource_attribute_key(
        display_name="approval_status",
        description="Approval status of the run",
        key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
        client=client
    )
    
    # Create enum values for approval_status
    approved_enum_id = create_resource_attribute_enum_value(
        resource_attribute_key_id=approval_status_key_id,
        display_name="approved",
        description="Run is approved",
        client=client
    )
    pending_enum_id = create_resource_attribute_enum_value(
        resource_attribute_key_id=approval_status_key_id,
        display_name="pending",
        description="Approval pending",
        client=client
    )
    rejected_enum_id = create_resource_attribute_enum_value(
        resource_attribute_key_id=approval_status_key_id,
        display_name="rejected",
        description="Approval rejected",
        client=client
    )

# ===== ASSIGN RESOURCE ATTRIBUTES TO RUNS =====
with ABACClient(channel_config) as client:
    # Run 1: Approved run (can be used)
    create_resource_attribute(
        resource_attribute_key_id=approval_required_key_id,
        entity_id=RUN_ID_APPROVED,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        boolean_value=True,
        client=client
    )
    create_resource_attribute(
        resource_attribute_key_id=approval_status_key_id,
        entity_id=RUN_ID_APPROVED,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        resource_attribute_enum_value_id=approved_enum_id,
        client=client
    )
    
    # Run 2: Pending approval (cannot be used)
    create_resource_attribute(
        resource_attribute_key_id=approval_required_key_id,
        entity_id=RUN_ID_PENDING,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        boolean_value=True,
        client=client
    )
    create_resource_attribute(
        resource_attribute_key_id=approval_status_key_id,
        entity_id=RUN_ID_PENDING,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        resource_attribute_enum_value_id=pending_enum_id,
        client=client
    )
    
    # Run 3: No approval required (can be used)
    create_resource_attribute(
        resource_attribute_key_id=approval_required_key_id,
        entity_id=RUN_ID_NO_APPROVAL,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        boolean_value=False,
        client=client
    )

# ===== CREATE POLICIES =====
approval_workflow_policies = [
    {
        'name': 'Deny Use Unapproved Runs in Analysis',
        'cedar_policy': '''forbid (
    principal,
    action == SiftApp::Action::"use-in-analysis",
    resource
)
when
{
    resource has approval_required &&
    resource.approval_required == true &&
    resource has approval_status &&
    resource.approval_status != "approved"
};''',
        'description': 'Deny use of runs in analysis if approval is required but not approved'
    },
    {
        'name': 'Engineering Can Use Approved Runs in Analysis',
        'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"use-in-analysis",
    resource is SiftApp::Run
)
when
{
    resource has approval_status &&
    resource.approval_status == "approved"
};''',
        'description': 'Allow use of approved runs in analysis'
    },
    {
        'name': 'Engineering Can Use Non-Approval-Required Runs',
        'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"use-in-analysis",
    resource is SiftApp::Run
)
when
{
    resource has approval_required &&
    resource.approval_required == false
};''',
        'description': 'Allow use of runs that don\'t require approval'
    }
]

with ABACClient(channel_config) as client:
    policy_ids = create_policies(approval_workflow_policies, client=client)

print("\n✅ Scenario 3 setup complete!")
print(f"Created {len(policy_ids)} policies for Mission Run Approval Workflow")
```

### Validation Steps

1. **Verify approved run can be used**: As an engineering user, attempt to use `RUN_ID_APPROVED` in analysis. This should succeed.

2. **Verify pending run cannot be used**: As an engineering user, attempt to use `RUN_ID_PENDING` in analysis. This should be denied.

3. **Verify non-approval-required run can be used**: As an engineering user, attempt to use `RUN_ID_NO_APPROVAL` in analysis. This should succeed.

---

## Quick Reference: Function Usage

### User Attributes Functions

- **`create_user_attribute_key(name, description, value_type, organization_id=None, client=None)`**
  - Creates a new user attribute key (e.g., "department", "team_memberships")
  - Returns the `user_attribute_key_id`

- **`create_user_attribute_value(user_attribute_key_id, user_id, string_value=None, number_value=None, boolean_value=None, organization_id=None, client=None)`**
  - Assigns an attribute value to a specific user
  - Returns the `user_attribute_value_id`

- **`list_user_attribute_keys(organization_id=None, include_archived=False, client=None)`**
  - Lists all user attribute keys

- **`archive_user_attribute_keys(user_attribute_key_ids, client=None)`**
  - Archives one or more user attribute keys

### Resource Attributes Functions

- **`create_resource_attribute_key(display_name, description, key_type, organization_id=None, initial_enum_values=None, client=None)`**
  - Creates a new resource attribute key
  - For enum types, you can optionally provide initial enum values
  - Returns the `resource_attribute_key_id`

- **`create_resource_attribute_enum_value(resource_attribute_key_id, display_name, description=None, client=None)`**
  - Creates an enum value for an enum-type resource attribute key
  - Returns the `resource_attribute_enum_value_id`

- **`create_resource_attribute(resource_attribute_key_id, entity_id, entity_type, resource_attribute_enum_value_id=None, boolean_value=None, number_value=None, client=None)`**
  - Assigns a resource attribute to an entity (asset or channel)
  - Returns the `resource_attribute_id`

- **`list_resource_attribute_keys(organization_id=None, include_archived=False, client=None)`**
  - Lists all resource attribute keys

- **`archive_resource_attribute_key(resource_attribute_key_id, client=None)`**
  - Archives a resource attribute key

### Access Policies Functions

- **`create_policy(name, cedar_policy, description=None, version_notes=None, client=None)`**
  - Creates a single access policy
  - Returns the `policy_id`

- **`create_policies(policies, client=None)`**
  - Creates multiple policies from a list of policy dictionaries
  - Each dictionary should have: `name`, `cedar_policy`, `description` (optional), `version_notes` (optional)
  - Returns a dictionary mapping policy names to their IDs

- **`list_policies(include_archived=False, client=None)`**
  - Lists all policies

- **`update_policy(policy_id, name=None, description=None, cedar_policy=None, version_notes=None, client=None)`**
  - Updates an existing policy

- **`archive_policy(policy_id, client=None)`**
  - Archives a policy

### Batch Operations

For multiple operations, use the `ABACClient` context manager to reuse a single gRPC channel:

```python
with ABACClient(channel_config) as client:
    # Create multiple attribute keys
    key1 = create_user_attribute_key("attr1", "...", ..., client=client)
    key2 = create_user_attribute_key("attr2", "...", ..., client=client)
    
    # Assign multiple values
    create_user_attribute_value(key1, user_id, string_value="value1", client=client)
    create_user_attribute_value(key2, user_id, string_value="value2", client=client)
    
    # Create multiple policies
    policy_ids = create_policies(policy_list, client=client)
```

---

## Future Test Scenarios

The following scenario tests write operations, which are not currently supported. This scenario is included for future testing when write endpoint enforcement is implemented.

---

## Future Scenario: Run Write Protection

### Business Context

Protect mission run data integrity by preventing unauthorized modifications to runs that have been ingested, approved, or archived. This ensures data quality and prevents accidental overwrites of processed data.

**⚠️ Note**: This scenario tests write operations (`edit`, `delete`, `update`, `ingest`) which are not currently supported. This scenario should be tested once write endpoint enforcement is implemented.

### Required Attributes

**User Attributes:**
- Note: This scenario uses `SiftApp::UserGroup::"engineering"` which is a user group, not a user attribute. User groups are managed separately from user attributes.

**Resource Attributes:**
- `protected_data_overwrite` (BOOLEAN): Flag indicating if data overwrite is protected
- `status` (ENUM): Run status with values: "draft", "ingested", "approved", "archived"

### Policies

This scenario uses 5 policies:

1. **Deny Overwrite Protected Data Runs**: Blocks any write operations on runs where `protected_data_overwrite == true`
2. **Deny Overwrite Ingested/Approved/Archived Runs**: Blocks write operations on runs with status "ingested", "approved", or "archived"
3. **Allow Editing Draft Runs**: Permits engineering users to edit runs in "draft" status
4. **Engineering Read All Runs**: Allows engineering users to view all runs
5. **Engineering Ingest Runs**: Allows engineering users to ingest runs (unless forbidden by other policies)

### Complete Setup Code

```python
# Replace with your actual user_id from Step 2
USER_ID = "your-user-id-here"

# Replace with actual run IDs you want to test with
RUN_ID_1 = "run-id-1"  # For draft run
RUN_ID_2 = "run-id-2"  # For protected run
RUN_ID_3 = "run-id-3"  # For ingested run

from sift.user_attributes.v1.user_attributes_pb2 import UserAttributeValueType
from sift.resource_attribute.v1.resource_attribute_pb2 import (
    ResourceAttributeKeyType,
    ResourceAttributeEntityType
)

# ===== CREATE USER ATTRIBUTE KEY =====
# Note: This scenario uses SiftApp::UserGroup::"engineering" which is a user group,
# not a user attribute. User groups are managed separately from user attributes.
# If you need to assign users to groups, that is done through a different API.
# For this test, ensure your test user is already in the "engineering" user group.

# ===== CREATE RESOURCE ATTRIBUTE KEYS =====
with ABACClient(channel_config) as client:
    # Create protected_data_overwrite (boolean)
    protected_key_id = create_resource_attribute_key(
        display_name="protected_data_overwrite",
        description="Flag to protect data from overwrite",
        key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_BOOLEAN,
        client=client
    )
    
    # Create status (enum)
    status_key_id = create_resource_attribute_key(
        display_name="status",
        description="Run status",
        key_type=ResourceAttributeKeyType.RESOURCE_ATTRIBUTE_KEY_TYPE_ENUM,
        client=client
    )
    
    # Create enum values for status
    draft_enum_id = create_resource_attribute_enum_value(
        resource_attribute_key_id=status_key_id,
        display_name="draft",
        description="Draft status",
        client=client
    )
    ingested_enum_id = create_resource_attribute_enum_value(
        resource_attribute_key_id=status_key_id,
        display_name="ingested",
        description="Ingested status",
        client=client
    )
    approved_enum_id = create_resource_attribute_enum_value(
        resource_attribute_key_id=status_key_id,
        display_name="approved",
        description="Approved status",
        client=client
    )
    archived_enum_id = create_resource_attribute_enum_value(
        resource_attribute_key_id=status_key_id,
        display_name="archived",
        description="Archived status",
        client=client
    )

# ===== ASSIGN RESOURCE ATTRIBUTES TO RUNS =====
with ABACClient(channel_config) as client:
    # Run 1: Draft run (can be edited)
    create_resource_attribute(
        resource_attribute_key_id=status_key_id,
        entity_id=RUN_ID_1,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        resource_attribute_enum_value_id=draft_enum_id,
        client=client
    )
    
    # Run 2: Protected run (cannot be overwritten)
    create_resource_attribute(
        resource_attribute_key_id=protected_key_id,
        entity_id=RUN_ID_2,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        boolean_value=True,
        client=client
    )
    
    # Run 3: Ingested run (cannot be overwritten)
    create_resource_attribute(
        resource_attribute_key_id=status_key_id,
        entity_id=RUN_ID_3,
        entity_type=ResourceAttributeEntityType.RESOURCE_ATTRIBUTE_ENTITY_TYPE_ASSET,
        resource_attribute_enum_value_id=ingested_enum_id,
        client=client
    )

# ===== CREATE POLICIES =====
run_protection_policies = [
    {
        'name': 'Deny Overwrite Protected Data Runs',
        'cedar_policy': '''forbid (
    principal,
    action in [SiftApp::Action::"edit", SiftApp::Action::"delete", SiftApp::Action::"update", SiftApp::Action::"ingest"],
    resource
)
when
{
    resource has protected_data_overwrite &&
    resource.protected_data_overwrite == true
};''',
        'description': 'Deny data overwrite on runs where protected_data_overwrite flag is true'
    },
    {
        'name': 'Deny Overwrite Ingested/Approved/Archived Runs',
        'cedar_policy': '''forbid (
    principal,
    action in [SiftApp::Action::"edit", SiftApp::Action::"delete", SiftApp::Action::"update", SiftApp::Action::"ingest"],
    resource
)
when
{
    resource has status &&
    (resource.status == "ingested" || 
     resource.status == "approved" || 
     resource.status == "archived")
};''',
        'description': 'Deny data overwrite on runs that have been ingested, approved, or archived'
    },
    {
        'name': 'Allow Editing Draft Runs',
        'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action in [SiftApp::Action::"edit", SiftApp::Action::"update"],
    resource
)
when
{
    resource has status &&
    resource.status == "draft"
};''',
        'description': 'Allow editing runs in draft status'
    },
    {
        'name': 'Engineering Read All Runs',
        'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"view",
    resource is SiftApp::Run
);''',
        'description': 'Allow read access to all runs for engineering'
    },
    {
        'name': 'Engineering Ingest Runs',
        'cedar_policy': '''permit (
    principal in SiftApp::UserGroup::"engineering",
    action == SiftApp::Action::"ingest",
    resource is SiftApp::Run
);''',
        'description': 'Allow ingesting runs for engineering users (unless forbidden by other policies)'
    }
]

with ABACClient(channel_config) as client:
    policy_ids = create_policies(run_protection_policies, client=client)

print("\n✅ Future Scenario setup complete!")
print(f"Created {len(policy_ids)} policies for Run Write Protection")
print("⚠️  Note: Write operations are not currently supported. This scenario should be tested once write endpoint enforcement is implemented.")
```

### Validation Steps

**⚠️ These validation steps require write endpoint enforcement, which is not currently implemented.**

1. **Verify draft run can be edited**: As an engineering user, attempt to edit `RUN_ID_1` (draft status). This should succeed.

2. **Verify protected run cannot be edited**: Attempt to edit `RUN_ID_2` (protected_data_overwrite=true). This should be denied.

3. **Verify ingested run cannot be edited**: Attempt to edit `RUN_ID_3` (status=ingested). This should be denied.

4. **Verify read access**: As an engineering user, attempt to view all three runs. This should succeed.

5. **Verify ingest permission**: As an engineering user, attempt to ingest a new run. This should succeed (unless other policies forbid it).

---

## Validation Guide

### General Validation Approach

1. **Verify attributes are created**: Use `list_user_attribute_keys()` and `list_resource_attribute_keys()` to confirm your attributes exist.

2. **Verify attributes are assigned**: Check that user attributes are assigned to your test user and resource attributes are assigned to your test resources.

3. **Verify policies are created**: Use `list_policies()` to confirm all policies are created and active.

4. **Test access control**: Attempt the operations that should be allowed/denied according to your policies and verify the results match expectations.

### Common Issues and Troubleshooting

1. **"Stream removed" errors**: Use `ABACClient` context manager for multiple operations instead of creating new channels for each call.

2. **Attributes not found in policies**: Ensure attribute keys are created before assigning values, and values are assigned before policies are evaluated.

3. **Policies not taking effect**: Verify that:
   - Policies are created (not archived)
   - Required attributes exist on both users and resources
   - Attribute values match what the policies expect (e.g., exact string matches, correct enum values)

4. **Permission denied errors**: Check that:
   - Your user has the necessary attributes (e.g., `team_memberships` contains the expected value)
   - Resources have the necessary attributes
   - Policies are correctly written and active

### Testing Tips

- Start with simple scenarios and gradually add complexity
- Test both positive cases (should succeed) and negative cases (should fail)
- Use different users to test different permission levels
- Verify that policies work together correctly (e.g., a permit policy doesn't override a forbid policy)

