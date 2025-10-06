/// Top-level macro that wraps split view and event handler declarations
#[macro_export]
macro_rules! tauri_splitview {
    // Pattern for split view class definition
    ($splitview_name:ident {
        $(
            config: {
                $($method_name:ident: $method_value:expr),* $(,)?
            }
        )?
        $(
            with: {
                $(tracking_area: {
                    $($tracking_key:ident: $tracking_value:expr),* $(,)?
                })?
            }
        )?
    }) => {
        #[allow(unused_imports)]
        use $crate::objc2::{define_class, msg_send, MainThreadOnly, Message, DefinedClass, rc::{Retained, Allocated}, ClassType, runtime::ProtocolObject};
        #[allow(unused_imports)]
        use $crate::objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker};
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::{NSWindowDelegate, NSSplitViewDelegate};
        #[allow(unused_imports)]
        use $crate::{NSNotification, NSWindow, NSView, NSSplitView, NSPoint, NSRect, NSSize, AnyObject};
        #[allow(unused_imports)]
        use $crate::objc2::runtime::Bool;
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::NSEvent;

        $crate::splitview!($splitview_name {
            $(
                config: {
                    $($method_name: $method_value),*
                }
            )?
            $(
                with: {
                    $(tracking_area: {
                        $($tracking_key: $tracking_value),*
                    })?
                }
            )?
        });
    };

    // Pattern for event handler declarations
    (
        $(
            splitview_event!($handler_name:ident {
                $(
                    $method:ident ( $first_param:ident : $first_type:ty $(, $param:ident : $param_type:ty)* $(,)? ) -> $return_type:ty
                ),* $(,)?
            })
        )*
    ) => {
        #[allow(unused_imports)]
        use $crate::objc2::{define_class, msg_send, MainThreadOnly, Message, DefinedClass, rc::{Retained, Allocated}, ClassType, runtime::ProtocolObject};
        #[allow(unused_imports)]
        use $crate::objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker};
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::{NSWindowDelegate, NSSplitViewDelegate};
        #[allow(unused_imports)]
        use $crate::{NSNotification, NSWindow, NSView, NSSplitView, NSPoint, NSRect, NSSize, AnyObject};
        #[allow(unused_imports)]
        use $crate::objc2::runtime::Bool;
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::NSEvent;

        $(
            $crate::splitview_event!($handler_name {
                $(
                    $method ( $first_param : $first_type $(, $param : $param_type)* ) -> $return_type
                ),*
            });
        )*
    };

    // Pattern for mixed split view and event handler declarations
    (
        $(
            splitview!($splitview_name:ident {
                $(
                    config: {
                        $($method_name:ident: $method_value:expr),* $(,)?
                    }
                )?
                $(
                    with: {
                        $(tracking_area: {
                            $($tracking_key:ident: $tracking_value:expr),* $(,)?
                        })?
                    }
                )?
            })
        )*
        $(
            splitview_event!($handler_name:ident {
                $(
                    $event_method:ident ( $first_param:ident : $first_type:ty $(, $param:ident : $param_type:ty)* $(,)? ) -> $return_type:ty
                ),* $(,)?
            })
        )*
    ) => {
        #[allow(unused_imports)]
        use $crate::objc2::{define_class, msg_send, MainThreadOnly, Message, DefinedClass, rc::{Retained, Allocated}, ClassType, runtime::ProtocolObject};
        #[allow(unused_imports)]
        use $crate::objc2_foundation::{NSObject, NSObjectProtocol, MainThreadMarker};
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::{NSWindowDelegate, NSSplitViewDelegate};
        #[allow(unused_imports)]
        use $crate::{NSNotification, NSWindow, NSView, NSSplitView, NSPoint, NSRect, NSSize, AnyObject};
        #[allow(unused_imports)]
        use $crate::objc2::runtime::Bool;
        #[allow(unused_imports)]
        use $crate::objc2_app_kit::NSEvent;

        $(
            $crate::splitview!($splitview_name {
                $(
                    config: {
                        $($method_name: $method_value),*
                    }
                )?
                $(
                    with: {
                        $(tracking_area: {
                            $($tracking_key: $tracking_value),*
                        })?
                    }
                )?
            });
        )*

        $(
            $crate::splitview_event!($handler_name {
                $(
                    $event_method ( $first_param : $first_type $(, $param : $param_type)* ) -> $return_type
                ),*
            });
        )*
    };
}
