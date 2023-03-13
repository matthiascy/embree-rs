use crate::{sys::*, Geometry};

/// Trait for extended intersection context enabling passing of additional
/// ray-query specific data.
///
/// # Safety
///
/// Structs that implement this trait must guarantee that they are
/// layout-compatible with [`IntersectContext`] (i.e. pointer casts between the
/// two types are valid). The corresponding pattern in C is called poor man's
/// inheritance. See [`IntersectContextExt`] for an example of how to do this
pub unsafe trait AsIntersectContext {
    type Ext;

    fn as_context(&self) -> &IntersectContext;
    fn as_mut_context(&mut self) -> &mut IntersectContext;

    fn as_context_ptr(&self) -> *const IntersectContext {
        self.as_context() as *const IntersectContext
    }

    fn as_mut_context_ptr(&mut self) -> *mut IntersectContext {
        self.as_mut_context() as *mut IntersectContext
    }

    fn as_extended(&self) -> Option<&Self::Ext>;
    fn as_mut_extended(&mut self) -> Option<&mut Self::Ext>;
}

/// Per ray-query intersection context.
///
/// This is used to configure intersection flags, specify a filter callback
/// function, and specify the chain of IDs of the current instance, and to
/// attach arbitrary user data to the query (e.g. per ray data).
///
/// # Context Filter Callback
///
/// A filter function can be specified inside the context. This function is
/// invoked as a second filter stage after the per-geometry intersect
/// [`Geometry::set_intersect_filter_function`] or occluded filter
/// [`Geometry::set_occluded_filter_function`] function is invoked. Only rays
/// that passed the first filter stage are valid in this second filter stage.
/// Having such a per ray-query filter function can be useful to implement
/// modifications of the behavior of the query, such as collecting all hits or
/// accumulating transparencies.
///
/// It is guaranteed that the intersection context passed to a ray query is
/// directly passed to the registered callback function. This means that it's
/// possible to attach arbitrary user data to the context and access it from the
/// callback (see [`IntersectContextExt`]). On the contrary, the ray is not
/// guaranteed to be passed to the callback functions, thus reading additional
/// data from the ray passed to the callback is not possible.
///
/// ## Note
///
/// The support for the context filter function must be enabled for a scene by
/// using the [`SceneFlags::CONTEXT_FILTER_FUNCTION`](`crate::SceneFlags::CONTEXT_FILTER_FUNCTION`) flag.
///
/// In case of instancing this feature has to get enabled also for each
/// instantiated scene.
///
/// # Hints
///
/// Best primary ray performance can be obtained by using the ray stream API
/// and setting the intersect context flag to
/// [`IntersectContextFlags::COHERENT`](`crate::IntersectContextFlags`). For
/// secondary rays, it is typically better to use the
/// [`IntersectContextFlags::INCOHERENT`](`crate::IntersectContextFlags::INCOHERENT`),
/// unless the rays are known to be coherent(e.g. for primary transparency
/// rays).
pub type IntersectContext = RTCIntersectContext;

impl IntersectContext {
    /// Shortcut to create a IntersectContext with coherent flag set.
    pub fn coherent() -> IntersectContext {
        IntersectContext::new(RTCIntersectContextFlags::COHERENT)
    }

    /// Shortcut to create a IntersectContext with incoherent flag set.
    pub fn incoherent() -> IntersectContext {
        IntersectContext::new(RTCIntersectContextFlags::INCOHERENT)
    }

    pub fn new(flags: RTCIntersectContextFlags) -> IntersectContext {
        RTCIntersectContext {
            flags,
            filter: None,
            instID: [u32::MAX; 1],
        }
    }
}

unsafe impl AsIntersectContext for IntersectContext {
    type Ext = ();

    fn as_context(&self) -> &IntersectContext { self }

    fn as_mut_context(&mut self) -> &mut IntersectContext { self }

    fn as_extended(&self) -> Option<&Self::Ext> { None }

    fn as_mut_extended(&mut self) -> Option<&mut Self::Ext> { None }
}

/// Extended intersection context with additional ray query specific data.
///
/// As Embree 3 does not support placing additional data at the end of the ray
/// structure, and accessing that data inside user geometry callbacks and filter
/// callback functions, we have to attach the data to the ray query context.
#[repr(C)]
#[derive(Debug)]
pub struct IntersectContextExt<E>
where
    E: Sized,
{
    pub ctx: IntersectContext,
    pub ext: E,
}

impl<E> Clone for IntersectContextExt<E>
where
    E: Sized + Clone,
{
    fn clone(&self) -> Self {
        IntersectContextExt {
            ctx: self.ctx,
            ext: self.ext.clone(),
        }
    }
}

impl<E> Copy for IntersectContextExt<E> where E: Sized + Copy {}

unsafe impl<E> AsIntersectContext for IntersectContextExt<E>
where
    E: Sized,
{
    type Ext = E;

    fn as_context(&self) -> &IntersectContext { &self.ctx }

    fn as_mut_context(&mut self) -> &mut IntersectContext { &mut self.ctx }

    fn as_extended(&self) -> Option<&Self::Ext> { Some(&self.ext) }

    fn as_mut_extended(&mut self) -> Option<&mut Self::Ext> { Some(&mut self.ext) }
}

impl<E> IntersectContextExt<E>
where
    E: Sized,
{
    pub fn new(flags: RTCIntersectContextFlags, extra: E) -> IntersectContextExt<E> {
        IntersectContextExt {
            ctx: IntersectContext::new(flags),
            ext: extra,
        }
    }

    pub fn coherent(extra: E) -> IntersectContextExt<E> {
        IntersectContextExt {
            ctx: IntersectContext::coherent(),
            ext: extra,
        }
    }

    pub fn incoherent(extra: E) -> IntersectContextExt<E> {
        IntersectContextExt {
            ctx: IntersectContext::incoherent(),
            ext: extra,
        }
    }
}

/// A stack which stores the IDs and instance transformations during a BVH
/// traversal for a point query.
///
/// The transformations are assumed to be affine transformations
/// (3×3 matrix plus translation) and therefore the last column is ignored.
pub type PointQueryContext = RTCPointQueryContext;

// TODO: PointQueryContext::new
