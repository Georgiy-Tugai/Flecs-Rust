use super::c_binding::bindings::*;
use super::component_registration::{ComponentType, Struct};
use crate::core::component_registration::{CachedComponentData, ComponentData};

use std::ffi::CStr;
use std::sync::OnceLock;

pub const RUST_ECS_ID_FLAGS_MASK: u64 = 0xFF << 60;
pub const RUST_ECS_COMPONENT_MASK: u64 = !RUST_ECS_ID_FLAGS_MASK;

pub type WorldT = ecs_world_t;
pub type WorldInfoT = ecs_world_info_t;
pub type QueryGroupInfoT = ecs_query_group_info_t;
pub type IdT = ecs_id_t;
pub type EntityT = ecs_entity_t;
pub type TypeT = ecs_type_t;
pub type TableT = ecs_table_t;
pub type FilterT = ecs_filter_t;
pub type ObserverT = ecs_observer_t;
pub type QueryT = ecs_query_t;
pub type RuleT = ecs_rule_t;
pub type RefT = ecs_ref_t;
pub type IterT = ecs_iter_t;
pub type TypeInfoT = ecs_type_info_t;
pub type TypeHooksT = ecs_type_hooks_t;
pub type Flags32T = ecs_flags32_t;
pub type TermIdT = ecs_term_id_t;
pub type TermT = ecs_term_t;

pub static SEPARATOR: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"::\0") };

#[repr(C)]
pub enum InOutKind {
    InOutDefault = 0, // InOut for regular terms, In for shared terms
    InOutNone = 1,    // Term is neither read nor written
    InOut = 2,        // Term is both read and written
    In = 3,           // Term is only read
    Out = 4,          // Term is only written
}

impl InOutKind {
    pub fn is_read_only(&self) -> bool {
        matches!(self, Self::In)
    }
}

impl From<::std::os::raw::c_int> for InOutKind {
    fn from(value: ::std::os::raw::c_int) -> Self {
        match value {
            0 => InOutKind::InOutDefault,
            1 => InOutKind::InOutNone,
            2 => InOutKind::InOut,
            3 => InOutKind::In,
            4 => InOutKind::Out,
            _ => InOutKind::InOutDefault,
        }
    }
}

#[repr(C)]
pub enum OperKind {
    And,      // The term must match
    Or,       // One of the terms in an or chain must match
    Not,      // The term must not match
    Optional, // The term may match
    AndFrom,  // Term must match all components from term id
    OrFrom,   // Term must match at least one component from term id
    NotFrom,  // Term must match none of the components from term id
}

impl OperKind {
    pub fn is_negation(&self) -> bool {
        matches!(self, Self::Not | Self::NotFrom)
    }
}

impl From<::std::os::raw::c_int> for OperKind {
    fn from(value: ::std::os::raw::c_int) -> Self {
        match value {
            0 => OperKind::And,
            1 => OperKind::Or,
            2 => OperKind::Not,
            3 => OperKind::Optional,
            4 => OperKind::AndFrom,
            5 => OperKind::OrFrom,
            6 => OperKind::NotFrom,
            _ => OperKind::And,
        }
    }
}

impl Default for TypeHooksT {
    fn default() -> Self {
        TypeHooksT {
            ctor: None,
            dtor: None,
            copy: None,
            move_: None,
            copy_ctor: None,
            move_ctor: None,
            ctor_move_dtor: None,
            move_dtor: None,
            on_add: None,
            on_set: None,
            on_remove: None,
            ctx: std::ptr::null_mut(),
            binding_ctx: std::ptr::null_mut(),
            ctx_free: None,
            binding_ctx_free: None,
        }
    }
}

// Id flags
pub const ECS_PAIR: u64 = 1 << 63;
pub const ECS_OVERRIDE: u64 = 1 << 62;
pub const ECS_TOGGLE: u64 = 1 << 61;
pub const ECS_AND: u64 = 1 << 60;

// Builtin component ids
pub const ECS_COMPONENT: u64 = 1;
pub const ECS_IDENTIFIER: u64 = 2;
pub const ECS_ITERABLE: u64 = 3;
pub const ECS_POLY: u64 = 4;

