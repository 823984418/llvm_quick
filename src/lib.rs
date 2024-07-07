use std::marker::PhantomData;
use std::ops::Deref;
use std::ptr::NonNull;

pub use llvm_sys;
use llvm_sys::debuginfo::LLVMMetadataKind;
use llvm_sys::prelude::{LLVMModuleFlagEntry, LLVMValueMetadataEntry};
use llvm_sys::*;

use crate::opaque::{Opaque, PhantomOpaque};
use crate::type_tag::*;

pub mod analysis;
pub mod bit_reader;
pub mod bit_writer;
pub mod blake3;
pub mod comdat;
pub mod core;
pub mod debuginfo;
pub mod disassembler;
pub mod error;
pub mod error_handling;
pub mod execution_engine;
pub mod ir_reader;
pub mod linker;
pub mod lto;
pub mod object;
pub mod opaque;
pub mod orc2;
pub mod owning;
pub mod remark;
pub mod support;
pub mod target;
pub mod target_machine;
pub mod transforms;
pub mod type_tag;

#[repr(transparent)]
pub struct MemoryBuffer {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for MemoryBuffer {
    type Inner = LLVMMemoryBuffer;
}

#[repr(transparent)]
pub struct Context {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Context {
    type Inner = LLVMContext;
}

#[repr(transparent)]
pub struct Module<'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'c Context>,
}

unsafe impl<'c> Opaque for Module<'c> {
    type Inner = LLVMModule;
}

#[repr(transparent)]
pub struct Type<T: TypeTag> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Type<T> {
    type Inner = LLVMType;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe { T::type_cast(Type::from_raw(ptr)) }
    }
}

#[repr(transparent)]
pub struct Value<T: TypeTag> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for Value<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Value::<any>::from_raw(ptr);
            x.get_type().try_cast::<Type<T>>()?;
            Some(x.cast_unchecked())
        }
    }
}

#[repr(transparent)]
pub struct Argument<T: TypeTag> {
    parent: Value<T>,
}

unsafe impl<T: TypeTag> Opaque for Argument<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe { Value::<T>::try_from_raw(ptr)?.is_a_argument() }
    }
}

impl<T: TypeTag> Deref for Argument<T> {
    type Target = Value<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct Constant<T: TypeTag> {
    parent: Value<T>,
}

unsafe impl<T: TypeTag> Opaque for Constant<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Value::<any>::try_from_raw(ptr)?.is_a_constant()?;
            x.get_type().try_cast::<Type<T>>()?;
            Some(x.cast_unchecked())
        }
    }
}

impl<T: TypeTag> Deref for Constant<T> {
    type Target = Value<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct GlobalValue<T: TypeTag> {
    parent: Constant<ptr_any>,
    _marker: PhantomData<fn(T) -> T>,
}

unsafe impl<T: TypeTag> Opaque for GlobalValue<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Value::<any>::try_from_raw(ptr)?.is_a_global_value()?;
            x.get_value_type().try_cast::<Type<T>>()?;
            Some(x.cast_unchecked())
        }
    }
}

impl<T: TypeTag> Deref for GlobalValue<T> {
    type Target = Constant<ptr_any>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct Function<T: FunTypeTag> {
    parent: GlobalValue<T>,
}

unsafe impl<T: FunTypeTag> Opaque for Function<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Value::<fun_any>::try_from_raw(ptr)?.is_a_function()?;
            x.get_value_type().try_cast::<Type<T>>()?;
            Some(x.cast_unchecked())
        }
    }
}

impl<T: FunTypeTag> Deref for Function<T> {
    type Target = GlobalValue<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct GlobalAlias<T: TypeTag> {
    parent: GlobalValue<T>,
}

unsafe impl<T: TypeTag> Opaque for GlobalAlias<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        let x = Value::<any>::try_from_raw(ptr)?.is_a_global_alias()?;
        x.get_value_type().try_cast::<Type<T>>()?;
        Some(x.cast_unchecked())
    }
}

