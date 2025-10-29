///interface [EntrypointContainer](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/entrypoint/EntrypointContainer.html)
pub enum EntrypointContainer {}
unsafe impl ::java_spaghetti::ReferenceType for EntrypointContainer {}
unsafe impl ::java_spaghetti::JniType for EntrypointContainer {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/entrypoint/EntrypointContainer")
    }
}
impl EntrypointContainer {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/entrypoint/EntrypointContainer"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[getProvider](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/entrypoint/EntrypointContainer.html#getProvider())
    pub fn getProvider<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        ::std::option::Option<::java_spaghetti::Local<'env, super::ModContainer>>,
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"getProvider",
                        c"()Lnet/fabricmc/loader/api/ModContainer;",
                    ))
                })
                .as_raw();
            __jni_env.call_object_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
///interface [PreLaunchEntrypoint](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint.html)
pub enum PreLaunchEntrypoint {}
unsafe impl ::java_spaghetti::ReferenceType for PreLaunchEntrypoint {}
unsafe impl ::java_spaghetti::JniType for PreLaunchEntrypoint {
    fn static_with_jni_type<R>(callback: impl FnOnce(&::std::ffi::CStr) -> R) -> R {
        callback(c"net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint")
    }
}
impl PreLaunchEntrypoint {
    fn __class_global_ref(__jni_env: ::java_spaghetti::Env) -> ::java_spaghetti::sys::jobject {
        static __CLASS: ::std::sync::OnceLock<
            ::java_spaghetti::Global<super::super::super::super::super::java::lang::Object>,
        > = ::std::sync::OnceLock::new();
        __CLASS
            .get_or_init(|| unsafe {
                ::java_spaghetti::Local::from_raw(
                    __jni_env,
                    __jni_env
                        .require_class(c"net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint"),
                )
                .as_global()
            })
            .as_raw()
    }
    ///[onPreLaunch](https://maven.fabricmc.net/docs/fabric-loader-0.17.3/net/fabricmc/net/fabricmc/loader/api/entrypoint/PreLaunchEntrypoint.html#onPreLaunch())
    pub fn onPreLaunch<'env>(
        self: &::java_spaghetti::Ref<'env, Self>,
    ) -> ::std::result::Result<
        (),
        ::java_spaghetti::Local<'env, super::super::super::super::super::java::lang::Throwable>,
    > {
        static __METHOD: ::std::sync::OnceLock<::java_spaghetti::JMethodID> =
            ::std::sync::OnceLock::new();
        unsafe {
            let __jni_args = [];
            let __jni_env = self.env();
            let __jni_class = Self::__class_global_ref(__jni_env);
            let __jni_method = __METHOD
                .get_or_init(|| {
                    ::java_spaghetti::JMethodID::from_raw(__jni_env.require_method(
                        __jni_class,
                        c"onPreLaunch",
                        c"()V",
                    ))
                })
                .as_raw();
            __jni_env.call_void_method_a(self.as_raw(), __jni_method, __jni_args.as_ptr())
        }
    }
}