// Poly target components
pub const ECS_QUERY: u64 = 5;
pub const ECS_OBSERVER: u64 = 6;
pub const ECS_SYSTEM: u64 = 7;

// The base ID, equivalent to the C #define
pub const FLECS_HI_COMPONENT_ID: u64 = 256;

// Core scopes & entities
pub const ECS_WORLD: u64 = FLECS_HI_COMPONENT_ID;
pub const ECS_FLECS: u64 = FLECS_HI_COMPONENT_ID + 1;
pub const ECS_FLECS_CORE: u64 = FLECS_HI_COMPONENT_ID + 2;
pub const ECS_FLECS_INTERNALS: u64 = FLECS_HI_COMPONENT_ID + 3;
pub const ECS_MODULE: u64 = FLECS_HI_COMPONENT_ID + 4;
pub const ECS_PRIVATE: u64 = FLECS_HI_COMPONENT_ID + 5;
pub const ECS_PREFAB: u64 = FLECS_HI_COMPONENT_ID + 6;
pub const ECS_DISABLED: u64 = FLECS_HI_COMPONENT_ID + 7;
pub const ECS_SLOT_OF: u64 = FLECS_HI_COMPONENT_ID + 8;
pub const ECS_FLAG: u64 = FLECS_HI_COMPONENT_ID + 9;

// Relationship properties
pub const ECS_WILDCARD: u64 = FLECS_HI_COMPONENT_ID + 10;
pub const ECS_ANY: u64 = FLECS_HI_COMPONENT_ID + 11;
pub const ECS_THIS: u64 = FLECS_HI_COMPONENT_ID + 12;
pub const ECS_VARIABLE: u64 = FLECS_HI_COMPONENT_ID + 13;
pub const ECS_TRANSITIVE: u64 = FLECS_HI_COMPONENT_ID + 14;
pub const ECS_REFLEXIVE: u64 = FLECS_HI_COMPONENT_ID + 15;
pub const ECS_SYMMETRIC: u64 = FLECS_HI_COMPONENT_ID + 16;
pub const ECS_FINAL: u64 = FLECS_HI_COMPONENT_ID + 17;
pub const ECS_DONT_INHERIT: u64 = FLECS_HI_COMPONENT_ID + 18;
pub const ECS_ALWAYS_OVERRIDE: u64 = FLECS_HI_COMPONENT_ID + 19;
pub const ECS_TAG: u64 = FLECS_HI_COMPONENT_ID + 20;
pub const ECS_UNION: u64 = FLECS_HI_COMPONENT_ID + 21;
pub const ECS_EXCLUSIVE: u64 = FLECS_HI_COMPONENT_ID + 22;
pub const ECS_ACYCLIC: u64 = FLECS_HI_COMPONENT_ID + 23;
pub const ECS_TRAVERSABLE: u64 = FLECS_HI_COMPONENT_ID + 24;
pub const ECS_WITH: u64 = FLECS_HI_COMPONENT_ID + 25;
pub const ECS_ONE_OF: u64 = FLECS_HI_COMPONENT_ID + 26;

// Builtin relationships
pub const ECS_CHILD_OF: u64 = FLECS_HI_COMPONENT_ID + 27;
pub const ECS_IS_A: u64 = FLECS_HI_COMPONENT_ID + 28;
pub const ECS_DEPENDS_ON: u64 = FLECS_HI_COMPONENT_ID + 29;

// Identifier tags
pub const ECS_NAME: u64 = FLECS_HI_COMPONENT_ID + 30;
pub const ECS_SYMBOL: u64 = FLECS_HI_COMPONENT_ID + 31;
pub const ECS_ALIAS: u64 = FLECS_HI_COMPONENT_ID + 32;

