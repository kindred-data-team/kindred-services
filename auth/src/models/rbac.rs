use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Permission {
    path: String
}

impl Permission {
    fn new(path: &str) -> Self {
        Permission {
            path: path.to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct Role {
    name: String,
    permissions: HashSet<String>
}

impl Role {
    pub fn new(name: &str) -> Self {
        Role {
            name: name.to_string(),
            permissions: HashSet::new()
        }
    }

    pub fn add_role(&mut self, permission: &str) {
        self.permissions.insert(permission.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct User {
    id: String,
    roles: HashSet<String>,
    direct_permissions: HashSet<String>,
    denied_permissions: HashSet<String>
}

impl User {
    pub fn new(id: &str) -> Self {
        User {
            id: id.to_string(),
            roles: HashSet::new(),
            direct_permissions: HashSet::new(),
            denied_permissions: HashSet::new(),
        }
    }

    pub fn add_role(&mut self, role: &str) {
        self.roles.insert(role.to_string());
    }

    pub fn add_direct_permission(&mut self, permission: &str) {
        self.direct_permissions.insert(permission.to_string());
    }

    pub fn add_denied_permission(&mut self, permission: &str) {
        self.denied_permissions.insert(permission.to_string());
    }
}

#[derive(Debug, Clone)]
pub struct PermissionTrie {
    root: TriedNode
}

#[derive(Debug, Clone)]
pub struct TriedNode {
    children: HashMap<String, TriedNode>,
    is_wildcard: bool,
    is_endpoint: bool,
}

impl TriedNode {
    pub fn new() -> Self {
        TriedNode {children: HashMap::new(), is_wildcard: false, is_endpoint: false}
    }
}

impl PermissionTrie {
    pub fn new() -> Self {
        PermissionTrie {root: TriedNode::new()}
    }

    pub fn insert(&mut self, permission: &str) {
        let mut node = &mut self.root;
        let parts: Vec<&str> = permission.trim_end_matches('/').split('/').collect();

        for part in parts {
            if part == "*" {
                node.is_wildcard = true;
                break;
            }

            node = node.children.entry(part.to_string()).or_insert(TriedNode::new());
        }
        node.is_endpoint = true;
    }

    pub fn has_permission(&self, path: &str) -> bool {
        let parts: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        let mut node= &self.root;
    
        for part in parts {
            if node.is_wildcard {
                return true;
            }
    
            if let Some(next_node) = node.children.get(part) {
                node = next_node;
            } else {
                return false;
            }
        }
    
        node.is_endpoint || node.is_wildcard
    }
}

struct RBAC {
    roles: HashMap<String, Role>,
    users: HashMap<String, User>,
    permission_trie: PermissionTrie,
}

impl RBAC {
    pub fn new() -> Self {
        RBAC {
            roles: HashMap::new(),
            users: HashMap::new(),
            permission_trie: PermissionTrie::new(),
        }
    }

    pub fn add_role(&mut self, role: Role) {
        self.roles.insert(role.name.clone(), role);
    }

    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.id.clone(), user);
    }

    pub fn check_resource_access(&self, rbac_id: &str, resource_scope: &str) -> bool {
        if let Some(profile) = self.users.get(rbac_id) {
            if profile.denied_permissions.contains(resource_scope) {
                return false;
            }

            if profile.direct_permissions.contains(resource_scope) {
                return true;
            }

            for role_name in &profile.roles {
                if let Some(role) = self.roles.get(role_name) {
                    if role.permissions.contains(resource_scope) {
                        return true;
                    }

                    for permission in &role.permissions {
                        if permission.ends_with("*") && resource_scope.starts_with(permission.trim_end_matches('*')) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

#[derive(Debug, Clone)]
pub struct RbacProfile {
    id: String,
    roles: HashSet<String>,
    direct_permissions: HashSet<String>,
    denied_permissions: HashSet<String>,
}