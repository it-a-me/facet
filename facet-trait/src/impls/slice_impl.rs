use crate::*;
use core::alloc::Layout;

unsafe impl<T> Facet for &[T]
where
    T: Facet,
{
    const ARCHETYPE: Self = &[];
    const SHAPE: &'static Shape = &const {
        Shape::builder()
            .layout(Layout::new::<&[T]>())
            .def(Def::List(
                ListDef::builder()
                    .vtable(
                        &const {
                            ListVTable::builder()
                        .init_in_place_with_capacity(|_, _| Err(()))
                        .push(|_, _| {
                            panic!("Cannot push to &[T]");
                        })
                        .len(|ptr| unsafe {
                            let slice = ptr.as_ref::<&[T]>();
                            slice.len()
                        })
                        .get_item_ptr(|ptr, index| unsafe {
                            let slice = ptr.as_ref::<&[T]>();
                            let len = slice.len();
                            if index >= len {
                                panic!(
                                    "Index out of bounds: the len is {len} but the index is {index}"
                                );
                            }
                            OpaqueConst::new_unchecked(slice.as_ptr().add(index))
                        })
                        .build()
                        },
                    )
                    .t(T::SHAPE)
                    .build(),
            ))
            .vtable(
                &const {
                    let mut builder = ValueVTable::builder()
                        .type_name(|f, opts| {
                            if let Some(opts) = opts.for_children() {
                                write!(f, "&[")?;
                                (T::SHAPE.vtable.type_name)(f, opts)?;
                                write!(f, "]")
                            } else {
                                write!(f, "&[⋯]")
                            }
                        })
                        .marker_traits(T::SHAPE.vtable.marker_traits)
                        .default_in_place(|ptr| unsafe { ptr.write(&[] as &[T]) })
                        .clone_into(|src, dst| unsafe {
                            // This works because we're cloning a shared reference (&[T]), not the actual slice data.
                            // We're just copying the fat pointer (ptr + length) that makes up the slice reference.
                            dst.write(src.as_ref::<&[T]>())
                        });

                    if T::SHAPE.vtable.debug.is_some() {
                        builder = builder.debug(|value, f| {
                            let value = unsafe { value.as_ref::<&[T]>() };
                            write!(f, "[")?;
                            for (i, item) in value.iter().enumerate() {
                                if i > 0 {
                                    write!(f, ", ")?;
                                }
                                unsafe {
                                    (T::SHAPE.vtable.debug.unwrap_unchecked())(
                                        OpaqueConst::from_ref(item),
                                        f,
                                    )?;
                                }
                            }
                            write!(f, "]")
                        });
                    }

                    if T::SHAPE.vtable.eq.is_some() {
                        builder = builder.eq(|a, b| {
                            let a = unsafe { a.as_ref::<&[T]>() };
                            let b = unsafe { b.as_ref::<&[T]>() };
                            if a.len() != b.len() {
                                return false;
                            }
                            for (x, y) in a.iter().zip(b.iter()) {
                                if !unsafe {
                                    (T::SHAPE.vtable.eq.unwrap_unchecked())(
                                        OpaqueConst::from_ref(x),
                                        OpaqueConst::from_ref(y),
                                    )
                                } {
                                    return false;
                                }
                            }
                            true
                        });
                    }

                    if T::SHAPE.vtable.ord.is_some() {
                        builder = builder.ord(|a, b| {
                            let a = unsafe { a.as_ref::<&[T]>() };
                            let b = unsafe { b.as_ref::<&[T]>() };
                            for (x, y) in a.iter().zip(b.iter()) {
                                let ord = unsafe {
                                    (T::SHAPE.vtable.ord.unwrap_unchecked())(
                                        OpaqueConst::from_ref(x),
                                        OpaqueConst::from_ref(y),
                                    )
                                };
                                if ord != core::cmp::Ordering::Equal {
                                    return ord;
                                }
                            }
                            a.len().cmp(&b.len())
                        });
                    }

                    if T::SHAPE.vtable.partial_ord.is_some() {
                        builder = builder.partial_ord(|a, b| {
                            let a = unsafe { a.as_ref::<&[T]>() };
                            let b = unsafe { b.as_ref::<&[T]>() };
                            for (x, y) in a.iter().zip(b.iter()) {
                                let ord = unsafe {
                                    (T::SHAPE.vtable.partial_ord.unwrap_unchecked())(
                                        OpaqueConst::from_ref(x),
                                        OpaqueConst::from_ref(y),
                                    )
                                };
                                match ord {
                                    Some(core::cmp::Ordering::Equal) => continue,
                                    Some(order) => return Some(order),
                                    None => return None,
                                }
                            }
                            a.len().partial_cmp(&b.len())
                        });
                    }

                    if T::SHAPE.vtable.hash.is_some() {
                        builder = builder.hash(|value, state, hasher| {
                            let value = unsafe { value.as_ref::<&[T]>() };
                            for item in value.iter() {
                                unsafe {
                                    (T::SHAPE.vtable.hash.unwrap_unchecked())(
                                        OpaqueConst::from_ref(item),
                                        state,
                                        hasher,
                                    )
                                };
                            }
                        });
                    }

                    builder.build()
                },
            )
            .build()
    };
}