// Events
pub const ECS_ON_ADD: u64 = FLECS_HI_COMPONENT_ID + 33;
pub const ECS_ON_REMOVE: u64 = FLECS_HI_COMPONENT_ID + 34;
pub const ECS_ON_SET: u64 = FLECS_HI_COMPONENT_ID + 35;
pub const ECS_UNSET: u64 = FLECS_HI_COMPONENT_ID + 36;
pub const ECS_ON_DELETE: u64 = FLECS_HI_COMPONENT_ID + 37;
pub const ECS_ON_TABLE_CREATE: u64 = FLECS_HI_COMPONENT_ID + 38;
pub const ECS_ON_TABLE_DELETE: u64 = FLECS_HI_COMPONENT_ID + 39;
pub const ECS_ON_TABLE_EMPTY: u64 = FLECS_HI_COMPONENT_ID + 40;
pub const ECS_ON_TABLE_FILL: u64 = FLECS_HI_COMPONENT_ID + 41;
pub const ECS_ON_CREATE_TRIGGER: u64 = FLECS_HI_COMPONENT_ID + 42;
pub const ECS_ON_DELETE_TRIGGER: u64 = FLECS_HI_COMPONENT_ID + 43;
pub const ECS_ON_DELETE_OBSERVABLE: u64 = FLECS_HI_COMPONENT_ID + 44;
pub const ECS_ON_COMPONENT_HOOKS: u64 = FLECS_HI_COMPONENT_ID + 45;
pub const ECS_ON_DELETE_TARGET: u64 = FLECS_HI_COMPONENT_ID + 46;

pub type Component = EcsComponent;
pub type Identifier = EcsIdentifier;
pub type Poly = EcsPoly;
pub type Target = EcsTarget;

#[allow(clippy::derivable_impls)]
impl Default for EcsComponent {
    fn default() -> Self {
        Self {
            size: Default::default(),
            alignment: Default::default(),
        }
    }
}

fn get_ecs_component_data() -> ComponentData {
    ComponentData {
        id: unsafe { FLECS__EEcsComponent },
        size: std::mem::size_of::<EcsComponent>(),
        alignment: std::mem::align_of::<EcsComponent>(),
        allow_tag: true,
    }
}

impl ComponentType<Struct> for EcsComponent {}

impl CachedComponentData for EcsComponent {
    fn register_explicit(_world: *mut WorldT) {
        //this is already registered as FLECS__EEcsComponent
        Self::__get_once_lock_data().get_or_init(get_ecs_component_data);
    }

    fn is_registered() -> bool {
        Self::__get_once_lock_data().get().is_some()
    }

    fn is_registered_with_world(world: *mut WorldT) -> bool {
        if Self::is_registered() {
            //because this is always registered in the c world
            true
        } else {
            Self::register_explicit(world);
            true
        }
    }

    fn get_data(_world: *mut WorldT) -> &'static ComponentData {
        Self::__get_once_lock_data().get_or_init(get_ecs_component_data)
    }

    fn get_id(_world: *mut WorldT) -> IdT {
        Self::__get_once_lock_data()
            .get_or_init(get_ecs_component_data)
            .id
    }

    fn get_size(_world: *mut WorldT) -> usize {
        Self::__get_once_lock_data()
            .get_or_init(get_ecs_component_data)
            .size
    }

    fn get_alignment(_world: *mut WorldT) -> usize {
        Self::__get_once_lock_data()
            .get_or_init(get_ecs_component_data)
            .alignment
    }

    fn get_allow_tag(_world: *mut WorldT) -> bool {
        Self::__get_once_lock_data()
            .get_or_init(get_ecs_component_data)
            .allow_tag
    }

    fn __get_once_lock_data() -> &'static OnceLock<ComponentData> {
        static ONCE_LOCK: OnceLock<ComponentData> = OnceLock::new();
        &ONCE_LOCK
    }

    fn get_symbol_name_c() -> &'static str {
        use std::any::type_name;
        static SYMBOL_NAME_C: OnceLock<String> = OnceLock::new();
        SYMBOL_NAME_C.get_or_init(|| String::from("EcsComponent\0"))
    }

    fn get_symbol_name() -> &'static str {
        let name = Self::get_symbol_name_c();
        &name[..name.len() - 1]
    }
}

/// Match on self
pub const ECS_SELF: u32 = 1 << 1;

/// Match by traversing upwards
pub const ECS_UP: u32 = 1 << 2;