impl<T: TypeTag> Deref for GlobalAlias<T> {
    type Target = GlobalValue<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct Instruction<T: TypeTag> {
    parent: Value<T>,
}

unsafe impl<T: TypeTag> Opaque for Instruction<T> {
    type Inner = LLVMValue;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe { Value::<T>::try_from_raw(ptr)?.is_a_instruction() }
    }
}

impl<T: TypeTag> Deref for Instruction<T> {
    type Target = Value<T>;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct BasicBlock {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for BasicBlock {
    type Inner = LLVMBasicBlock;
}

#[repr(transparent)]
pub struct Metadata {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Metadata {
    type Inner = LLVMOpaqueMetadata;
}

pub trait MetadataKind: Opaque<Inner = LLVMOpaqueMetadata> {
    fn match_kind(kind: &LLVMMetadataKind) -> bool;

    unsafe fn from_check_kind<'r>(ptr: *mut LLVMOpaqueMetadata) -> Option<&'r Self> {
        unsafe {
            let metadata = Metadata::try_from_raw(ptr)?;
            if Self::match_kind(&metadata.get_kind()) {
                Some(metadata.cast_unchecked())
            } else {
                None
            }
        }
    }
}

macro_rules! opaque_metadata {
    (private $name:ident: $parent:ty) => {
        #[repr(transparent)]
        pub struct $name {
            parent: $parent,
        }

        unsafe impl Opaque for $name {
            type Inner = LLVMOpaqueMetadata;

            unsafe fn try_from_raw<'r>(ptr: *mut Self::Inner) -> Option<&'r Self> {
                unsafe { Self::from_check_kind(ptr) }
            }
        }

        impl Deref for $name {
            type Target = $parent;

            fn deref(&self) -> &Self::Target {
                &self.parent
            }
        }
    };

    ($name:ident: $parent:ty = [$($sub:ty),*]) => {
        opaque_metadata!(private $name: $parent);

        impl MetadataKind for $name {
            fn match_kind(kind: &LLVMMetadataKind) -> bool {
                true $(|| <$sub as MetadataKind>::match_kind(kind))*
            }
        }
    };

    ($name:ident: $parent:ty = $kind:ident) => {
        opaque_metadata!(private $name: $parent);

        impl MetadataKind for $name {
            fn match_kind(kind: &LLVMMetadataKind) -> bool {
                matches!(kind, LLVMMetadataKind::$kind)
            }
        }
    };
}

opaque_metadata!(DIArgList: Metadata = LLVMDIArgListMetadataKind);
opaque_metadata!(DistinctMDOperandPlaceholder: Metadata = LLVMDistinctMDOperandPlaceholderMetadataKind);
opaque_metadata!(MDNode: Metadata = [DIAssignID, DIExpression, DIGlobalVariableExpression, DILocation, DIMacroNode, DINode, MDTuple]);
opaque_metadata!(DIAssignID: MDNode = LLVMDIAssignIDMetadataKind);
opaque_metadata!(DIExpression: MDNode = LLVMDIExpressionMetadataKind);
opaque_metadata!(DIGlobalVariableExpression: MDNode = LLVMDIGlobalVariableExpressionMetadataKind);
opaque_metadata!(DILocation: MDNode = LLVMDILocationMetadataKind);
opaque_metadata!(DIMacroNode: MDNode = [DIMacro, DIMacroFile]);
opaque_metadata!(DIMacro: DIMacroNode = LLVMDIMacroMetadataKind);
opaque_metadata!(DIMacroFile: DIMacroNode = LLVMDIMacroFileMetadataKind);
opaque_metadata!(DINode: MDNode = [DIEnumerator, DIGenericSubrange, DIImportedEntity, DILabel, DIObjCProperty, DIScope, DISubrange, DITemplateParameter, DIVariable, GenericDINode]);
opaque_metadata!(DIEnumerator: DINode = LLVMDIEnumeratorMetadataKind);
opaque_metadata!(DIGenericSubrange: DINode = LLVMDIGenericSubrangeMetadataKind);
opaque_metadata!(DIImportedEntity: DINode = LLVMDIImportedEntityMetadataKind);
opaque_metadata!(DILabel: DINode = LLVMDILabelMetadataKind);
opaque_metadata!(DIObjCProperty: DINode = LLVMDIObjCPropertyMetadataKind);
opaque_metadata!(DIScope: DINode = [DICommonBlock, DICompileUnit, DIFile, DILocalScope, DIModule, DINamespace, DIType]);
opaque_metadata!(DICommonBlock: DIScope = LLVMDICommonBlockMetadataKind);
opaque_metadata!(DICompileUnit: DIScope = LLVMDICompileUnitMetadataKind);
opaque_metadata!(DIFile: DIScope = LLVMDIFileMetadataKind);
opaque_metadata!(DILocalScope: DIScope = [DILexicalBlockBase, DISubrange]);
opaque_metadata!(DILexicalBlockBase: DILocalScope = [DILexicalBlock, DILexicalBlockFile]);
opaque_metadata!(DILexicalBlock: DILexicalBlockBase = LLVMDILexicalBlockMetadataKind);
opaque_metadata!(DILexicalBlockFile: DILexicalBlockBase = LLVMDILexicalBlockFileMetadataKind);
opaque_metadata!(DISubprogram: DILocalScope = LLVMDISubprogramMetadataKind);
opaque_metadata!(DIModule: DIScope = LLVMDIModuleMetadataKind);
opaque_metadata!(DINamespace: DIScope = LLVMDINamespaceMetadataKind);
opaque_metadata!(DIType: DIScope = [DICompositeType, DIDerivedType, DIStringType, DISubroutineType]);
opaque_metadata!(DIBasicType: DIType = LLVMDIBasicTypeMetadataKind);
opaque_metadata!(DICompositeType: DIType = LLVMDICompositeTypeMetadataKind);
opaque_metadata!(DIDerivedType: DIType = LLVMDIDerivedTypeMetadataKind);
opaque_metadata!(DIStringType: DIType = LLVMDIStringTypeMetadataKind);
opaque_metadata!(DISubroutineType: DIType = LLVMDISubroutineTypeMetadataKind);
opaque_metadata!(DISubrange: DINode = LLVMDISubrangeMetadataKind);
opaque_metadata!(DITemplateParameter: DINode = [DITemplateTypeParameter, DITemplateValueParameter]);
opaque_metadata!(DITemplateTypeParameter: DITemplateParameter = LLVMDITemplateTypeParameterMetadataKind);
opaque_metadata!(DITemplateValueParameter: DITemplateParameter = LLVMDITemplateValueParameterMetadataKind);
opaque_metadata!(DIVariable: DINode = [DIGlobalVariable, DILocalVariable]);
opaque_metadata!(DIGlobalVariable: DIVariable = LLVMDIGlobalVariableMetadataKind);
opaque_metadata!(DILocalVariable: DIVariable = LLVMDILocalVariableMetadataKind);
opaque_metadata!(GenericDINode: DINode = LLVMGenericDINodeMetadataKind);
opaque_metadata!(MDTuple: MDNode = LLVMMDTupleMetadataKind);
opaque_metadata!(MDString: MDNode = LLVMMDStringMetadataKind);
opaque_metadata!(ValueAsMetadata: Metadata = [ConstantAsMetadata, LocalAsMetadata]);
opaque_metadata!(ConstantAsMetadata: ValueAsMetadata = LLVMConstantAsMetadataMetadataKind);
opaque_metadata!(LocalAsMetadata: ValueAsMetadata = LLVMLocalAsMetadataMetadataKind);

#[repr(transparent)]
pub struct NamedMDNode {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for NamedMDNode {
    type Inner = LLVMOpaqueNamedMDNode;
}

// #[repr(transparent)]
// pub struct ValueMetadataEntry {
//     _opaque: PhantomOpaque,
// }
//
// unsafe impl Opaque for ValueMetadataEntry {
//     type Inner = LLVMOpaqueValueMetadataEntry;
// }

#[repr(transparent)]
pub struct Builder<'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'c Context>,
}

unsafe impl<'c> Opaque for Builder<'c> {
    type Inner = LLVMBuilder;
}

#[repr(transparent)]
pub struct DIBuilder<'m, 'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'m Module<'c>>,
}

unsafe impl<'m, 'c> Opaque for DIBuilder<'m, 'c> {
    type Inner = LLVMOpaqueDIBuilder;
}

#[repr(transparent)]
pub struct ModuleProvider {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for ModuleProvider {
    type Inner = LLVMModuleProvider;
}

#[repr(transparent)]
pub struct PassManager {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for PassManager {
    type Inner = LLVMPassManager;
}

#[repr(transparent)]
pub struct Use {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Use {
    type Inner = LLVMUse;
}

#[repr(transparent)]
pub struct OperandBundle<'c> {
    _opaque: PhantomOpaque,
    _marker: PhantomData<&'c Context>,
}

unsafe impl<'c> Opaque for OperandBundle<'c> {
    type Inner = LLVMOpaqueOperandBundle;
}

#[repr(transparent)]
pub struct DiagnosticInfo {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for DiagnosticInfo {
    type Inner = LLVMDiagnosticInfo;
}

#[repr(transparent)]
pub struct Comdat {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Comdat {
    type Inner = LLVMComdat;
}

// #[repr(transparent)]
// pub struct ModuleFlagEntry {
//     _opaque: PhantomOpaque,
// }
//
// unsafe impl Opaque for ModuleFlagEntry {
//     type Inner = LLVMOpaqueModuleFlagEntry;
// }

#[repr(transparent)]
pub struct JITEventListener {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for JITEventListener {
    type Inner = LLVMOpaqueJITEventListener;
}

#[repr(transparent)]
pub struct Attribute {
    _opaque: PhantomOpaque,
}

unsafe impl Opaque for Attribute {
    type Inner = LLVMOpaqueAttributeRef;
}

#[repr(transparent)]
pub struct EnumAttribute {
    parent: Attribute,
}

unsafe impl Opaque for EnumAttribute {
    type Inner = LLVMOpaqueAttributeRef;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Attribute::try_from_raw(ptr)?;
            if x.is_enum_attribute() {
                Some(x.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl Deref for EnumAttribute {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct TypeAttribute {
    parent: Attribute,
}

unsafe impl Opaque for TypeAttribute {
    type Inner = LLVMOpaqueAttributeRef;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Attribute::try_from_raw(ptr)?;
            if x.is_type_attribute() {
                Some(x.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl Deref for TypeAttribute {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

#[repr(transparent)]
pub struct StringAttribute {
    parent: Attribute,
}

unsafe impl Opaque for StringAttribute {
    type Inner = LLVMOpaqueAttributeRef;

    unsafe fn try_from_raw<'a>(ptr: *mut Self::Inner) -> Option<&'a Self> {
        unsafe {
            let x = Attribute::try_from_raw(ptr)?;
            if x.is_string_attribute() {
                Some(x.cast_unchecked())
            } else {
                None
            }
        }
    }
}

impl Deref for StringAttribute {
    type Target = Attribute;

    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

pub struct ValueMetadataEntries<'m> {
    ptr: NonNull<LLVMValueMetadataEntry>,
    len: usize,
    marker: PhantomData<&'m Metadata>,
}

impl<'m> ValueMetadataEntries<'m> {
    pub unsafe fn from_raw(ptr: *mut LLVMValueMetadataEntry, len: usize) -> Self {
        unsafe {
            Self {
                ptr: NonNull::new(ptr).unwrap_unchecked(),
                len,
                marker: PhantomData,
            }
        }
    }

    pub fn as_raw(&self) -> *mut LLVMValueMetadataEntry {
        self.ptr.as_ptr()
    }
}

pub struct ModuleFlagsMetadata<'m> {
    ptr: NonNull<LLVMModuleFlagEntry>,
    len: usize,
    marker: PhantomData<&'m Metadata>,
}

impl<'m> ModuleFlagsMetadata<'m> {
    pub unsafe fn from_raw(ptr: *mut LLVMModuleFlagEntry, len: usize) -> Self {
        unsafe {
            Self {
                ptr: NonNull::new(ptr).unwrap_unchecked(),
                len,
                marker: PhantomData,
            }
        }
    }

    pub fn as_raw(&self) -> *mut LLVMModuleFlagEntry {
        self.ptr.as_ptr()
    }
}
