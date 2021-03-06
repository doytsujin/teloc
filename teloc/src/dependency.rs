use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

/// Trait is used to working with `Resolver` trait. If you want that your service can be resolved by
/// `Resolver`, you may implement this trait for your service. There are three ways:
/// 1. Implement it by yourself. Not recommended for business code, recommended for library code.
/// 2. Create a constructor and add `#[inject]` macro. Recommended for business code.
/// 3. Derive `Teloc` macro, when all of your fields of structs implement `Dependency`.
pub trait Dependency<Deps> {
    fn init(deps: Deps) -> Self;
}

impl<Deps, D> Dependency<Deps> for Rc<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        Rc::new(D::init(deps))
    }
}
impl<Deps, D> Dependency<Deps> for RefCell<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        RefCell::new(D::init(deps))
    }
}
impl<Deps, D> Dependency<Deps> for Box<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        Box::new(D::init(deps))
    }
}
impl<Deps, D> Dependency<Deps> for Arc<D>
where
    D: Dependency<Deps>,
{
    fn init(deps: Deps) -> Self {
        Arc::new(D::init(deps))
    }
}

/// Trait is used to resolve services by cloning. It must be implement only for wrappers that
/// guarantees that there are only one instance and many references, like `Rc`, `Arc` structs and
/// immutable reference. For comfort implement for primitive number types.
pub trait DependencyClone: Clone {}

impl<D> DependencyClone for Rc<D> {}

impl<D> DependencyClone for Arc<D> {}

impl<D> DependencyClone for &D {}

macro_rules! impl_dpc {
    {$($x:ty),*} => {
        $(
            impl DependencyClone for $x {}
        )*
    }
}

impl_dpc! {
    bool, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128
}