/// Match by traversing downwards (derived, cannot be set)
pub const ECS_DOWN: u32 = 1 << 3;

/// Match all entities encountered through traversal
pub const ECS_TRAVERSE_ALL: u32 = 1 << 4;

/// Sort results breadth first
pub const ECS_CASCADE: u32 = 1 << 5;

/// Short for up(ChildOf)
pub const ECS_PARENT: u32 = 1 << 6;

/// Term id is a variable
pub const ECS_IS_VARIABLE: u32 = 1 << 7;

/// Term id is an entity
pub const ECS_IS_ENTITY: u32 = 1 << 8;

/// Term id is a name (don't attempt to lookup as entity)
pub const ECS_IS_NAME: u32 = 1 << 9;

/// Prevent observer from triggering on term
pub const ECS_FILTER: u32 = 1 << 10;

/// Union of flags used for traversing (EcsUp|EcsDown|EcsTraverseAll|EcsSelf|EcsCascade|EcsParent)
pub const ECS_TRAVERSE_FLAGS: u32 =
    ECS_UP | ECS_DOWN | ECS_TRAVERSE_ALL | ECS_SELF | ECS_CASCADE | ECS_PARENT;

impl Default for ecs_term_id_t {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: std::ptr::null_mut(),
            trav: Default::default(),
            flags: Default::default(),
        }
    }
}

impl Default for ecs_term_t {
    fn default() -> Self {
        Self {
            id: Default::default(),
            src: Default::default(),
            first: Default::default(),
            second: Default::default(),
            inout: Default::default(),
            oper: Default::default(),
            id_flags: Default::default(),
            name: std::ptr::null_mut(),
            field_index: Default::default(),
            idr: std::ptr::null_mut(),
            flags: Default::default(),
            move_: Default::default(),
        }
    }
}

impl Default for ecs_filter_desc_t {
    fn default() -> Self {
        Self {
            _canary: Default::default(),
            terms: Default::default(),
            terms_buffer: std::ptr::null_mut(),
            terms_buffer_count: Default::default(),
            storage: std::ptr::null_mut(),
            instanced: Default::default(),
            flags: Default::default(),
            expr: std::ptr::null(),
            entity: Default::default(),
        }
    }
}

impl Default for ecs_query_desc_t {
    fn default() -> Self {
        Self {
            _canary: Default::default(),
            filter: Default::default(),
            order_by_component: Default::default(),
            order_by: Default::default(),
            sort_table: Default::default(),
            group_by_id: Default::default(),
            group_by: Default::default(),
            on_group_create: Default::default(),
            on_group_delete: Default::default(),
            group_by_ctx: std::ptr::null_mut(),
            group_by_ctx_free: Default::default(),
            parent: std::ptr::null_mut(),
        }
    }
}

impl Default for ecs_header_t {
    fn default() -> Self {
        Self {
            magic: ecs_filter_t_magic as ::std::os::raw::c_int,
            type_: Default::default(),
            mixins: std::ptr::null_mut(),
        }
    }
}

#[allow(clippy::derivable_impls)]
impl Default for ecs_iterable_t {
    fn default() -> Self {
        Self {
            init: Default::default(),
        }
    }
}

impl Default for ecs_filter_t {
    fn default() -> Self {
        unsafe { ECS_FILTER_INIT }
    }
}

impl Default for ecs_entity_desc_t {
    fn default() -> Self {
        Self {
            _canary: Default::default(),
            id: Default::default(),
            name: std::ptr::null(),
            sep: std::ptr::null(),
            root_sep: std::ptr::null(),
            symbol: std::ptr::null(),
            use_low_id: Default::default(),
            add: Default::default(),
            add_expr: std::ptr::null(),
        }
    }
}

impl Default for ecs_app_desc_t {
    fn default() -> Self {
        Self {
            target_fps: Default::default(),
            delta_time: Default::default(),
            threads: Default::default(),
            frames: Default::default(),
            enable_rest: Default::default(),
            enable_monitor: Default::default(),
            port: Default::default(),
            init: Default::default(),
            ctx: std::ptr::null_mut(),
        }
    }
}
